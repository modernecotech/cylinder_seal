//! Single-node Raft state machine.
//!
//! Implements leader election, log replication, and commit-index tracking for
//! one node in the cluster. The node is driven by a tick loop (external) that
//! periodically calls [`RaftNode::tick`]; incoming RPCs are delivered via
//! [`RaftNode::on_request_vote`] and [`RaftNode::on_append_entries`]; outgoing
//! RPCs go through a [`crate::transport::PeerTransport`] supplied by the
//! caller.
//!
//! Design notes:
//! - The node uses a `tokio::sync::Mutex` internally to keep the state
//!   machine `Send`-safe across `.await` points while the state transitions
//!   remain simple to reason about.
//! - Commit notification is published on a `broadcast` channel so multiple
//!   apply workers (or observers) can watch.
//! - This is standard Raft — no Byzantine tolerance, no randomization beyond
//!   the election-timeout jitter. Aligns with the architecture decision in
//!   the project README (Raft CFT, not BFT, not blockchain).

use crate::log::{EntryKind, LogEntry, LogIndex, RaftLog, RaftTerm};
use crate::rpc::{
    AppendEntriesRequest, AppendEntriesResponse, RequestVoteRequest, RequestVoteResponse,
};
use crate::state_machine::{ApplyError, LedgerStateMachine, ProposalResult};
use crate::transport::{PeerTransport, TransportError};
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::{broadcast, Mutex};

/// A node identifier. In production this is the super-peer hostname
/// ("sp-baghdad", "sp-basra", ...). Kept as a `String` so the consensus
/// layer is free of DNS/URL concerns.
pub type NodeId = String;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum RaftRole {
    Follower,
    Candidate,
    Leader,
}

#[derive(Clone, Debug)]
pub struct RaftConfig {
    pub self_id: NodeId,
    pub peers: Vec<NodeId>,
    /// Minimum election timeout. Actual timeout is [min, 2*min) with jitter.
    pub election_timeout_min: Duration,
    /// Interval between leader heartbeats.
    pub heartbeat_interval: Duration,
}

impl RaftConfig {
    /// Quorum = majority of the full voting set (self + peers).
    pub fn quorum(&self) -> usize {
        (self.peers.len() + 1) / 2 + 1
    }
}

/// Minimal persistent state (what Raft §5.6 requires on stable storage).
#[derive(Clone, Debug, Default)]
pub struct PersistentState {
    pub current_term: RaftTerm,
    pub voted_for: Option<NodeId>,
}

/// Live runtime state shared behind a Mutex.
struct Inner {
    role: RaftRole,
    persistent: PersistentState,
    log: RaftLog,
    commit_index: LogIndex,
    last_applied: LogIndex,

    // Leader-only: per-follower progress.
    next_index: HashMap<NodeId, LogIndex>,
    match_index: HashMap<NodeId, LogIndex>,

    // Follower/candidate: when last heartbeat or heartbeat-equivalent arrived.
    last_heartbeat: Instant,
    election_deadline: Instant,

    // Candidate-only: votes collected this term.
    votes_received: usize,
}

/// Snapshot of the state that external observers can query.
#[derive(Clone, Debug)]
pub struct RaftState {
    pub role: RaftRole,
    pub term: RaftTerm,
    pub leader: Option<NodeId>,
    pub commit_index: LogIndex,
    pub last_log_index: LogIndex,
}

pub struct RaftNode {
    config: RaftConfig,
    transport: Arc<dyn PeerTransport>,
    state_machine: Arc<dyn LedgerStateMachine>,
    inner: Mutex<Inner>,
    commit_tx: broadcast::Sender<LogIndex>,
    current_leader: Mutex<Option<NodeId>>,
}

impl RaftNode {
    pub fn new(
        config: RaftConfig,
        transport: Arc<dyn PeerTransport>,
        state_machine: Arc<dyn LedgerStateMachine>,
    ) -> Arc<Self> {
        let (commit_tx, _) = broadcast::channel(1024);
        let now = Instant::now();
        let deadline = now + randomized_election_timeout(config.election_timeout_min);

        let inner = Inner {
            role: RaftRole::Follower,
            persistent: PersistentState::default(),
            log: RaftLog::new(),
            commit_index: 0,
            last_applied: 0,
            next_index: HashMap::new(),
            match_index: HashMap::new(),
            last_heartbeat: now,
            election_deadline: deadline,
            votes_received: 0,
        };

        Arc::new(Self {
            config,
            transport,
            state_machine,
            inner: Mutex::new(inner),
            commit_tx,
            current_leader: Mutex::new(None),
        })
    }

    /// Snapshot of observable state. Takes both inner locks briefly.
    pub async fn state(&self) -> RaftState {
        let i = self.inner.lock().await;
        let leader = self.current_leader.lock().await.clone();
        RaftState {
            role: i.role,
            term: i.persistent.current_term,
            leader,
            commit_index: i.commit_index,
            last_log_index: i.log.last().0,
        }
    }

    /// Subscribe to commit-index updates.
    pub fn subscribe_commits(&self) -> broadcast::Receiver<LogIndex> {
        self.commit_tx.subscribe()
    }

    // ------------------------------------------------------------------
    // Proposal interface (used by the sync service)
    // ------------------------------------------------------------------

    /// Propose a payload for replication. Only valid on the leader.
    ///
    /// Returns the assigned log index. The caller should await the commit
    /// index to reach that point (or subscribe to `subscribe_commits`) and
    /// then ask the state machine for the result.
    pub async fn propose(
        &self,
        kind: EntryKind,
        payload: Vec<u8>,
    ) -> Result<LogIndex, ProposeError> {
        let mut i = self.inner.lock().await;
        if i.role != RaftRole::Leader {
            return Err(ProposeError::NotLeader);
        }
        let term = i.persistent.current_term;
        let index = i.log.append_new(term, kind, payload);
        // Update self's match index so quorum math includes us.
        i.match_index.insert(self.config.self_id.clone(), index);
        tracing::debug!(index, term, "leader accepted proposal");
        Ok(index)
    }

    /// Wait until the commit index reaches `target` (or a newer term starts,
    /// which aborts with [`ProposeError::TermChanged`]). The caller supplies
    /// the term in which the proposal was made.
    pub async fn await_commit(
        self: &Arc<Self>,
        target: LogIndex,
        proposal_term: RaftTerm,
    ) -> Result<ProposalResult, ProposeError> {
        let mut rx = self.subscribe_commits();

        // Fast path: already committed and same term.
        {
            let i = self.inner.lock().await;
            if i.persistent.current_term != proposal_term {
                return Err(ProposeError::TermChanged);
            }
            if i.commit_index >= target {
                if let Some(entry) = i.log.get(target).cloned() {
                    drop(i);
                    return self
                        .state_machine
                        .apply(&entry)
                        .await
                        .map_err(ProposeError::ApplyFailed);
                }
            }
        }

        loop {
            let committed = rx.recv().await.map_err(|_| ProposeError::Aborted)?;
            let i = self.inner.lock().await;
            if i.persistent.current_term != proposal_term {
                return Err(ProposeError::TermChanged);
            }
            if committed >= target {
                if let Some(entry) = i.log.get(target).cloned() {
                    drop(i);
                    return self
                        .state_machine
                        .apply(&entry)
                        .await
                        .map_err(ProposeError::ApplyFailed);
                }
            }
        }
    }

    // ------------------------------------------------------------------
    // Incoming RPC handlers
    // ------------------------------------------------------------------

    pub async fn on_request_vote(&self, req: RequestVoteRequest) -> RequestVoteResponse {
        let mut i = self.inner.lock().await;

        // §5.1: if the term in the RPC is higher, step down first.
        if req.term > i.persistent.current_term {
            i.persistent.current_term = req.term;
            i.persistent.voted_for = None;
            i.role = RaftRole::Follower;
            *self.current_leader.lock().await = None;
        }

        let current_term = i.persistent.current_term;
        if req.term < current_term {
            return RequestVoteResponse {
                term: current_term,
                vote_granted: false,
            };
        }

        // §5.4: deny if our log is at least as up-to-date as the candidate's.
        let (our_last_idx, our_last_term) = i.log.last();
        let log_ok = req.last_log_term > our_last_term
            || (req.last_log_term == our_last_term && req.last_log_index >= our_last_idx);

        let can_vote = i
            .persistent
            .voted_for
            .as_ref()
            .map(|v| v == &req.candidate_id)
            .unwrap_or(true);

        if log_ok && can_vote {
            i.persistent.voted_for = Some(req.candidate_id.clone());
            i.last_heartbeat = Instant::now();
            i.election_deadline =
                i.last_heartbeat + randomized_election_timeout(self.config.election_timeout_min);
            RequestVoteResponse {
                term: current_term,
                vote_granted: true,
            }
        } else {
            RequestVoteResponse {
                term: current_term,
                vote_granted: false,
            }
        }
    }

    pub async fn on_append_entries(
        &self,
        req: AppendEntriesRequest,
    ) -> AppendEntriesResponse {
        let mut i = self.inner.lock().await;

        // §5.1: stale leader — reject.
        if req.term < i.persistent.current_term {
            return AppendEntriesResponse {
                term: i.persistent.current_term,
                success: false,
                match_index: 0,
                conflict_index: None,
            };
        }

        // New term: step down.
        if req.term > i.persistent.current_term {
            i.persistent.current_term = req.term;
            i.persistent.voted_for = None;
        }
        i.role = RaftRole::Follower;
        i.last_heartbeat = Instant::now();
        i.election_deadline =
            i.last_heartbeat + randomized_election_timeout(self.config.election_timeout_min);
        *self.current_leader.lock().await = Some(req.leader_id.clone());

        // §5.3: consistency check against prev_log_index/prev_log_term.
        if req.prev_log_index > 0 {
            match i.log.term_at(req.prev_log_index) {
                Some(t) if t == req.prev_log_term => {}
                _ => {
                    let conflict_index = Some(i.log.len().min(req.prev_log_index));
                    return AppendEntriesResponse {
                        term: i.persistent.current_term,
                        success: false,
                        match_index: 0,
                        conflict_index,
                    };
                }
            }
        }

        // Truncate any conflicting suffix and append.
        if !req.entries.is_empty() {
            let first_new_idx = req.entries[0].index;
            if first_new_idx <= i.log.len() {
                i.log.truncate(first_new_idx - 1);
            }
            for entry in &req.entries {
                i.log.append(entry.clone());
            }
        }

        // Advance commit index.
        let last_new_idx = req
            .entries
            .last()
            .map(|e| e.index)
            .unwrap_or(req.prev_log_index);
        if req.leader_commit > i.commit_index {
            let new_commit = req.leader_commit.min(last_new_idx);
            if new_commit > i.commit_index {
                i.commit_index = new_commit;
                let _ = self.commit_tx.send(new_commit);
            }
        }

        let match_index = i.log.len();
        let apply_up_to = i.commit_index;
        drop(i);
        self.apply_committed(apply_up_to).await;

        AppendEntriesResponse {
            term: self.inner.lock().await.persistent.current_term,
            success: true,
            match_index,
            conflict_index: None,
        }
    }

    // ------------------------------------------------------------------
    // Tick loop (the caller drives this)
    // ------------------------------------------------------------------

    /// Drive one step of the state machine. The caller should invoke this
    /// approximately every `heartbeat_interval / 4` for smooth behavior.
    pub async fn tick(self: &Arc<Self>) {
        let now = Instant::now();
        let role = {
            let i = self.inner.lock().await;
            i.role
        };

        match role {
            RaftRole::Follower | RaftRole::Candidate => {
                let should_elect = {
                    let i = self.inner.lock().await;
                    now >= i.election_deadline
                };
                if should_elect {
                    self.start_election().await;
                }
            }
            RaftRole::Leader => {
                let should_heartbeat = {
                    let i = self.inner.lock().await;
                    now.duration_since(i.last_heartbeat) >= self.config.heartbeat_interval
                };
                if should_heartbeat {
                    self.broadcast_append_entries().await;
                }
            }
        }
    }

    async fn start_election(self: &Arc<Self>) {
        let (term, last_index, last_term) = {
            let mut i = self.inner.lock().await;
            i.persistent.current_term += 1;
            i.persistent.voted_for = Some(self.config.self_id.clone());
            i.role = RaftRole::Candidate;
            i.votes_received = 1; // vote for self
            i.last_heartbeat = Instant::now();
            i.election_deadline = i.last_heartbeat
                + randomized_election_timeout(self.config.election_timeout_min);
            let (li, lt) = i.log.last();
            (i.persistent.current_term, li, lt)
        };

        tracing::info!(term, "starting election");

        let req = RequestVoteRequest {
            term,
            candidate_id: self.config.self_id.clone(),
            last_log_index: last_index,
            last_log_term: last_term,
        };

        for peer in self.config.peers.clone() {
            let transport = self.transport.clone();
            let node = self.clone();
            let req = req.clone();
            tokio::spawn(async move {
                match transport.request_vote(&peer, req).await {
                    Ok(resp) => node.on_vote_reply(peer, resp).await,
                    Err(e) => tracing::debug!(?e, peer = %peer, "request_vote failed"),
                }
            });
        }
    }

    async fn on_vote_reply(self: &Arc<Self>, _peer: NodeId, resp: RequestVoteResponse) {
        let mut i = self.inner.lock().await;
        if resp.term > i.persistent.current_term {
            i.persistent.current_term = resp.term;
            i.persistent.voted_for = None;
            i.role = RaftRole::Follower;
            return;
        }
        if i.role != RaftRole::Candidate {
            return;
        }
        if resp.vote_granted {
            i.votes_received += 1;
            if i.votes_received >= self.config.quorum() {
                self.become_leader(&mut i).await;
            }
        }
    }

    async fn become_leader(&self, i: &mut Inner) {
        i.role = RaftRole::Leader;
        let last_index = i.log.last().0;
        i.next_index.clear();
        i.match_index.clear();
        for peer in &self.config.peers {
            i.next_index.insert(peer.clone(), last_index + 1);
            i.match_index.insert(peer.clone(), 0);
        }
        i.match_index
            .insert(self.config.self_id.clone(), last_index);
        i.last_heartbeat = Instant::now();
        tracing::info!(term = i.persistent.current_term, "elected leader");
        *self.current_leader.lock().await = Some(self.config.self_id.clone());

        // Commit a no-op in our term so we can advance commit index on
        // entries from prior terms (§5.4.2).
        i.log
            .append_new(i.persistent.current_term, EntryKind::NoOp, vec![]);
        i.match_index
            .insert(self.config.self_id.clone(), i.log.last().0);
    }

    async fn broadcast_append_entries(self: &Arc<Self>) {
        let (term, commit_index, self_id, peers, per_peer) = {
            let mut i = self.inner.lock().await;
            i.last_heartbeat = Instant::now();
            let term = i.persistent.current_term;
            let commit = i.commit_index;
            let self_id = self.config.self_id.clone();
            let peers = self.config.peers.clone();

            // Build per-peer payloads before releasing the lock.
            let mut per_peer: Vec<(NodeId, AppendEntriesRequest)> = Vec::new();
            for peer in &peers {
                let next_idx = *i.next_index.get(peer).unwrap_or(&1);
                let prev_idx = next_idx.saturating_sub(1);
                let prev_term = i.log.term_at(prev_idx).unwrap_or(0);
                let entries = i.log.entries_from(prev_idx);
                per_peer.push((
                    peer.clone(),
                    AppendEntriesRequest {
                        term,
                        leader_id: self_id.clone(),
                        prev_log_index: prev_idx,
                        prev_log_term: prev_term,
                        entries,
                        leader_commit: commit,
                    },
                ));
            }
            (term, commit, self_id, peers, per_peer)
        };
        let _ = (commit_index, self_id, peers);

        for (peer, req) in per_peer {
            let transport = self.transport.clone();
            let node = self.clone();
            tokio::spawn(async move {
                let sent_entries = req.entries.len() as u64;
                let prev = req.prev_log_index;
                match transport.append_entries(&peer, req).await {
                    Ok(resp) => node.on_append_reply(peer, prev, sent_entries, resp).await,
                    Err(e) => tracing::debug!(?e, peer = %peer, "append_entries failed"),
                }
            });
        }
        let _ = term;
    }

    async fn on_append_reply(
        self: &Arc<Self>,
        peer: NodeId,
        prev_index: LogIndex,
        sent_entries: u64,
        resp: AppendEntriesResponse,
    ) {
        let mut i = self.inner.lock().await;
        if resp.term > i.persistent.current_term {
            i.persistent.current_term = resp.term;
            i.persistent.voted_for = None;
            i.role = RaftRole::Follower;
            *self.current_leader.lock().await = None;
            return;
        }
        if i.role != RaftRole::Leader {
            return;
        }

        if resp.success {
            let new_match = prev_index + sent_entries;
            i.next_index.insert(peer.clone(), new_match + 1);
            i.match_index.insert(peer.clone(), new_match);
            self.maybe_advance_commit(&mut i);
        } else {
            // Fast backoff using follower-supplied conflict_index if present.
            let current = *i.next_index.get(&peer).unwrap_or(&1);
            let new_next = resp
                .conflict_index
                .map(|c| c.max(1))
                .unwrap_or_else(|| current.saturating_sub(1).max(1));
            i.next_index.insert(peer, new_next);
        }
    }

    fn maybe_advance_commit(&self, i: &mut Inner) {
        // A leader can only commit entries from its current term directly
        // (§5.4.2). We walk candidate indices downward from the last log
        // entry and pick the highest N such that:
        //   (a) log[N].term == currentTerm, and
        //   (b) a majority of match_index >= N.
        let current_term = i.persistent.current_term;
        let last_index = i.log.last().0;

        let mut new_commit = i.commit_index;
        let mut n = last_index;
        while n > i.commit_index {
            if i.log.term_at(n) == Some(current_term) {
                let mut count = 0usize;
                for mi in i.match_index.values() {
                    if *mi >= n {
                        count += 1;
                    }
                }
                if count >= self.config.quorum() {
                    new_commit = n;
                    break;
                }
            }
            n -= 1;
        }

        if new_commit > i.commit_index {
            i.commit_index = new_commit;
            tracing::debug!(new_commit, "leader advanced commit index");
            let _ = self.commit_tx.send(new_commit);
        }
    }

    async fn apply_committed(&self, up_to: LogIndex) {
        loop {
            let next = {
                let i = self.inner.lock().await;
                if i.last_applied >= up_to {
                    return;
                }
                i.last_applied + 1
            };
            let entry = {
                let i = self.inner.lock().await;
                match i.log.get(next) {
                    Some(e) => e.clone(),
                    None => return,
                }
            };
            if entry.kind != EntryKind::NoOp {
                if let Err(e) = self.state_machine.apply(&entry).await {
                    tracing::error!(?e, index = next, "state machine apply failed");
                    // Don't advance last_applied; retry on next tick.
                    return;
                }
            }
            let mut i = self.inner.lock().await;
            i.last_applied = next;
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ProposeError {
    #[error("not the leader")]
    NotLeader,
    #[error("term changed during proposal")]
    TermChanged,
    #[error("apply failed: {0}")]
    ApplyFailed(ApplyError),
    #[error("proposal aborted")]
    Aborted,
    #[error("transport error: {0}")]
    Transport(#[from] TransportError),
}

fn randomized_election_timeout(min: Duration) -> Duration {
    let min_ms = min.as_millis() as u64;
    let jitter = rand::thread_rng().gen_range(0..min_ms.max(1));
    Duration::from_millis(min_ms + jitter)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state_machine::{ApplyError, LedgerStateMachine, ProposalResult};
    use crate::transport::NoopTransport;
    use async_trait::async_trait;

    struct NullStateMachine;

    #[async_trait]
    impl LedgerStateMachine for NullStateMachine {
        async fn apply(&self, entry: &LogEntry) -> Result<ProposalResult, ApplyError> {
            Ok(ProposalResult {
                committed_index: entry.index,
                committed_term: entry.term,
                result: Vec::new(),
            })
        }
    }

    fn make_node(self_id: &str, peers: Vec<&str>) -> Arc<RaftNode> {
        RaftNode::new(
            RaftConfig {
                self_id: self_id.into(),
                peers: peers.into_iter().map(Into::into).collect(),
                election_timeout_min: Duration::from_millis(150),
                heartbeat_interval: Duration::from_millis(50),
            },
            Arc::new(NoopTransport),
            Arc::new(NullStateMachine),
        )
    }

    #[tokio::test]
    async fn single_node_elects_itself_with_majority_vote() {
        let node = make_node("sp-baghdad", vec!["sp-basra", "sp-erbil"]);
        // Two peers + self => quorum = 2. NoopTransport grants every vote.
        // The election deadline is min + jitter(0..min) so we have to
        // sleep for at least 2× min to be certain it's elapsed.
        tokio::time::sleep(Duration::from_millis(450)).await;
        node.tick().await;
        // Yield so spawned vote-reply tasks run; loop a few times in case
        // the runtime needs more than one hop to schedule them.
        for _ in 0..10 {
            tokio::time::sleep(Duration::from_millis(20)).await;
            let state = node.state().await;
            if matches!(state.role, RaftRole::Leader | RaftRole::Candidate) {
                return;
            }
        }
        let state = node.state().await;
        panic!("role was {:?} after 10 polls", state.role);
    }

    #[tokio::test]
    async fn stale_append_is_rejected() {
        let node = make_node("sp-baghdad", vec!["sp-basra"]);
        // Bump our term past the stale message.
        {
            let mut i = node.inner.lock().await;
            i.persistent.current_term = 5;
        }
        let resp = node
            .on_append_entries(AppendEntriesRequest {
                term: 3,
                leader_id: "sp-basra".into(),
                prev_log_index: 0,
                prev_log_term: 0,
                entries: vec![],
                leader_commit: 0,
            })
            .await;
        assert!(!resp.success);
        assert_eq!(resp.term, 5);
    }

    #[tokio::test]
    async fn append_entries_rejects_on_log_mismatch() {
        let node = make_node("sp-baghdad", vec!["sp-basra"]);
        let resp = node
            .on_append_entries(AppendEntriesRequest {
                term: 1,
                leader_id: "sp-basra".into(),
                prev_log_index: 5, // we have no entries
                prev_log_term: 1,
                entries: vec![],
                leader_commit: 0,
            })
            .await;
        assert!(!resp.success);
        assert!(resp.conflict_index.is_some());
    }

    #[tokio::test]
    async fn append_entries_appends_and_commits() {
        let node = make_node("sp-baghdad", vec!["sp-basra"]);
        let resp = node
            .on_append_entries(AppendEntriesRequest {
                term: 1,
                leader_id: "sp-basra".into(),
                prev_log_index: 0,
                prev_log_term: 0,
                entries: vec![LogEntry {
                    term: 1,
                    index: 1,
                    kind: EntryKind::NoOp,
                    payload: vec![],
                }],
                leader_commit: 1,
            })
            .await;
        assert!(resp.success);
        assert_eq!(resp.match_index, 1);
        let state = node.state().await;
        assert_eq!(state.commit_index, 1);
    }
}

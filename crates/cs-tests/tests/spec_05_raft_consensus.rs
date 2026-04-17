//! Spec §Architecture Decisions — "Consensus: 3-of-5 Raft (CFT)".
//!
//! Validates the Raft implementation in cs-consensus: quorum math,
//! log-replication semantics, term transitions, leader behavior.

use cs_consensus::log::{EntryKind, LogEntry, RaftLog};
use cs_consensus::node::{RaftConfig, RaftNode, RaftRole};
use cs_consensus::rpc::{
    AppendEntriesRequest, AppendEntriesResponse, RequestVoteRequest, RequestVoteResponse,
};
use cs_consensus::state_machine::{ApplyError, LedgerStateMachine, ProposalResult};
use cs_consensus::transport::{NoopTransport, PeerTransport};
use std::sync::Arc;
use std::time::Duration;

struct NullApplier;
#[async_trait::async_trait]
impl LedgerStateMachine for NullApplier {
    async fn apply(&self, entry: &LogEntry) -> Result<ProposalResult, ApplyError> {
        Ok(ProposalResult {
            committed_index: entry.index,
            committed_term: entry.term,
            result: vec![],
        })
    }
}

fn make_node(self_id: &str, peers: Vec<&str>) -> Arc<RaftNode> {
    RaftNode::new(
        RaftConfig {
            self_id: self_id.into(),
            peers: peers.into_iter().map(Into::into).collect(),
            election_timeout_min: Duration::from_millis(100),
            heartbeat_interval: Duration::from_millis(40),
        },
        Arc::new(NoopTransport),
        Arc::new(NullApplier),
    )
}

#[test]
fn spec_quorum_math_3_of_5() {
    let cfg = RaftConfig {
        self_id: "self".into(),
        peers: vec!["a".into(), "b".into(), "c".into(), "d".into()],
        election_timeout_min: Duration::from_millis(150),
        heartbeat_interval: Duration::from_millis(50),
    };
    // Majority of 5 = 3.
    assert_eq!(cfg.quorum(), 3, "Spec violation: 5-node cluster must require 3-of-5 quorum");
}

#[test]
fn spec_quorum_math_2_of_3() {
    let cfg = RaftConfig {
        self_id: "self".into(),
        peers: vec!["a".into(), "b".into()],
        election_timeout_min: Duration::from_millis(150),
        heartbeat_interval: Duration::from_millis(50),
    };
    assert_eq!(cfg.quorum(), 2, "3-node cluster must require 2-of-3 quorum");
}

#[tokio::test]
async fn spec_stale_term_append_is_rejected() {
    let node = make_node("self", vec!["a", "b", "c", "d"]);

    // Bump our term forward.
    let high_term = 10;
    node.on_append_entries(AppendEntriesRequest {
        term: high_term,
        leader_id: "a".into(),
        prev_log_index: 0,
        prev_log_term: 0,
        entries: vec![],
        leader_commit: 0,
    })
    .await;

    // Now a stale-term RPC must be rejected.
    let resp: AppendEntriesResponse = node
        .on_append_entries(AppendEntriesRequest {
            term: 3,
            leader_id: "b".into(),
            prev_log_index: 0,
            prev_log_term: 0,
            entries: vec![],
            leader_commit: 0,
        })
        .await;
    assert!(!resp.success, "Spec violation: stale-term append must fail");
    assert!(resp.term >= high_term, "Response must echo the current term");
}

#[tokio::test]
async fn spec_log_conflict_returns_conflict_index() {
    let node = make_node("self", vec!["a", "b"]);
    // Follower has no entries, leader claims prev_log_index=5.
    let resp = node
        .on_append_entries(AppendEntriesRequest {
            term: 1,
            leader_id: "a".into(),
            prev_log_index: 5,
            prev_log_term: 1,
            entries: vec![],
            leader_commit: 0,
        })
        .await;
    assert!(!resp.success);
    assert!(resp.conflict_index.is_some(), "Spec: must return a conflict hint for fast backoff");
}

#[tokio::test]
async fn spec_append_entries_extends_log_and_advances_commit() {
    let node = make_node("self", vec!["a", "b"]);
    let entries = vec![LogEntry {
        term: 1,
        index: 1,
        kind: EntryKind::NoOp,
        payload: vec![],
    }];
    let resp = node
        .on_append_entries(AppendEntriesRequest {
            term: 1,
            leader_id: "a".into(),
            prev_log_index: 0,
            prev_log_term: 0,
            entries,
            leader_commit: 1,
        })
        .await;
    assert!(resp.success);
    let state = node.state().await;
    assert_eq!(state.commit_index, 1, "Commit index must advance to leader_commit");
    assert_eq!(state.last_log_index, 1);
}

#[tokio::test]
async fn spec_request_vote_rejects_stale_log() {
    let node = make_node("self", vec!["a", "b"]);
    // Seed some log entries on self.
    node.on_append_entries(AppendEntriesRequest {
        term: 1,
        leader_id: "a".into(),
        prev_log_index: 0,
        prev_log_term: 0,
        entries: vec![
            LogEntry { term: 1, index: 1, kind: EntryKind::NoOp, payload: vec![] },
            LogEntry { term: 1, index: 2, kind: EntryKind::NoOp, payload: vec![] },
        ],
        leader_commit: 0,
    })
    .await;

    // Candidate whose log is behind must NOT win our vote.
    let resp: RequestVoteResponse = node
        .on_request_vote(RequestVoteRequest {
            term: 5,
            candidate_id: "impostor".into(),
            last_log_index: 0,
            last_log_term: 0,
        })
        .await;
    assert!(!resp.vote_granted, "Spec §5.4: reject vote when candidate log is behind");
}

#[test]
fn spec_raft_log_append_returns_1_indexed() {
    let mut log = RaftLog::new();
    let idx = log.append_new(1, EntryKind::NoOp, vec![]);
    assert_eq!(idx, 1, "First entry must be at index 1 (1-indexed)");
    let idx2 = log.append_new(1, EntryKind::NoOp, vec![]);
    assert_eq!(idx2, 2);
}

#[test]
fn spec_raft_log_truncate_is_destructive() {
    let mut log = RaftLog::new();
    log.append_new(1, EntryKind::NoOp, vec![]);
    log.append_new(1, EntryKind::NoOp, vec![]);
    log.append_new(2, EntryKind::NoOp, vec![]);
    log.truncate(1);
    assert_eq!(log.len(), 1);
    assert_eq!(log.last(), (1, 1));
}

#[tokio::test]
async fn spec_initial_role_is_follower() {
    let node = make_node("self", vec!["a", "b"]);
    let state = node.state().await;
    assert_eq!(state.role, RaftRole::Follower, "Raft nodes must start as followers");
    assert_eq!(state.term, 0);
    assert_eq!(state.commit_index, 0);
}

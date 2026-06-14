//! Civic-work domain models and lifecycle controls.
//!
//! This crate intentionally contains policy-facing primitives rather than
//! payment plumbing. It models when a civic-work task is authorized, verified,
//! and eligible for payment evidence.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum CivicWorkCategory {
    MunicipalRepair,
    EnvironmentalRestoration,
    VisitorServices,
    CareSupport,
    CulturalHeritage,
    DisasterResilience,
    TrainingBridge,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SafetyClass {
    Low,
    Controlled,
    Restricted,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum TaskStatus {
    PendingApproval,
    Approved,
    OpenForEnrollment,
    InProgress,
    EvidenceSubmitted,
    Verified,
    PaymentReleased,
    Suspended,
    Rejected,
    Closed,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum VerificationStatus {
    Pending,
    Approved,
    Rejected,
    RequiresReview,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CivicWorkTask {
    pub task_id: Uuid,
    pub jurisdiction: String,
    pub category: CivicWorkCategory,
    pub title: String,
    pub status: TaskStatus,
    pub local_authority_id: Option<String>,
    pub supervisor_id: Option<String>,
    pub planned_worker_slots: u32,
    pub hourly_rate_iqd: u64,
    pub evidence_requirements: Vec<String>,
    pub safety_class: SafetyClass,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl CivicWorkTask {
    pub fn new(
        jurisdiction: impl Into<String>,
        category: CivicWorkCategory,
        title: impl Into<String>,
        planned_worker_slots: u32,
        hourly_rate_iqd: u64,
        evidence_requirements: Vec<String>,
        safety_class: SafetyClass,
    ) -> Self {
        let now = Utc::now();
        Self {
            task_id: Uuid::new_v4(),
            jurisdiction: jurisdiction.into(),
            category,
            title: title.into(),
            status: TaskStatus::PendingApproval,
            local_authority_id: None,
            supervisor_id: None,
            planned_worker_slots,
            hourly_rate_iqd,
            evidence_requirements,
            safety_class,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn attach_governance(
        &mut self,
        local_authority_id: impl Into<String>,
        supervisor_id: impl Into<String>,
    ) {
        self.local_authority_id = Some(local_authority_id.into());
        self.supervisor_id = Some(supervisor_id.into());
        self.updated_at = Utc::now();
    }

    pub fn transition(&mut self, next: TaskStatus) -> Result<(), CivicWorkError> {
        if !is_allowed_transition(self.status, next) {
            return Err(CivicWorkError::InvalidTransition {
                from: self.status,
                to: next,
            });
        }
        self.status = next;
        self.updated_at = Utc::now();
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EvidenceItem {
    pub evidence_id: Uuid,
    pub kind: String,
    pub uri: String,
    pub submitted_at: DateTime<Utc>,
    pub supervisor_id: Option<String>,
    pub geo_hash: Option<String>,
}

impl EvidenceItem {
    pub fn new(kind: impl Into<String>, uri: impl Into<String>) -> Self {
        Self {
            evidence_id: Uuid::new_v4(),
            kind: kind.into(),
            uri: uri.into(),
            submitted_at: Utc::now(),
            supervisor_id: None,
            geo_hash: None,
        }
    }

    pub fn with_supervisor(mut self, supervisor_id: impl Into<String>) -> Self {
        self.supervisor_id = Some(supervisor_id.into());
        self
    }

    pub fn with_geo_hash(mut self, geo_hash: impl Into<String>) -> Self {
        self.geo_hash = Some(geo_hash.into());
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VerificationDecision {
    pub status: VerificationStatus,
    pub verified_by: Option<String>,
    pub reason: Option<String>,
    pub decided_at: DateTime<Utc>,
}

impl VerificationDecision {
    pub fn pending() -> Self {
        Self {
            status: VerificationStatus::Pending,
            verified_by: None,
            reason: None,
            decided_at: Utc::now(),
        }
    }

    pub fn approved(verified_by: impl Into<String>) -> Self {
        Self {
            status: VerificationStatus::Approved,
            verified_by: Some(verified_by.into()),
            reason: None,
            decided_at: Utc::now(),
        }
    }

    pub fn rejected(verified_by: impl Into<String>, reason: impl Into<String>) -> Self {
        Self {
            status: VerificationStatus::Rejected,
            verified_by: Some(verified_by.into()),
            reason: Some(reason.into()),
            decided_at: Utc::now(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WorkerAssignment {
    pub assignment_id: Uuid,
    pub task_id: Uuid,
    pub worker_id: String,
    pub hours_claimed: u16,
    pub evidence: Vec<EvidenceItem>,
    pub verification: VerificationDecision,
    pub duplicate_risk_score: f32,
    pub safety_incident: bool,
}

impl WorkerAssignment {
    pub fn new(task_id: Uuid, worker_id: impl Into<String>) -> Self {
        Self {
            assignment_id: Uuid::new_v4(),
            task_id,
            worker_id: worker_id.into(),
            hours_claimed: 0,
            evidence: vec![],
            verification: VerificationDecision::pending(),
            duplicate_risk_score: 0.0,
            safety_incident: false,
        }
    }

    pub fn claim_hours(&mut self, hours: u16) {
        self.hours_claimed = hours;
    }

    pub fn submit_evidence(&mut self, evidence: EvidenceItem) {
        self.evidence.push(evidence);
    }

    pub fn set_duplicate_risk_score(&mut self, score: f32) {
        self.duplicate_risk_score = score;
    }

    pub fn mark_safety_incident(&mut self) {
        self.safety_incident = true;
    }

    pub fn decide(&mut self, decision: VerificationDecision) {
        self.verification = decision;
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProgramControls {
    pub max_hours_per_week: u16,
    pub duplicate_risk_hold_threshold_bps: u16,
    pub requires_local_authority: bool,
    pub requires_supervisor: bool,
    pub requires_geotagged_evidence: bool,
    pub allow_payment_without_verification: bool,
}

impl Default for ProgramControls {
    fn default() -> Self {
        Self {
            max_hours_per_week: 24,
            duplicate_risk_hold_threshold_bps: 8_000,
            requires_local_authority: true,
            requires_supervisor: true,
            requires_geotagged_evidence: true,
            allow_payment_without_verification: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PaymentInstruction {
    pub task_id: Uuid,
    pub assignment_id: Uuid,
    pub worker_id: String,
    pub amount_iqd: u64,
    pub evidence_count: usize,
}

pub fn evaluate_assignment_for_payment(
    task: &CivicWorkTask,
    assignment: &WorkerAssignment,
    controls: &ProgramControls,
) -> Result<PaymentInstruction, CivicWorkError> {
    if task.task_id != assignment.task_id {
        return Err(CivicWorkError::TaskAssignmentMismatch);
    }

    if controls.requires_local_authority && task.local_authority_id.is_none() {
        return Err(CivicWorkError::MissingLocalAuthority);
    }

    if controls.requires_supervisor && task.supervisor_id.is_none() {
        return Err(CivicWorkError::MissingSupervisor);
    }

    if !controls.allow_payment_without_verification && task.status != TaskStatus::Verified {
        return Err(CivicWorkError::TaskNotVerified);
    }

    if assignment.verification.status != VerificationStatus::Approved {
        return Err(CivicWorkError::AssignmentNotApproved);
    }

    if assignment.hours_claimed == 0 || assignment.hours_claimed > controls.max_hours_per_week {
        return Err(CivicWorkError::InvalidHours {
            claimed: assignment.hours_claimed,
            max: controls.max_hours_per_week,
        });
    }

    if assignment.evidence.is_empty() {
        return Err(CivicWorkError::MissingEvidence);
    }

    if controls.requires_geotagged_evidence
        && !assignment.evidence.iter().any(|item| {
            item.geo_hash
                .as_deref()
                .is_some_and(|value| !value.is_empty())
        })
    {
        return Err(CivicWorkError::MissingGeotaggedEvidence);
    }

    let threshold = f32::from(controls.duplicate_risk_hold_threshold_bps) / 10_000.0;
    if assignment.duplicate_risk_score >= threshold {
        return Err(CivicWorkError::DuplicateClaimRisk);
    }

    if assignment.safety_incident {
        return Err(CivicWorkError::SafetyIncidentHold);
    }

    Ok(PaymentInstruction {
        task_id: task.task_id,
        assignment_id: assignment.assignment_id,
        worker_id: assignment.worker_id.clone(),
        amount_iqd: u64::from(assignment.hours_claimed) * task.hourly_rate_iqd,
        evidence_count: assignment.evidence.len(),
    })
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum CivicWorkError {
    #[error("invalid transition from {from:?} to {to:?}")]
    InvalidTransition { from: TaskStatus, to: TaskStatus },
    #[error("assignment belongs to a different task")]
    TaskAssignmentMismatch,
    #[error("task is missing local authority approval")]
    MissingLocalAuthority,
    #[error("task is missing supervisor assignment")]
    MissingSupervisor,
    #[error("task has not reached verified status")]
    TaskNotVerified,
    #[error("assignment has not been approved")]
    AssignmentNotApproved,
    #[error("claimed hours {claimed} exceed allowed range 1..={max}")]
    InvalidHours { claimed: u16, max: u16 },
    #[error("assignment has no evidence")]
    MissingEvidence,
    #[error("assignment has no geotagged evidence")]
    MissingGeotaggedEvidence,
    #[error("assignment duplicate-claim risk is too high")]
    DuplicateClaimRisk,
    #[error("assignment is held because of a safety incident")]
    SafetyIncidentHold,
}

fn is_allowed_transition(from: TaskStatus, to: TaskStatus) -> bool {
    use TaskStatus::*;
    matches!(
        (from, to),
        (PendingApproval, Approved | Rejected | Suspended)
            | (Approved, OpenForEnrollment | Suspended)
            | (OpenForEnrollment, InProgress | Suspended | Closed)
            | (InProgress, EvidenceSubmitted | Suspended | Closed)
            | (EvidenceSubmitted, Verified | Rejected | Suspended)
            | (Verified, PaymentReleased | Closed)
            | (PaymentReleased, Closed)
            | (Suspended, InProgress | Closed | Rejected)
            | (Rejected, Closed)
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn task_lifecycle_reaches_payment_release() {
        let mut task = governed_task();

        task.transition(TaskStatus::Approved).unwrap();
        task.transition(TaskStatus::OpenForEnrollment).unwrap();
        task.transition(TaskStatus::InProgress).unwrap();
        task.transition(TaskStatus::EvidenceSubmitted).unwrap();
        task.transition(TaskStatus::Verified).unwrap();
        task.transition(TaskStatus::PaymentReleased).unwrap();

        assert_eq!(task.status, TaskStatus::PaymentReleased);
    }

    #[test]
    fn invalid_transition_is_rejected() {
        let mut task = governed_task();

        let err = task.transition(TaskStatus::PaymentReleased).unwrap_err();

        assert_eq!(
            err,
            CivicWorkError::InvalidTransition {
                from: TaskStatus::PendingApproval,
                to: TaskStatus::PaymentReleased,
            }
        );
        assert_eq!(task.status, TaskStatus::PendingApproval);
    }

    #[test]
    fn approved_assignment_generates_payment_instruction() {
        let mut task = verified_task();
        task.hourly_rate_iqd = 8_000;
        let mut assignment = approved_assignment(task.task_id);
        assignment.claim_hours(12);

        let payment =
            evaluate_assignment_for_payment(&task, &assignment, &ProgramControls::default())
                .unwrap();

        assert_eq!(payment.amount_iqd, 96_000);
        assert_eq!(payment.worker_id, "worker-001");
        assert_eq!(payment.evidence_count, 1);
    }

    #[test]
    fn missing_evidence_blocks_payment() {
        let task = verified_task();
        let mut assignment = WorkerAssignment::new(task.task_id, "worker-001");
        assignment.claim_hours(8);
        assignment.decide(VerificationDecision::approved("supervisor-001"));

        let err = evaluate_assignment_for_payment(
            &task,
            &assignment,
            &ProgramControls {
                requires_geotagged_evidence: false,
                ..ProgramControls::default()
            },
        )
        .unwrap_err();

        assert_eq!(err, CivicWorkError::MissingEvidence);
    }

    #[test]
    fn duplicate_risk_blocks_payment() {
        let task = verified_task();
        let mut assignment = approved_assignment(task.task_id);
        assignment.claim_hours(8);
        assignment.set_duplicate_risk_score(0.93);

        let err = evaluate_assignment_for_payment(&task, &assignment, &ProgramControls::default())
            .unwrap_err();

        assert_eq!(err, CivicWorkError::DuplicateClaimRisk);
    }

    #[test]
    fn safety_incident_blocks_payment() {
        let task = verified_task();
        let mut assignment = approved_assignment(task.task_id);
        assignment.claim_hours(8);
        assignment.mark_safety_incident();

        let err = evaluate_assignment_for_payment(&task, &assignment, &ProgramControls::default())
            .unwrap_err();

        assert_eq!(err, CivicWorkError::SafetyIncidentHold);
    }

    #[test]
    fn missing_local_authority_blocks_payment() {
        let mut task = CivicWorkTask::new(
            "Najaf",
            CivicWorkCategory::VisitorServices,
            "Visitor corridor support",
            40,
            7_500,
            vec!["photo".to_string()],
            SafetyClass::Low,
        );
        task.supervisor_id = Some("supervisor-001".to_string());
        task.status = TaskStatus::Verified;
        let mut assignment = approved_assignment(task.task_id);
        assignment.claim_hours(8);

        let err = evaluate_assignment_for_payment(&task, &assignment, &ProgramControls::default())
            .unwrap_err();

        assert_eq!(err, CivicWorkError::MissingLocalAuthority);
    }

    #[test]
    fn excessive_hours_are_held() {
        let task = verified_task();
        let mut assignment = approved_assignment(task.task_id);
        assignment.claim_hours(48);

        let err = evaluate_assignment_for_payment(&task, &assignment, &ProgramControls::default())
            .unwrap_err();

        assert_eq!(
            err,
            CivicWorkError::InvalidHours {
                claimed: 48,
                max: 24,
            }
        );
    }

    fn governed_task() -> CivicWorkTask {
        let mut task = CivicWorkTask::new(
            "Najaf",
            CivicWorkCategory::VisitorServices,
            "Visitor corridor support",
            40,
            7_500,
            vec!["photo".to_string(), "supervisor signoff".to_string()],
            SafetyClass::Low,
        );
        task.attach_governance("najaf-municipality", "supervisor-001");
        task
    }

    fn verified_task() -> CivicWorkTask {
        let mut task = governed_task();
        task.status = TaskStatus::Verified;
        task
    }

    fn approved_assignment(task_id: Uuid) -> WorkerAssignment {
        let mut assignment = WorkerAssignment::new(task_id, "worker-001");
        assignment.submit_evidence(
            EvidenceItem::new("photo", "cs://evidence/001")
                .with_supervisor("supervisor-001")
                .with_geo_hash("najaf-corridor-12"),
        );
        assignment.decide(VerificationDecision::approved("supervisor-001"));
        assignment
    }
}

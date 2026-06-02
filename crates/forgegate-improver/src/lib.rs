//! Prompt/policy improvement candidate contracts.
//!
//! Phase 5 implements failure summarization, candidate generation, eval-gated promotion.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImprovementCandidate {
    pub candidate_id: String,
    pub agent_id: String,
    pub base_version: u32,
    pub change_type: ChangeType,
    pub hypothesis: String,
    pub status: CandidateStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChangeType {
    Prompt,
    Policy,
    PromptAndPolicy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CandidateStatus {
    Draft,
    PendingEval,
    PassedEval,
    FailedEval,
    Promoted,
    Rejected,
}

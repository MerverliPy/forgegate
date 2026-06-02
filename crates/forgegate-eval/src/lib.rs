//! Eval case and scoring contracts.
//!
//! Phase 3 implements parsing, fixture isolation, checks, and scoring.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvalCase {
    pub id: String,
    pub name: String,
    pub task: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvalResult {
    pub eval_id: String,
    pub passed: bool,
    pub score: f64,
}

//! Coding-tool contracts for ForgeGate.
//!
//! Phase 2 replaces these stubs with concrete read/write/git/test/shell tools.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolResult {
    pub tool: String,
    pub success: bool,
    pub output: String,
    pub error: Option<String>,
}

pub trait CodingTool {
    fn name(&self) -> &'static str;
    fn run(&self, args: serde_json::Value) -> anyhow::Result<ToolResult>;
}

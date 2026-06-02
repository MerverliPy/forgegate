use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunTrace {
    pub trace_id: String,
    pub run_id: String,
    pub agent_id: String,
    pub agent_version: u32,
    pub task: String,
    pub started_at: DateTime<Utc>,
    pub finished_at: Option<DateTime<Utc>>,
    pub status: RunStatus,
    pub score: Option<f64>,
    pub events: Vec<TraceEvent>,
    pub outcome: Option<RunOutcome>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum RunStatus {
    Queued,
    Running,
    Passed,
    Failed,
    BlockedByPolicy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum TraceEvent {
    ModelCall {
        model: String,
        input_tokens: u32,
        output_tokens: u32,
        latency_ms: u64,
    },
    ToolCall {
        tool: String,
        args: serde_json::Value,
        status: ToolStatus,
        latency_ms: u64,
        error: Option<String>,
    },
    PolicyDecision {
        allowed: bool,
        reason: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ToolStatus {
    Success,
    Failed,
    Blocked,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunOutcome {
    pub summary: String,
    pub failure_category: Option<String>,
}

impl RunTrace {
    pub fn mock(task: &str) -> Self {
        let now = Utc::now();
        Self {
            trace_id: format!("trace_{}", Uuid::new_v4().simple()),
            run_id: format!("run_{}", Uuid::new_v4().simple()),
            agent_id: "coding-agent".to_string(),
            agent_version: 1,
            task: task.to_string(),
            started_at: now,
            finished_at: Some(now),
            status: RunStatus::Passed,
            score: Some(0.50),
            events: vec![
                TraceEvent::PolicyDecision {
                    allowed: true,
                    reason: "mock run allowed by scaffold policy".to_string(),
                },
                TraceEvent::ModelCall {
                    model: "mock-model".to_string(),
                    input_tokens: 1200,
                    output_tokens: 400,
                    latency_ms: 25,
                },
                TraceEvent::ToolCall {
                    tool: "git_diff".to_string(),
                    args: serde_json::json!({}),
                    status: ToolStatus::Success,
                    latency_ms: 10,
                    error: None,
                },
            ],
            outcome: Some(RunOutcome {
                summary: "mock run completed; replace with real coding-agent loop in Phase 1"
                    .to_string(),
                failure_category: None,
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trace_serializes_to_json() {
        let trace = RunTrace::mock("test task");
        let json = serde_json::to_string(&trace).expect("trace should serialize");
        assert!(json.contains("coding-agent"));
        assert!(json.contains("model_call"));
    }
}

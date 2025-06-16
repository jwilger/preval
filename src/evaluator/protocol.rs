use serde::{Deserialize, Serialize};

/// Evaluation mode for the evaluator
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum EvaluationMode {
    TestSuite,
    OnlineCollection,
    Continuous,
}

/// Information about the evaluator
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct EvaluatorInfo {
    pub name: String,
    pub description: Option<String>,
    pub version: Option<String>,
}

/// Execution plan for the evaluation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct ExecutionPlan {
    pub total_samples: u32,
    pub batch_size: Option<u32>,
}

/// Metric definition in the handshake
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct MetricDefinition {
    pub name: String,
    pub description: Option<String>,
    pub unit: Option<String>,
}

/// Handshake message sent by evaluator at startup
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct Handshake {
    #[serde(rename = "type")]
    pub msg_type: String,
    pub mode: EvaluationMode,
    pub version: String,
    pub evaluator: EvaluatorInfo,
    pub execution_plan: Option<ExecutionPlan>,
    pub metrics_schema: Vec<MetricDefinition>,
}

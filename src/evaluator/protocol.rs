use serde::{Deserialize, Serialize};

/// Evaluation mode for the evaluator
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvaluationMode {
    TestSuite,
    OnlineCollection,
    Continuous,
}

/// Information about the evaluator
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvaluatorInfo {
    pub name: String,
    pub description: Option<String>,
    pub version: Option<String>,
}

/// Execution plan for the evaluation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionPlan {
    pub total_samples: u32,
    pub batch_size: Option<u32>,
}

/// Metric definition in the handshake
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricDefinition {
    pub name: String,
    pub description: Option<String>,
    pub unit: Option<String>,
}

/// Handshake message sent by evaluator at startup
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Handshake {
    #[serde(rename = "type")]
    pub msg_type: String,
    pub mode: EvaluationMode,
    pub version: String,
    pub evaluator: EvaluatorInfo,
    pub execution_plan: Option<ExecutionPlan>,
    pub metrics_schema: Vec<MetricDefinition>,
}

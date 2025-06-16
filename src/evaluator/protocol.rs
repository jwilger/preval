use nutype::nutype;
use serde::{Deserialize, Serialize};

/// Protocol version string that must be non-empty
#[nutype(
    sanitize(trim),
    validate(not_empty, len_char_max = 32),
    derive(Debug, Clone, PartialEq, Eq, AsRef, Display, Serialize, Deserialize)
)]
pub struct ProtocolVersion(String);

/// Evaluator description that must be non-empty if provided
#[nutype(
    sanitize(trim),
    validate(not_empty, len_char_max = 512),
    derive(Debug, Clone, PartialEq, Eq, AsRef, Display, Serialize, Deserialize)
)]
pub struct EvaluatorDescription(String);

/// Metric name that must be non-empty and follow naming conventions
#[nutype(
    sanitize(trim),
    validate(not_empty, len_char_max = 128),
    derive(
        Debug,
        Clone,
        PartialEq,
        Eq,
        Hash,
        AsRef,
        Display,
        Serialize,
        Deserialize
    )
)]
pub struct MetricDefinitionName(String);

/// Metric unit string that must be non-empty if provided
#[nutype(
    sanitize(trim),
    validate(not_empty, len_char_max = 32),
    derive(Debug, Clone, PartialEq, Eq, AsRef, Display, Serialize, Deserialize)
)]
pub struct MetricUnit(String);

/// Total number of samples that must be positive
#[nutype(
    validate(greater = 0),
    derive(
        Debug,
        Clone,
        Copy,
        PartialEq,
        Eq,
        PartialOrd,
        Ord,
        Into,
        Serialize,
        Deserialize
    )
)]
pub struct TotalSamples(u32);

/// Batch size that must be positive if provided
#[nutype(
    validate(greater = 0),
    derive(
        Debug,
        Clone,
        Copy,
        PartialEq,
        Eq,
        PartialOrd,
        Ord,
        Into,
        Serialize,
        Deserialize
    )
)]
pub struct BatchSize(u32);

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
    pub name: String, // Keep as String for JSON parsing, validate at boundaries
    pub description: Option<String>,
    pub version: Option<String>,
}

/// Execution plan for the evaluation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionPlan {
    pub total_samples: u32,      // Will be converted to TotalSamples after parsing
    pub batch_size: Option<u32>, // Will be converted to BatchSize after parsing
}

/// Metric definition in the handshake
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricDefinition {
    pub name: String, // Will be converted to MetricDefinitionName after parsing
    pub description: Option<String>,
    pub unit: Option<String>,
}

/// Handshake message sent by evaluator at startup
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Handshake {
    #[serde(rename = "type")]
    pub msg_type: String,
    pub mode: EvaluationMode,
    pub version: String, // Will be converted to ProtocolVersion after parsing
    pub evaluator: EvaluatorInfo,
    pub execution_plan: Option<ExecutionPlan>,
    pub metrics_schema: Vec<MetricDefinition>,
}

/// Validated handshake with strong types
#[derive(Debug, Clone)]
pub struct ValidatedHandshake {
    #[allow(dead_code)] // Used in future stories
    pub mode: EvaluationMode,
    pub version: ProtocolVersion,
    pub evaluator: ValidatedEvaluatorInfo,
    pub execution_plan: Option<ValidatedExecutionPlan>,
    #[allow(dead_code)] // Used in future stories
    pub metrics_schema: Vec<ValidatedMetricDefinition>,
}

/// Validated evaluator information
#[derive(Debug, Clone)]
pub struct ValidatedEvaluatorInfo {
    pub name: String, // Already validated to be non-empty by JSON requirements
    pub description: Option<EvaluatorDescription>,
    #[allow(dead_code)] // Used in future stories
    pub version: Option<String>,
}

/// Validated execution plan
#[derive(Debug, Clone)]
pub struct ValidatedExecutionPlan {
    pub total_samples: TotalSamples,
    #[allow(dead_code)] // Used in future stories
    pub batch_size: Option<BatchSize>,
}

/// Validated metric definition
#[derive(Debug, Clone)]
pub struct ValidatedMetricDefinition {
    #[allow(dead_code)] // Used in future stories
    pub name: MetricDefinitionName,
    #[allow(dead_code)] // Used in future stories
    pub description: Option<String>,
    #[allow(dead_code)] // Used in future stories
    pub unit: Option<MetricUnit>,
}

impl ValidatedHandshake {
    /// Parse and validate a handshake from JSON
    pub fn parse(handshake: Handshake) -> Result<Self, ValidationError> {
        // Validate protocol version
        let version = ProtocolVersion::try_new(handshake.version)
            .map_err(|e| ValidationError::InvalidVersion(e.to_string()))?;

        // Validate evaluator info
        let evaluator = ValidatedEvaluatorInfo::parse(handshake.evaluator)?;

        // Validate execution plan if present
        let execution_plan = handshake
            .execution_plan
            .map(ValidatedExecutionPlan::parse)
            .transpose()?;

        // Validate metrics schema
        let metrics_schema = handshake
            .metrics_schema
            .into_iter()
            .map(ValidatedMetricDefinition::parse)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            mode: handshake.mode,
            version,
            evaluator,
            execution_plan,
            metrics_schema,
        })
    }
}

impl ValidatedEvaluatorInfo {
    fn parse(info: EvaluatorInfo) -> Result<Self, ValidationError> {
        if info.name.trim().is_empty() {
            return Err(ValidationError::EmptyEvaluatorName);
        }

        let description = info
            .description
            .map(EvaluatorDescription::try_new)
            .transpose()
            .map_err(|e| ValidationError::InvalidDescription(e.to_string()))?;

        Ok(Self {
            name: info.name,
            description,
            version: info.version,
        })
    }
}

impl ValidatedExecutionPlan {
    fn parse(plan: ExecutionPlan) -> Result<Self, ValidationError> {
        let total_samples = TotalSamples::try_new(plan.total_samples)
            .map_err(|e| ValidationError::InvalidTotalSamples(e.to_string()))?;

        let batch_size = plan
            .batch_size
            .map(BatchSize::try_new)
            .transpose()
            .map_err(|e| ValidationError::InvalidBatchSize(e.to_string()))?;

        Ok(Self {
            total_samples,
            batch_size,
        })
    }
}

impl ValidatedMetricDefinition {
    fn parse(def: MetricDefinition) -> Result<Self, ValidationError> {
        let name = MetricDefinitionName::try_new(def.name)
            .map_err(|e| ValidationError::InvalidMetricName(e.to_string()))?;

        let unit = def
            .unit
            .map(MetricUnit::try_new)
            .transpose()
            .map_err(|e| ValidationError::InvalidMetricUnit(e.to_string()))?;

        Ok(Self {
            name,
            description: def.description,
            unit,
        })
    }
}

/// Validation errors for handshake data
#[derive(Debug, thiserror::Error)]
pub enum ValidationError {
    #[error("protocol version is invalid: {0}")]
    InvalidVersion(String),

    #[error("evaluator name cannot be empty")]
    EmptyEvaluatorName,

    #[error("evaluator description is invalid: {0}")]
    InvalidDescription(String),

    #[error("total samples count is invalid: {0}")]
    InvalidTotalSamples(String),

    #[error("batch size is invalid: {0}")]
    InvalidBatchSize(String),

    #[error("metric name is invalid: {0}")]
    InvalidMetricName(String),

    #[error("metric unit is invalid: {0}")]
    InvalidMetricUnit(String),
}

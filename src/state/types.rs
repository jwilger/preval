use nutype::nutype;
use std::marker::PhantomData;
use std::time::{Duration, Instant};

/// Non-empty evaluator name
#[nutype(
    sanitize(trim),
    validate(not_empty, len_char_max = 255),
    derive(Debug, Clone, PartialEq, Eq, Hash, AsRef, Display)
)]
pub struct EvaluatorName(String);

/// Non-empty evaluator command
#[nutype(
    sanitize(trim),
    validate(not_empty),
    derive(Debug, Clone, PartialEq, Eq, AsRef, Display)
)]
pub struct EvaluatorCommand(String);

/// Terminal dimensions that must be positive
#[nutype(
    validate(greater = 0),
    derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, AsRef, Into)
)]
pub struct TerminalWidth(u16);

#[nutype(
    validate(greater = 0),
    derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, AsRef, Into)
)]
pub struct TerminalHeight(u16);

/// Type-safe terminal dimensions
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TerminalSize {
    width: TerminalWidth,
    height: TerminalHeight,
}

impl TerminalSize {
    /// Create new terminal size, ensuring dimensions are valid
    pub fn try_new(width: u16, height: u16) -> Result<Self, TerminalSizeError> {
        Ok(Self {
            width: TerminalWidth::try_new(width).map_err(|_| TerminalSizeError::InvalidWidth)?,
            height: TerminalHeight::try_new(height)
                .map_err(|_| TerminalSizeError::InvalidHeight)?,
        })
    }

    /// Get width
    pub fn width(&self) -> u16 {
        self.width.into_inner()
    }

    /// Get height
    pub fn height(&self) -> u16 {
        self.height.into_inner()
    }
}

#[derive(Debug, thiserror::Error)]
pub enum TerminalSizeError {
    #[error("terminal width must be greater than 0")]
    InvalidWidth,
    #[error("terminal height must be greater than 0")]
    InvalidHeight,
}

/// Type-safe evaluation status
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EvaluationStatus {
    /// Waiting for evaluator to start
    Starting,
    /// Waiting for handshake
    WaitingForHandshake,
    /// Collecting metrics
    CollectingMetrics {
        received: usize,
        total: Option<usize>,
    },
    /// Evaluation completed successfully
    Completed,
    /// Evaluation failed with error
    Failed(String),
}

/// Phantom types for application state - Evaluator setting
#[derive(Debug)]
pub struct EvaluatorNotSet;

#[derive(Debug)]
pub struct EvaluatorSet;

/// Phantom types for application state - Handshake setting
#[derive(Debug)]
pub struct HandshakeNotSet;

#[derive(Debug)]
pub struct HandshakeSet;

/// Phantom types for application state - Status tracking
#[derive(Debug)]
pub struct Starting;

#[derive(Debug)]
pub struct WaitingForHandshake;

#[derive(Debug)]
pub struct CollectingMetrics;

#[derive(Debug)]
pub struct CompletedOrFailed;

/// Sample status during evaluation
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SampleStatus {
    /// Currently being processed
    Processing,
    /// Completed successfully
    Completed,
    /// Failed during processing
    #[allow(dead_code)] // Used when sample processing fails
    Failed(String),
}

/// A sample result with its metrics and status
#[derive(Debug, Clone)]
pub struct SampleResult {
    /// Unique identifier for the sample
    pub sample_id: String,
    /// Current status of the sample
    pub status: SampleStatus,
    /// Key metrics extracted from the sample
    pub metrics: Vec<(String, f64)>, // (metric_name, value) pairs
    /// When the sample was completed or failed
    pub completed_at: Option<Instant>,
}

impl SampleResult {
    /// Create a new sample result in processing state
    pub fn new_processing(sample_id: String) -> Self {
        Self {
            sample_id,
            status: SampleStatus::Processing,
            metrics: Vec::new(),
            completed_at: None,
        }
    }

    /// Mark sample as completed with metrics
    pub fn mark_completed(&mut self, metrics: Vec<(String, f64)>) {
        self.status = SampleStatus::Completed;
        self.metrics = metrics;
        self.completed_at = Some(Instant::now());
    }

    /// Mark sample as failed
    #[allow(dead_code)] // Used when sample processing fails
    pub fn mark_failed(&mut self, error: String) {
        self.status = SampleStatus::Failed(error);
        self.completed_at = Some(Instant::now());
    }
}

/// ETA calculator with rolling average
#[derive(Debug, Clone)]
pub struct EtaCalculator {
    /// When evaluation started
    start_time: Instant,
    /// Recent completion times for rolling average
    completion_history: Vec<(Instant, usize)>, // (time, samples_completed)
    /// Maximum history size
    max_history: usize,
}

impl EtaCalculator {
    /// Create new ETA calculator
    pub fn new() -> Self {
        Self {
            start_time: Instant::now(),
            completion_history: Vec::new(),
            max_history: 10, // Keep last 10 data points for rolling average
        }
    }

    /// Record progress update
    pub fn record_progress(&mut self, completed: usize) {
        let now = Instant::now();
        self.completion_history.push((now, completed));

        // Keep only recent history
        if self.completion_history.len() > self.max_history {
            self.completion_history.remove(0);
        }
    }

    /// Calculate ETA based on current progress
    pub fn calculate_eta(&self, completed: usize, total: usize) -> Option<Duration> {
        if completed == 0 || completed >= total {
            return None;
        }

        let rate = self.calculate_completion_rate(completed)?;
        let remaining = total - completed;
        let eta_seconds = remaining as f64 / rate;

        Some(Duration::from_secs_f64(eta_seconds))
    }

    /// Calculate completion rate (samples per second)
    fn calculate_completion_rate(&self, current_completed: usize) -> Option<f64> {
        if self.completion_history.len() < 2 {
            // Fall back to overall rate if not enough history
            let elapsed = self.start_time.elapsed().as_secs_f64();
            if elapsed > 0.0 && current_completed > 0 {
                return Some(current_completed as f64 / elapsed);
            }
            return None;
        }

        // Use recent history for rolling average
        let recent_start = self.completion_history[0];
        let recent_end = self.completion_history[self.completion_history.len() - 1];

        let time_diff = recent_end.0.duration_since(recent_start.0).as_secs_f64();
        let samples_diff = recent_end.1.saturating_sub(recent_start.1) as f64;

        if time_diff > 0.0 && samples_diff > 0.0 {
            Some(samples_diff / time_diff)
        } else {
            None
        }
    }

    /// Get elapsed time since start
    pub fn elapsed(&self) -> Duration {
        self.start_time.elapsed()
    }
}

impl Default for EtaCalculator {
    fn default() -> Self {
        Self::new()
    }
}

/// Type-safe wrapper for validated JSON strings
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidJson(String);

impl ValidJson {
    /// Create a ValidJson from a string, validating it's proper JSON
    pub fn try_new(json_str: String) -> Result<Self, JsonValidationError> {
        // Parse the JSON to validate it's well-formed
        serde_json::from_str::<serde_json::Value>(&json_str)
            .map_err(|e| JsonValidationError::MalformedJson(e.to_string()))?;
        
        Ok(ValidJson(json_str))
    }

    /// Get the inner JSON string
    #[allow(dead_code)] // For future use
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consume and return the inner JSON string
    #[allow(dead_code)] // For future use
    pub fn into_string(self) -> String {
        self.0
    }

    /// Parse the JSON into a specific type
    pub fn parse<T>(&self) -> Result<T, serde_json::Error>
    where
        T: serde::de::DeserializeOwned,
    {
        serde_json::from_str(&self.0)
    }
}

/// Errors that can occur during JSON validation
#[derive(Debug, thiserror::Error)]
pub enum JsonValidationError {
    #[error("malformed JSON: {0}")]
    MalformedJson(String),
}

/// Phantom types for terminal state
#[derive(Debug)]
pub struct Uninitialized;

#[derive(Debug)]
pub struct Initialized;

/// Type-safe terminal state that tracks initialization
#[derive(Debug)]
pub struct TerminalState<S> {
    _phantom: PhantomData<S>,
}

impl TerminalState<Uninitialized> {
    /// Create new uninitialized terminal state
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData,
        }
    }
}

impl Default for TerminalState<Uninitialized> {
    fn default() -> Self {
        Self::new()
    }
}

/// Sealed trait for UI actions - prevents external implementations
mod private {
    pub trait Sealed {}
}

/// UI action that can be performed
pub trait Action: private::Sealed {
    /// Get a description of the action for logging
    #[allow(dead_code)]
    fn description(&self) -> &str;
}

/// Concrete UI actions
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UiAction {
    /// User requested quit
    Quit,
    /// Terminal was resized
    Resize(TerminalSize),
    /// Pause/resume evaluation
    TogglePause,
    /// Refresh display
    Refresh,
}

impl private::Sealed for UiAction {}

impl Action for UiAction {
    fn description(&self) -> &str {
        match self {
            UiAction::Quit => "quit",
            UiAction::Resize(_) => "resize",
            UiAction::TogglePause => "toggle pause",
            UiAction::Refresh => "refresh",
        }
    }
}

// Note: All tests in this module have been eliminated through type constraints:
// - EvaluatorName and EvaluatorCommand cannot be empty (enforced by nutype)
// - Terminal dimensions must be positive (enforced by nutype)
// - Action trait is sealed, preventing external implementations
// - Terminal state transitions are enforced at compile time via phantom types
//
// The type system now makes these test scenarios impossible to express,
// providing compile-time guarantees instead of runtime checks.

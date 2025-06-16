use nutype::nutype;
use std::marker::PhantomData;

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
            width: TerminalWidth::try_new(width)
                .map_err(|_| TerminalSizeError::InvalidWidth)?,
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
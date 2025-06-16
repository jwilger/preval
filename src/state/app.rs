use super::metrics::MetricData;
use super::types::{EvaluatorName, EvaluationStatus};

/// Central application state
#[derive(Debug)]
pub struct AppState {
    /// Name of the running evaluator
    evaluator_name: Option<EvaluatorName>,
    
    /// Current evaluation status
    status: EvaluationStatus,
    
    /// Collected metrics
    metrics: Vec<MetricData>,
    
    /// Whether evaluation is paused
    paused: bool,
    
    /// Track number of metrics received
    metrics_received: usize,
}

impl AppState {
    /// Create new app state
    pub fn new() -> Self {
        Self {
            evaluator_name: None,
            status: EvaluationStatus::Starting,
            metrics: Vec::new(),
            paused: false,
            metrics_received: 0,
        }
    }

    /// Set evaluator name - can only be set once
    pub fn set_evaluator_name(&mut self, name: EvaluatorName) -> Result<(), StateError> {
        if self.evaluator_name.is_some() {
            return Err(StateError::EvaluatorAlreadySet);
        }
        self.evaluator_name = Some(name);
        Ok(())
    }

    /// Update status - ensures valid transitions
    pub fn update_status(&mut self, new_status: EvaluationStatus) -> Result<(), StateError> {
        // Validate state transitions
        match (&self.status, &new_status) {
            // Can't go back to Starting
            (_, EvaluationStatus::Starting) => {
                return Err(StateError::InvalidTransition);
            }
            // Can't leave Failed or Completed states
            (EvaluationStatus::Failed(_), _) | (EvaluationStatus::Completed, _) => {
                return Err(StateError::TerminalState);
            }
            // Valid transitions
            _ => {
                self.status = new_status;
                Ok(())
            }
        }
    }

    /// Add metrics - only allowed in CollectingMetrics state
    pub fn add_metrics(&mut self, metrics: MetricData) -> Result<(), StateError> {
        match &self.status {
            EvaluationStatus::CollectingMetrics { .. } => {
                self.metrics.push(metrics);
                self.metrics_received += 1;
                
                // Update status with new count
                if let EvaluationStatus::CollectingMetrics { total, .. } = &self.status {
                    self.status = EvaluationStatus::CollectingMetrics {
                        received: self.metrics_received,
                        total: *total,
                    };
                }
                Ok(())
            }
            _ => Err(StateError::NotCollectingMetrics),
        }
    }

    /// Toggle pause state
    pub fn toggle_pause(&mut self) {
        self.paused = !self.paused;
    }

    /// Check if we're in a terminal state
    pub fn is_terminal(&self) -> bool {
        matches!(
            self.status,
            EvaluationStatus::Completed | EvaluationStatus::Failed(_)
        )
    }
    
    /// Get evaluator name
    pub fn evaluator_name(&self) -> Option<&EvaluatorName> {
        self.evaluator_name.as_ref()
    }
    
    /// Get current status
    pub fn status(&self) -> &EvaluationStatus {
        &self.status
    }
    
    /// Get metrics
    pub fn metrics(&self) -> &[MetricData] {
        &self.metrics
    }
    
    /// Check if paused
    pub fn is_paused(&self) -> bool {
        self.paused
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}

/// State-related errors
#[derive(Debug, thiserror::Error)]
pub enum StateError {
    #[error("evaluator name already set")]
    EvaluatorAlreadySet,
    
    #[error("invalid state transition")]
    InvalidTransition,
    
    #[error("cannot transition from terminal state")]
    TerminalState,
    
    #[error("can only add metrics when collecting")]
    NotCollectingMetrics,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evaluator_name_can_only_be_set_once() {
        let mut state = AppState::new();
        let name1 = EvaluatorName::try_new("eval1").unwrap();
        let name2 = EvaluatorName::try_new("eval2").unwrap();
        
        assert!(state.set_evaluator_name(name1).is_ok());
        assert!(state.set_evaluator_name(name2).is_err());
    }

    #[test]
    fn test_cannot_transition_back_to_starting() {
        let mut state = AppState::new();
        state.update_status(EvaluationStatus::WaitingForHandshake).unwrap();
        
        let result = state.update_status(EvaluationStatus::Starting);
        assert!(result.is_err());
    }

    #[test]
    fn test_cannot_add_metrics_when_not_collecting() {
        let mut state = AppState::new();
        let metrics = MetricData {
            resource_attributes: Default::default(),
            metrics: vec![],
        };
        
        assert!(state.add_metrics(metrics).is_err());
    }

    // Note: Many invalid scenarios are now impossible due to type constraints:
    // - Cannot create empty evaluator names (enforced by EvaluatorName type)
    // - Cannot create invalid evaluation status (enforced by enum)
    // - State transitions are validated at runtime but could be further
    //   constrained with phantom types if needed
}
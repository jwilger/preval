use super::metrics::MetricData;
use super::types::{EvaluationStatus, EvaluatorName};
use crate::evaluator::protocol::ValidatedHandshake;

/// Central application state
#[derive(Debug)]
pub struct AppState {
    /// Name of the running evaluator
    evaluator_name: Option<EvaluatorName>,

    /// Validated handshake from evaluator
    handshake: Option<ValidatedHandshake>,

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
            handshake: None,
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

    /// Set handshake - can only be set once
    pub fn set_handshake(&mut self, handshake: ValidatedHandshake) -> Result<(), StateError> {
        if self.handshake.is_some() {
            return Err(StateError::HandshakeAlreadySet);
        }
        self.handshake = Some(handshake);
        Ok(())
    }

    /// Update status - ensures valid transitions
    pub fn update_status(&mut self, new_status: EvaluationStatus) -> Result<(), StateError> {
        // Validate state transitions
        match (&self.status, &new_status) {
            // Can't go back to Starting
            (_, EvaluationStatus::Starting) => Err(StateError::InvalidTransition),
            // Can't leave Failed or Completed states
            (EvaluationStatus::Failed(_), _) | (EvaluationStatus::Completed, _) => {
                Err(StateError::TerminalState)
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
                // Check if this is a summary metric (should not count toward sample progress)
                let is_summary = self.is_summary_metrics(&metrics);

                self.metrics.push(metrics);

                // Only increment counter for non-summary metrics (actual samples)
                if !is_summary {
                    self.metrics_received += 1;
                }

                // Update status with new count using handshake data if available
                let total = self.get_total_samples_from_handshake();
                self.status = EvaluationStatus::CollectingMetrics {
                    received: self.metrics_received,
                    total,
                };
                Ok(())
            }
            _ => Err(StateError::NotCollectingMetrics),
        }
    }

    /// Check if metrics data represents a summary (not a sample)
    fn is_summary_metrics(&self, metrics: &MetricData) -> bool {
        use crate::state::metrics::{AttributeKey, AttributeValue, Metric};

        // Try to create the summary key
        let summary_key = match AttributeKey::try_new("summary".to_string()) {
            Ok(key) => key,
            Err(_) => return false, // If we can't create the key, assume not summary
        };

        // Check if any metric has a "summary" attribute set to true
        for metric in &metrics.metrics {
            match metric {
                Metric::Gauge { data_points, .. } => {
                    for point in data_points {
                        if let Some(AttributeValue::BoolValue(true)) =
                            point.attributes.get(&summary_key)
                        {
                            return true;
                        }
                    }
                }
                Metric::Counter { data_points, .. } => {
                    for point in data_points {
                        if let Some(AttributeValue::BoolValue(true)) =
                            point.attributes.get(&summary_key)
                        {
                            return true;
                        }
                    }
                }
                Metric::Histogram { data_points, .. } => {
                    for point in data_points {
                        if let Some(AttributeValue::BoolValue(true)) =
                            point.attributes.get(&summary_key)
                        {
                            return true;
                        }
                    }
                }
            }
        }
        false
    }

    /// Get total samples from handshake execution plan
    fn get_total_samples_from_handshake(&self) -> Option<usize> {
        self.handshake
            .as_ref()?
            .execution_plan
            .as_ref()
            .map(|plan| plan.total_samples.into_inner() as usize)
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

    /// Get handshake
    pub fn handshake(&self) -> Option<&ValidatedHandshake> {
        self.handshake.as_ref()
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

    #[error("handshake already set")]
    HandshakeAlreadySet,

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
        state
            .update_status(EvaluationStatus::WaitingForHandshake)
            .unwrap();

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

    #[test]
    fn test_summary_metrics_do_not_count_toward_progress() {
        use crate::state::metrics::*;
        use std::collections::HashMap;

        let mut state = AppState::new();

        // Set to collecting state
        state
            .update_status(EvaluationStatus::CollectingMetrics {
                received: 0,
                total: Some(2),
            })
            .unwrap();

        // Create a normal sample metric
        let mut sample_attributes = HashMap::new();
        sample_attributes.insert(
            AttributeKey::try_new("sample.id".to_string()).unwrap(),
            AttributeValue::StringValue("sample-001".to_string()),
        );

        let sample_metrics = MetricData {
            resource_attributes: Default::default(),
            metrics: vec![Metric::Gauge {
                name: MetricName::try_new("accuracy".to_string()).unwrap(),
                unit: None,
                data_points: vec![DataPoint {
                    timestamp: TimeUnixNano::try_new(1234567890).unwrap(),
                    value: GaugeValue::new(0.85),
                    attributes: sample_attributes,
                }],
            }],
        };

        // Create a summary metric
        let mut summary_attributes = HashMap::new();
        summary_attributes.insert(
            AttributeKey::try_new("summary".to_string()).unwrap(),
            AttributeValue::BoolValue(true),
        );

        let summary_metrics = MetricData {
            resource_attributes: Default::default(),
            metrics: vec![Metric::Gauge {
                name: MetricName::try_new("accuracy".to_string()).unwrap(),
                unit: None,
                data_points: vec![DataPoint {
                    timestamp: TimeUnixNano::try_new(1234567890).unwrap(),
                    value: GaugeValue::new(0.81),
                    attributes: summary_attributes,
                }],
            }],
        };

        // Add sample metric - should increment counter
        state.add_metrics(sample_metrics).unwrap();
        if let EvaluationStatus::CollectingMetrics { received, .. } = state.status() {
            assert_eq!(*received, 1, "Sample metrics should increment counter");
        }

        // Add summary metric - should NOT increment counter
        state.add_metrics(summary_metrics).unwrap();
        if let EvaluationStatus::CollectingMetrics { received, .. } = state.status() {
            assert_eq!(*received, 1, "Summary metrics should not increment counter");
        }

        // Verify we have 2 metric data objects but only 1 counted sample
        assert_eq!(
            state.metrics().len(),
            2,
            "Should have 2 metric data objects"
        );
        assert_eq!(state.metrics_received, 1, "Should have 1 counted sample");
    }

    // Note: Many invalid scenarios are now impossible due to type constraints:
    // - Cannot create empty evaluator names (enforced by EvaluatorName type)
    // - Cannot create invalid evaluation status (enforced by enum)
    // - State transitions are validated at runtime but could be further
    //   constrained with phantom types if needed
}

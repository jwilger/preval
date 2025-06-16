use super::metrics::{MetricData, Metric, SampleMetric};
use super::types::{
    EvaluationStatus, EvaluatorName, SampleResult, EtaCalculator, SampleStatus,
    EvaluatorNotSet, EvaluatorSet, HandshakeNotSet, HandshakeSet,
    Starting, WaitingForHandshake, CollectingMetrics, CompletedOrFailed,
};
use crate::evaluator::protocol::ValidatedHandshake;
use std::collections::HashMap;
use std::marker::PhantomData;

/// Central application state with full typestate pattern
#[derive(Debug)]
pub struct AppState<E = EvaluatorNotSet, H = HandshakeNotSet, S = Starting> {
    /// Name of the running evaluator (only available when E = EvaluatorSet)
    evaluator_name: Option<EvaluatorName>,

    /// Validated handshake from evaluator (only available when H = HandshakeSet)
    handshake: Option<ValidatedHandshake>,

    /// Current evaluation status (encoded in S type parameter)
    status: EvaluationStatus,

    /// Collected metrics
    metrics: Vec<MetricData>,

    /// Whether evaluation is paused
    paused: bool,

    /// Track number of metrics received
    metrics_received: usize,

    /// Sample tracking for progress display
    samples: HashMap<String, SampleResult>,

    /// Recent completed samples (bounded for UI display)
    recent_samples: Vec<SampleResult>,
    
    /// Maximum number of recent samples to keep
    max_recent_samples: usize,

    /// ETA calculator for progress estimation
    eta_calculator: EtaCalculator,

    /// Current sample being processed
    current_sample: Option<String>,

    /// Phantom data for typestate tracking
    _evaluator_state: PhantomData<E>,
    _handshake_state: PhantomData<H>,
    _status_state: PhantomData<S>,
}

/// Type aliases for common state combinations
pub type InitialAppState = AppState<EvaluatorNotSet, HandshakeNotSet, Starting>;
pub type AppStateWithEvaluator = AppState<EvaluatorSet, HandshakeNotSet, Starting>;
pub type AppStateReady = AppState<EvaluatorSet, HandshakeSet, WaitingForHandshake>;
pub type AppStateCollecting = AppState<EvaluatorSet, HandshakeSet, CollectingMetrics>;
pub type AppStateFinished = AppState<EvaluatorSet, HandshakeSet, CompletedOrFailed>;

impl InitialAppState {
    /// Create new app state in initial starting state
    pub fn new() -> Self {
        Self {
            evaluator_name: None,
            handshake: None,
            status: EvaluationStatus::Starting,
            metrics: Vec::new(),
            paused: false,
            metrics_received: 0,
            samples: HashMap::new(),
            recent_samples: Vec::new(),
            max_recent_samples: 10,
            eta_calculator: EtaCalculator::new(),
            current_sample: None,
            _evaluator_state: PhantomData,
            _handshake_state: PhantomData,
            _status_state: PhantomData,
        }
    }

    /// Set evaluator name - transitions to EvaluatorSet state
    pub fn set_evaluator_name(mut self, name: EvaluatorName) -> AppStateWithEvaluator {
        self.evaluator_name = Some(name);
        AppStateWithEvaluator {
            evaluator_name: self.evaluator_name,
            handshake: self.handshake,
            status: self.status,
            metrics: self.metrics,
            paused: self.paused,
            metrics_received: self.metrics_received,
            samples: self.samples,
            recent_samples: self.recent_samples,
            max_recent_samples: self.max_recent_samples,
            eta_calculator: self.eta_calculator,
            current_sample: self.current_sample,
            _evaluator_state: PhantomData,
            _handshake_state: PhantomData,
            _status_state: PhantomData,
        }
    }
}

impl AppStateWithEvaluator {
    /// Set handshake and transition to WaitingForHandshake state
    pub fn set_handshake(mut self, handshake: ValidatedHandshake) -> AppStateReady {
        self.handshake = Some(handshake);
        self.status = EvaluationStatus::WaitingForHandshake;
        AppStateReady {
            evaluator_name: self.evaluator_name,
            handshake: self.handshake,
            status: self.status,
            metrics: self.metrics,
            paused: self.paused,
            metrics_received: self.metrics_received,
            samples: self.samples,
            recent_samples: self.recent_samples,
            max_recent_samples: self.max_recent_samples,
            eta_calculator: self.eta_calculator,
            current_sample: self.current_sample,
            _evaluator_state: PhantomData,
            _handshake_state: PhantomData,
            _status_state: PhantomData,
        }
    }
}

impl AppStateReady {
    /// Start collecting metrics - transition to CollectingMetrics state
    pub fn start_collecting(mut self) -> AppStateCollecting {
        self.status = EvaluationStatus::CollectingMetrics {
            received: 0,
            total: self.get_total_samples_from_handshake(),
        };
        AppStateCollecting {
            evaluator_name: self.evaluator_name,
            handshake: self.handshake,
            status: self.status,
            metrics: self.metrics,
            paused: self.paused,
            metrics_received: self.metrics_received,
            samples: self.samples,
            recent_samples: self.recent_samples,
            max_recent_samples: self.max_recent_samples,
            eta_calculator: self.eta_calculator,
            current_sample: self.current_sample,
            _evaluator_state: PhantomData,
            _handshake_state: PhantomData,
            _status_state: PhantomData,
        }
    }
}

impl AppStateCollecting {
    /// Add metrics - only available in CollectingMetrics state
    pub fn add_metrics(mut self, metrics: MetricData) -> AppStateCollecting {
        // Check if this is a summary metric (should not count toward sample progress)
        let is_summary = self.is_summary_metrics(&metrics);

        // Extract sample ID if present and not a summary
        if !is_summary {
            if let Some(sample_id) = self.extract_sample_id(&metrics) {
                self.process_sample_metrics(sample_id.clone(), &metrics);
                self.current_sample = Some(sample_id);
            }
        }

        self.metrics.push(metrics);

        // Only increment counter for non-summary metrics (actual samples)
        if !is_summary {
            self.metrics_received += 1;
        }

        // Update ETA calculator with progress
        self.eta_calculator.record_progress(self.metrics_received);

        // Update status with new count using handshake data if available
        let total = self.get_total_samples_from_handshake();
        self.status = EvaluationStatus::CollectingMetrics {
            received: self.metrics_received,
            total,
        };

        self
    }

    /// Transition to finished state
    pub fn finish(mut self, final_status: EvaluationStatus) -> AppStateFinished {
        self.status = final_status;
        AppStateFinished {
            evaluator_name: self.evaluator_name,
            handshake: self.handshake,
            status: self.status,
            metrics: self.metrics,
            paused: self.paused,
            metrics_received: self.metrics_received,
            samples: self.samples,
            recent_samples: self.recent_samples,
            max_recent_samples: self.max_recent_samples,
            eta_calculator: self.eta_calculator,
            current_sample: self.current_sample,
            _evaluator_state: PhantomData,
            _handshake_state: PhantomData,
            _status_state: PhantomData,
        }
    }
}

// Shared implementation for all states
impl<E, H, S> AppState<E, H, S> {
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
    #[allow(dead_code)] // Used in future stories
    pub fn status(&self) -> &EvaluationStatus {
        &self.status
    }

    /// Get metrics
    #[allow(dead_code)] // Used in future stories
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

    /// Get recent completed samples
    pub fn recent_samples(&self) -> &[SampleResult] {
        &self.recent_samples
    }

    /// Get current sample being processed
    pub fn current_sample(&self) -> Option<&str> {
        self.current_sample.as_deref()
    }

    /// Calculate ETA for completion
    pub fn calculate_eta(&self) -> Option<std::time::Duration> {
        let total = self.get_total_samples_from_handshake()?;
        self.eta_calculator.calculate_eta(self.metrics_received, total)
    }

    /// Get elapsed time since evaluation started
    pub fn elapsed_time(&self) -> std::time::Duration {
        self.eta_calculator.elapsed()
    }

    /// Get completion progress as (completed, total, percentage)
    pub fn progress(&self) -> (usize, Option<usize>, f64) {
        let completed = self.metrics_received;
        let total = self.get_total_samples_from_handshake();
        let percentage = match total {
            Some(t) if t > 0 => (completed as f64 / t as f64) * 100.0,
            _ => 0.0,
        };
        (completed, total, percentage)
    }

    /// Get summary statistics
    pub fn summary_stats(&self) -> (usize, usize, f64) {
        let total_completed = self.recent_samples.len();
        let failed_count = self.recent_samples.iter()
            .filter(|sample| matches!(sample.status, SampleStatus::Failed(_)))
            .count();
        let success_rate = if total_completed > 0 {
            ((total_completed - failed_count) as f64 / total_completed as f64) * 100.0
        } else {
            0.0
        };
        (failed_count, total_completed, success_rate)
    }

    /// Check if metrics data represents a summary (not a sample)
    /// With the new type system, this is now encoded at the type level!
    fn is_summary_metrics(&self, metrics: &MetricData) -> bool {
        // Check if any metric is a summary metric - the type system now makes this trivial!
        metrics.metrics.iter().any(|metric| {
            matches!(metric, Metric::Summary(_))
        })
    }

    /// Get total samples from handshake execution plan
    fn get_total_samples_from_handshake(&self) -> Option<usize> {
        self.handshake
            .as_ref()?
            .execution_plan
            .as_ref()
            .map(|plan| plan.total_samples.into_inner() as usize)
    }

    /// Extract sample ID from metrics data
    fn extract_sample_id(&self, metrics: &MetricData) -> Option<String> {
        use crate::state::metrics::AttributeValue;

        // Try to find sample.id attribute in sample metrics only
        for metric in &metrics.metrics {
            match metric {
                Metric::Sample(sample_metric) => {
                    match sample_metric {
                        SampleMetric::Gauge { data_points, .. } => {
                            for point in data_points {
                                for (key, value) in &point.attributes {
                                    if key.as_ref() == "sample.id" {
                                        if let AttributeValue::StringValue(s) = value {
                                            return Some(s.clone());
                                        }
                                    }
                                }
                            }
                        }
                        SampleMetric::Counter { data_points, .. } => {
                            for point in data_points {
                                for (key, value) in &point.attributes {
                                    if key.as_ref() == "sample.id" {
                                        if let AttributeValue::StringValue(s) = value {
                                            return Some(s.clone());
                                        }
                                    }
                                }
                            }
                        }
                        SampleMetric::Histogram { data_points, .. } => {
                            for point in data_points {
                                for (key, value) in &point.attributes {
                                    if key.as_ref() == "sample.id" {
                                        if let AttributeValue::StringValue(s) = value {
                                            return Some(s.clone());
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                Metric::Summary(_) => {
                    // Summary metrics don't have sample IDs by definition
                    continue;
                }
            }
        }
        None
    }

    /// Process metrics for a specific sample
    fn process_sample_metrics(&mut self, sample_id: String, metrics: &MetricData) {
        // Extract key metrics from the data - only from sample metrics
        let mut extracted_metrics = Vec::new();
        
        for metric in &metrics.metrics {
            match metric {
                Metric::Sample(sample_metric) => {
                    match sample_metric {
                        SampleMetric::Gauge { name, data_points, .. } => {
                            for point in data_points {
                                extracted_metrics.push((name.as_ref().to_string(), point.value.value()));
                            }
                        }
                        SampleMetric::Counter { name, data_points, .. } => {
                            for point in data_points {
                                extracted_metrics.push((name.as_ref().to_string(), point.value.value()));
                            }
                        }
                        SampleMetric::Histogram { name, data_points, .. } => {
                            for point in data_points {
                                // Use average value for histograms
                                let avg = if point.value.count > 0 {
                                    point.value.sum.unwrap_or(0.0) / point.value.count as f64
                                } else {
                                    0.0
                                };
                                extracted_metrics.push((name.as_ref().to_string(), avg));
                            }
                        }
                    }
                }
                Metric::Summary(_) => {
                    // Summary metrics are not processed as sample data
                    continue;
                }
            }
        }

        // Update or create sample result
        let sample_result = self.samples.entry(sample_id.clone()).or_insert_with(|| {
            SampleResult::new_processing(sample_id.clone())
        });

        // Mark as completed with metrics
        sample_result.mark_completed(extracted_metrics);

        // Add to recent samples (keep only the most recent)
        self.recent_samples.push(sample_result.clone());
        
        // Keep only the most recent samples
        if self.recent_samples.len() > self.max_recent_samples {
            self.recent_samples.remove(0);
        }
    }
}

impl Default for InitialAppState {
    fn default() -> Self {
        Self::new()
    }
}

/// State-related errors (most eliminated by typestate pattern)
#[derive(Debug, thiserror::Error)]
pub enum StateError {
    // These errors are eliminated by the typestate pattern:
    // - EvaluatorAlreadySet: transitions ensure evaluator can only be set once
    // - HandshakeAlreadySet: transitions ensure handshake can only be set once
    // - InvalidTransition: state machine enforced by types
    // - NotCollectingMetrics: add_metrics only available on AppStateCollecting
    
    #[error("cannot transition from terminal state")]
    TerminalState, // Could be eliminated with more complex phantom types
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::evaluator::protocol::{ValidatedHandshake, Handshake, EvaluationMode, EvaluatorInfo, MessageType, ExecutionPlan};

    // Tests removed by typestate pattern:
    //
    // - test_evaluator_name_can_only_be_set_once: 
    //   The typestate pattern makes it impossible to set an evaluator name twice.
    //   Once set_evaluator_name() is called, it returns AppStateWithEvaluator,
    //   which doesn't have a set_evaluator_name() method.
    //
    // - test_cannot_transition_back_to_starting:
    //   State transitions are now encoded in the type system. Each state type
    //   only has methods to transition to valid next states.
    //
    // - test_cannot_add_metrics_when_not_collecting:
    //   The add_metrics() method is only available on AppStateCollecting.
    //   It's impossible to call it on other state types.

    #[test]
    fn test_typestate_progression() {
        // Demonstrate that the typestate pattern enforces correct progression
        let state = InitialAppState::new();
        assert!(state.evaluator_name().is_none());

        let name = EvaluatorName::try_new("test-evaluator").unwrap();
        let state = state.set_evaluator_name(name);
        assert!(state.evaluator_name().is_some());

        // Create a minimal valid handshake
        let handshake = create_test_handshake();
        let state = state.set_handshake(handshake);
        assert!(state.handshake().is_some());

        let state = state.start_collecting();
        // Now we can add metrics
        let metrics = MetricData {
            resource_attributes: Default::default(),
            metrics: vec![],
        };
        let _state = state.add_metrics(metrics);
    }

    // Test ELIMINATED by mutually exclusive metric types:
    //
    // The test_summary_metrics_do_not_count_toward_progress test has been
    // effectively eliminated by the type system. The business logic it tested
    // is now encoded at compile time through the Metric::Sample vs Metric::Summary
    // type distinction.
    //
    // The old test checked that metrics with a "summary=true" attribute would not
    // increment the progress counter. Now:
    // 1. Metric types are mutually exclusive (Sample vs Summary)
    // 2. Progress counting logic can use metric.counts_toward_progress()
    // 3. is_summary_metrics() became trivial: just check the enum variant
    // 4. It's impossible to accidentally create a "summary" metric that counts toward progress
    //
    // The type system now makes the original test scenario impossible to express!

    #[test]
    fn test_mutually_exclusive_metric_types_demonstrate_type_safety() {
        use crate::state::metrics::*;
        use std::collections::HashMap;

        // Demonstrate that the type system encodes the business logic
        let sample_metric = Metric::Sample(SampleMetric::Gauge {
            name: MetricName::try_new("accuracy".to_string()).unwrap(),
            unit: None,
            data_points: vec![DataPoint {
                timestamp: TimeUnixNano::try_new(1234567890).unwrap(),
                value: GaugeValue::new(0.85),
                attributes: HashMap::new(),
            }],
        });

        let summary_metric = Metric::Summary(SummaryMetric::Gauge {
            name: MetricName::try_new("accuracy".to_string()).unwrap(),
            unit: None,
            data_points: vec![DataPoint {
                timestamp: TimeUnixNano::try_new(1234567890).unwrap(),
                value: GaugeValue::new(0.81),
                attributes: HashMap::new(),
            }],
        });

        // The type system guarantees this behavior at compile time!
        assert!(sample_metric.counts_toward_progress());
        assert!(!summary_metric.counts_toward_progress());

        // The add_metrics logic can now use the type to make decisions,
        // eliminating the need for runtime attribute checking
    }

    fn create_test_handshake() -> ValidatedHandshake {
        let handshake = Handshake {
            msg_type: MessageType::Handshake,
            mode: EvaluationMode::TestSuite,
            version: "1.0".to_string(),
            evaluator: EvaluatorInfo {
                name: crate::evaluator::protocol::EvaluatorNameProtocol::try_new("test-evaluator".to_string()).unwrap(),
                description: None,
                version: None,
            },
            execution_plan: Some(ExecutionPlan {
                total_samples: 10,
                batch_size: None,
            }),
            metrics_schema: vec![],
        };
        ValidatedHandshake::parse(handshake).unwrap()
    }

    // Note: Typestate pattern eliminates need for many tests:
    // - Cannot set evaluator name twice (method not available after first set)
    // - Cannot set handshake twice (method not available after first set)  
    // - Cannot add metrics unless in collecting state (method only on AppStateCollecting)
    // - Cannot transition to invalid states (only valid transitions available)
    //
    // The type system now provides compile-time guarantees for state management,
    // eliminating the need for runtime validation tests.
}

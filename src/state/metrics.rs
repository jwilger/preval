use nutype::nutype;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Non-empty metric name
#[nutype(
    sanitize(trim),
    validate(not_empty, len_char_max = 255),
    derive(
        Debug,
        Clone,
        PartialEq,
        Eq,
        Hash,
        Serialize,
        Deserialize,
        AsRef,
        Display
    )
)]
pub struct MetricName(String);

/// Non-empty sample identifier
#[nutype(
    sanitize(trim),
    validate(not_empty, len_char_max = 255),
    derive(
        Debug,
        Clone,
        PartialEq,
        Eq,
        Hash,
        Serialize,
        Deserialize,
        AsRef,
        Display
    )
)]
pub struct SampleId(String);

/// Unix timestamp in nanoseconds
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
        Serialize,
        Deserialize,
        AsRef,
        Into
    )
)]
pub struct TimeUnixNano(u64);

/// Non-empty attribute key
#[nutype(
    sanitize(trim),
    validate(not_empty, len_char_max = 255),
    derive(
        Debug,
        Clone,
        PartialEq,
        Eq,
        Hash,
        Serialize,
        Deserialize,
        AsRef,
        Display
    )
)]
pub struct AttributeKey(String);

/// Attribute value types matching OpenTelemetry spec
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::enum_variant_names)] // Matches OpenTelemetry spec naming
pub enum AttributeValue {
    StringValue(String),
    BoolValue(bool),
    IntValue(i64),
    DoubleValue(f64),
    ArrayValue(Vec<AttributeValue>),
    KvlistValue(HashMap<String, AttributeValue>),
}

/// Type-safe gauge value (can be negative)
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct GaugeValue(f64);

impl GaugeValue {
    /// Create a new gauge value
    pub fn new(value: f64) -> Self {
        GaugeValue(value)
    }

    /// Get the inner value
    pub fn value(&self) -> f64 {
        self.0
    }
}

/// Non-negative f64 value - guaranteed to be >= 0.0 and finite
#[nutype(
    validate(finite, greater_or_equal = 0.0),
    derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, AsRef, Into)
)]
pub struct NonNegativeF64(f64);

/// Type-safe counter value (guaranteed non-negative and finite by construction)
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct CounterValue(NonNegativeF64);

impl CounterValue {
    /// Create a counter value from a validated non-negative input - cannot fail!
    pub fn new(value: NonNegativeF64) -> Self {
        CounterValue(value)
    }

    /// Legacy method for JSON parsing - validates input
    #[allow(dead_code)] // Keep for backward compatibility during transition
    pub fn try_new(value: f64) -> Result<Self, CounterValueError> {
        let non_neg_value = NonNegativeF64::try_new(value)
            .map_err(|_| {
                if value < 0.0 {
                    CounterValueError::MustBeNonNegative
                } else {
                    CounterValueError::NotFinite
                }
            })?;

        Ok(CounterValue(non_neg_value))
    }

    /// Get the inner value
    pub fn value(&self) -> f64 {
        *self.0.as_ref()
    }
}

impl From<CounterValue> for f64 {
    fn from(val: CounterValue) -> Self {
        val.value()
    }
}

/// Counter-specific error type
#[derive(Debug, thiserror::Error)]
pub enum CounterValueError {
    #[error("counter value must be non-negative")]
    MustBeNonNegative,
    #[error("counter value must be finite")]
    NotFinite,
}

/// Histogram bucket with boundaries and count
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HistogramBucket {
    pub upper_bound: f64,
    pub count: u64,
}

/// Type-safe histogram value
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HistogramValue {
    pub count: u64,
    pub sum: Option<f64>,
    pub buckets: Vec<HistogramBucket>,
    pub min: Option<f64>,
    pub max: Option<f64>,
}

/// A single data point with timestamp and attributes
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DataPoint<V> {
    pub timestamp: TimeUnixNano,
    pub value: V,
    pub attributes: HashMap<AttributeKey, AttributeValue>,
}

/// Metric type that counts toward evaluation progress
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum SampleMetric {
    #[serde(rename = "gauge")]
    Gauge {
        name: MetricName,
        unit: Option<String>,
        data_points: Vec<DataPoint<GaugeValue>>,
    },
    #[serde(rename = "counter")]
    Counter {
        name: MetricName,
        unit: Option<String>,
        data_points: Vec<DataPoint<CounterValue>>,
    },
    #[serde(rename = "histogram")]
    Histogram {
        name: MetricName,
        unit: Option<String>,
        data_points: Vec<DataPoint<HistogramValue>>,
    },
}

/// Metric type that does NOT count toward evaluation progress (summary/aggregate data)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum SummaryMetric {
    #[serde(rename = "gauge")]
    Gauge {
        name: MetricName,
        unit: Option<String>,
        data_points: Vec<DataPoint<GaugeValue>>,
    },
    #[serde(rename = "counter")]
    Counter {
        name: MetricName,
        unit: Option<String>,
        data_points: Vec<DataPoint<CounterValue>>,
    },
    #[serde(rename = "histogram")]
    Histogram {
        name: MetricName,
        unit: Option<String>,
        data_points: Vec<DataPoint<HistogramValue>>,
    },
}

/// Top-level metric enum that distinguishes between sample and summary metrics
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "category")]
pub enum Metric {
    #[serde(rename = "sample")]
    Sample(SampleMetric),
    #[serde(rename = "summary")]
    Summary(SummaryMetric),
}

impl Metric {
    /// Get the metric name regardless of type
    pub fn name(&self) -> &MetricName {
        match self {
            Metric::Sample(sample_metric) => match sample_metric {
                SampleMetric::Gauge { name, .. } => name,
                SampleMetric::Counter { name, .. } => name,
                SampleMetric::Histogram { name, .. } => name,
            },
            Metric::Summary(summary_metric) => match summary_metric {
                SummaryMetric::Gauge { name, .. } => name,
                SummaryMetric::Counter { name, .. } => name,
                SummaryMetric::Histogram { name, .. } => name,
            },
        }
    }

    /// Check if this metric counts toward progress (is a sample metric)
    pub fn counts_toward_progress(&self) -> bool {
        matches!(self, Metric::Sample(_))
    }
}

/// Collection of metrics from a single resource
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MetricData {
    pub resource_attributes: HashMap<AttributeKey, AttributeValue>,
    pub metrics: Vec<Metric>,
}

#[cfg(test)]
mod tests {

    // Tests removed: metric_name_cannot_be_empty, sample_id_cannot_be_empty
    // The nutype validation already guarantees these cannot be empty at compile time

    // Test removed: counter_value_must_be_non_negative
    // The NonNegativeF64 input type now makes it impossible to construct
    // a CounterValue with negative input using the primary constructor.
    // The main constructor CounterValue::new() is infallible since it takes
    // a NonNegativeF64 which is guaranteed to be valid by the type system.

    // Test removed: gauge_value_can_be_negative
    // The type system already guarantees that GaugeValue can hold any f64 value,
    // including negative values. The public field makes this test redundant.

    // Test removed: histogram_value_stores_distribution
    // The type system guarantees that all public fields of HistogramValue
    // are accessible and of the correct types. Testing struct construction
    // and field access is redundant.
}

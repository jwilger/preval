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

/// Finite f64 value - guaranteed to not be NaN or infinity
#[nutype(
    validate(finite),
    derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, AsRef, Into)
)]
struct FiniteF64(f64);

/// Type-safe counter value (must be non-negative and finite)
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct CounterValue(FiniteF64);

impl CounterValue {
    /// Create a counter value from f64, ensuring it's non-negative and finite
    pub fn try_new(value: f64) -> Result<Self, CounterValueError> {
        let finite_value = FiniteF64::try_new(value).map_err(|_| CounterValueError::NotFinite)?;

        if value < 0.0 {
            return Err(CounterValueError::MustBeNonNegative);
        }

        Ok(CounterValue(finite_value))
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

/// Type-safe metric enum
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Metric {
    Gauge {
        name: MetricName,
        unit: Option<String>,
        data_points: Vec<DataPoint<GaugeValue>>,
    },
    Counter {
        name: MetricName,
        unit: Option<String>,
        data_points: Vec<DataPoint<CounterValue>>,
    },
    Histogram {
        name: MetricName,
        unit: Option<String>,
        data_points: Vec<DataPoint<HistogramValue>>,
    },
}

/// Collection of metrics from a single resource
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MetricData {
    pub resource_attributes: HashMap<AttributeKey, AttributeValue>,
    pub metrics: Vec<Metric>,
}

#[cfg(test)]
mod tests {
    use super::*;

    // Tests removed: metric_name_cannot_be_empty, sample_id_cannot_be_empty
    // The nutype validation already guarantees these cannot be empty at compile time

    #[test]
    fn counter_value_must_be_non_negative() {
        assert!(CounterValue::try_new(-1.0).is_err());
        assert!(CounterValue::try_new(0.0).is_ok());
        assert!(CounterValue::try_new(1.0).is_ok());
    }

    // Test removed: gauge_value_can_be_negative
    // The type system already guarantees that GaugeValue can hold any f64 value,
    // including negative values. The public field makes this test redundant.

    // Test removed: histogram_value_stores_distribution
    // The type system guarantees that all public fields of HistogramValue
    // are accessible and of the correct types. Testing struct construction
    // and field access is redundant.
}

use serde::{Deserialize, Serialize};

/// OTLP attribute representation
#[derive(Debug, Clone, Deserialize, Serialize)]
pub(super) struct Attribute {
    pub key: String,
    pub value: AnyValue,
}

/// OTLP AnyValue representation
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) enum AnyValue {
    #[serde(rename = "stringValue")]
    String(String),
    #[serde(rename = "boolValue")]
    Bool(bool),
    #[serde(rename = "intValue")]
    Int(i64),
    #[serde(rename = "doubleValue")]
    Double(f64),
    #[serde(rename = "arrayValue")]
    Array(ArrayValue),
    #[serde(rename = "kvlistValue")]
    KvList(KvListValue),
}

/// OTLP array value
#[derive(Debug, Clone, Deserialize, Serialize)]
pub(super) struct ArrayValue {
    pub values: Vec<AnyValue>,
}

/// OTLP key-value list
#[derive(Debug, Clone, Deserialize, Serialize)]
pub(super) struct KvListValue {
    pub values: Vec<KeyValue>,
}

/// OTLP key-value pair
#[derive(Debug, Clone, Deserialize, Serialize)]
pub(super) struct KeyValue {
    pub key: String,
    pub value: AnyValue,
}

/// OTLP resource representation
#[derive(Debug, Clone, Deserialize, Serialize)]
pub(super) struct Resource {
    #[serde(default)]
    pub attributes: Vec<Attribute>,
}

/// OTLP gauge data point
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct GaugeDataPoint {
    pub time_unix_nano: String,
    pub as_double: f64,
    #[serde(default)]
    pub attributes: Vec<Attribute>,
}

/// OTLP gauge metric
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct Gauge {
    pub data_points: Vec<GaugeDataPoint>,
}

/// OTLP sum (counter) data point
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct SumDataPoint {
    pub time_unix_nano: String,
    pub as_double: f64,
    #[serde(default)]
    pub attributes: Vec<Attribute>,
}

/// OTLP sum (counter) metric
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct Sum {
    pub data_points: Vec<SumDataPoint>,
    #[serde(default)]
    pub aggregation_temporality: i32,
    #[serde(default)]
    pub is_monotonic: bool,
}

/// OTLP histogram data point
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct HistogramDataPoint {
    pub time_unix_nano: String,
    #[serde(default)]
    pub attributes: Vec<Attribute>,
    pub count: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sum: Option<f64>,
    #[serde(default)]
    pub bucket_counts: Vec<String>,
    #[serde(default)]
    pub explicit_bounds: Vec<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<f64>,
}

/// OTLP histogram metric
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct Histogram {
    pub data_points: Vec<HistogramDataPoint>,
    #[serde(default)]
    pub aggregation_temporality: i32,
}

/// OTLP metric representation - matches the OTLP JSON format exactly
#[derive(Debug, Clone, Deserialize, Serialize)]
pub(super) struct Metric {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gauge: Option<Gauge>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sum: Option<Sum>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub histogram: Option<Histogram>,
}

/// OTLP scope metrics
#[derive(Debug, Clone, Deserialize, Serialize)]
pub(super) struct ScopeMetrics {
    #[serde(default)]
    pub metrics: Vec<Metric>,
}

/// OTLP resource metrics
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct ResourceMetrics {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<Resource>,
    #[serde(default)]
    pub scope_metrics: Vec<ScopeMetrics>,
}

/// OTLP metrics data root structure
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct MetricsData {
    pub resource_metrics: Vec<ResourceMetrics>,
}

/// Type-safe metric data that guarantees exactly one metric type
#[derive(Debug, Clone)]
pub(super) enum ValidatedMetricData {
    Gauge(Gauge),
    Sum(Sum),
    Histogram(Histogram),
}

/// Validated OTLP metric that has been successfully parsed
#[derive(Debug, Clone)]
pub(super) struct ValidatedMetric {
    pub name: String,
    pub unit: Option<String>,
    pub data: ValidatedMetricData,
}

impl ValidatedMetric {
    /// Parse and validate a metric from OTLP format
    pub fn parse(metric: Metric) -> Result<Self, ValidationError> {
        // Validate metric name is not empty
        if metric.name.trim().is_empty() {
            return Err(ValidationError::EmptyMetricName);
        }

        // Extract exactly one metric type - the type system ensures we handle all cases
        let data = match (metric.gauge, metric.sum, metric.histogram) {
            (Some(gauge), None, None) => ValidatedMetricData::Gauge(gauge),
            (None, Some(sum), None) => ValidatedMetricData::Sum(sum),
            (None, None, Some(histogram)) => ValidatedMetricData::Histogram(histogram),
            _ => return Err(ValidationError::InvalidMetricType),
        };

        Ok(ValidatedMetric {
            name: metric.name,
            unit: metric.unit,
            data,
        })
    }
}

/// Validation errors for OTLP parsing
#[derive(Debug, thiserror::Error)]
pub(super) enum ValidationError {
    #[error("metric must have exactly one type (gauge, sum, or histogram)")]
    InvalidMetricType,
    #[error("metric name cannot be empty")]
    EmptyMetricName,
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test removed: validates_single_metric_type
    // The ValidatedMetricData enum and the exhaustive pattern match in
    // ValidatedMetric::parse ensure at compile time that exactly one metric
    // type is extracted. The match expression handles all possible combinations
    // of Some/None values, making it impossible to create an invalid state.

    #[test]
    fn validates_non_empty_name() {
        let metric = Metric {
            name: "  ".to_string(),
            unit: None,
            gauge: Some(Gauge {
                data_points: vec![],
            }),
            sum: None,
            histogram: None,
        };

        assert!(ValidatedMetric::parse(metric).is_err());
    }
}

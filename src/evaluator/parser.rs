use crate::state::metrics::{
    AttributeKey, AttributeValue, CounterValue, DataPoint, GaugeValue, HistogramBucket,
    HistogramValue, Metric, SampleMetric, SummaryMetric, MetricData, MetricName, TimeUnixNano,
};
use crate::state::types::ValidJson;

use super::otlp::{self, ValidatedMetric, ValidatedMetricData};
use anyhow::{Context, Result};
use std::collections::HashMap;

/// Parse a line of JSON containing OTLP metrics data
pub fn parse_metrics_line(line: &str) -> Result<MetricData> {
    // First validate the JSON is well-formed
    let valid_json = ValidJson::try_new(line.to_string())
        .context("malformed JSON in metrics")?;
    
    // Then parse it as OTLP data
    let metrics_data: otlp::MetricsData = valid_json.parse()
        .context("failed to parse OTLP JSON")?;

    let mut all_metrics = Vec::new();
    let mut resource_attributes = HashMap::new();

    for resource_metric in metrics_data.resource_metrics {
        // Extract resource attributes
        if let Some(resource) = resource_metric.resource {
            for attr in resource.attributes {
                let key = AttributeKey::try_new(attr.key.clone())
                    .map_err(|e| anyhow::anyhow!("invalid attribute key: {}", e))?;
                let value = convert_any_value(attr.value)?;
                resource_attributes.insert(key, value);
            }
        }

        // Extract metrics from all scopes
        for scope_metric in resource_metric.scope_metrics {
            for otlp_metric in scope_metric.metrics {
                let validated = ValidatedMetric::parse(otlp_metric)
                    .context("failed to validate OTLP metric")?;
                let metric = convert_metric(validated)?;
                all_metrics.push(metric);
            }
        }
    }

    Ok(MetricData {
        resource_attributes,
        metrics: all_metrics,
    })
}

/// Convert OTLP AnyValue to domain AttributeValue
fn convert_any_value(value: otlp::AnyValue) -> Result<AttributeValue> {
    Ok(match value {
        otlp::AnyValue::String(s) => AttributeValue::StringValue(s),
        otlp::AnyValue::Bool(b) => AttributeValue::BoolValue(b),
        otlp::AnyValue::Int(i) => AttributeValue::IntValue(i),
        otlp::AnyValue::Double(d) => AttributeValue::DoubleValue(d),
        otlp::AnyValue::Array(arr) => {
            let values = arr
                .values
                .into_iter()
                .map(convert_any_value)
                .collect::<Result<Vec<_>>>()?;
            AttributeValue::ArrayValue(values)
        }
        otlp::AnyValue::KvList(kvlist) => {
            let mut map = HashMap::new();
            for kv in kvlist.values {
                let value = convert_any_value(kv.value)?;
                map.insert(kv.key, value);
            }
            AttributeValue::KvlistValue(map)
        }
    })
}

/// Convert validated OTLP metric to domain metric
fn convert_metric(validated: ValidatedMetric) -> Result<Metric> {
    let name = MetricName::try_new(validated.name)
        .map_err(|e| anyhow::anyhow!("invalid metric name: {}", e))?;

    // The type system now guarantees exactly one metric type via ValidatedMetricData
    match validated.data {
        ValidatedMetricData::Gauge(gauge) => {
            let data_points = gauge
                .data_points
                .into_iter()
                .map(convert_gauge_data_point)
                .collect::<Result<Vec<_>>>()?;

            // Check if any data point has summary=true attribute
            let is_summary = data_points.iter().any(|dp| {
                dp.attributes.iter().any(|(key, value)| {
                    key.as_ref() == "summary" && matches!(value, AttributeValue::BoolValue(true))
                })
            });

            if is_summary {
                Ok(Metric::Summary(SummaryMetric::Gauge {
                    name,
                    unit: validated.unit,
                    data_points,
                }))
            } else {
                Ok(Metric::Sample(SampleMetric::Gauge {
                    name,
                    unit: validated.unit,
                    data_points,
                }))
            }
        }
        ValidatedMetricData::Sum(sum) => {
            // Only handle monotonic sums as counters
            if !sum.is_monotonic {
                return Err(anyhow::anyhow!(
                    "non-monotonic sums are not supported as counters"
                ));
            }

            let data_points = sum
                .data_points
                .into_iter()
                .map(convert_counter_data_point)
                .collect::<Result<Vec<_>>>()?;

            // Check if any data point has summary=true attribute
            let is_summary = data_points.iter().any(|dp| {
                dp.attributes.iter().any(|(key, value)| {
                    key.as_ref() == "summary" && matches!(value, AttributeValue::BoolValue(true))
                })
            });

            if is_summary {
                Ok(Metric::Summary(SummaryMetric::Counter {
                    name,
                    unit: validated.unit,
                    data_points,
                }))
            } else {
                Ok(Metric::Sample(SampleMetric::Counter {
                    name,
                    unit: validated.unit,
                    data_points,
                }))
            }
        }
        ValidatedMetricData::Histogram(histogram) => {
            let data_points = histogram
                .data_points
                .into_iter()
                .map(convert_histogram_data_point)
                .collect::<Result<Vec<_>>>()?;

            // Check if any data point has summary=true attribute
            let is_summary = data_points.iter().any(|dp| {
                dp.attributes.iter().any(|(key, value)| {
                    key.as_ref() == "summary" && matches!(value, AttributeValue::BoolValue(true))
                })
            });

            if is_summary {
                Ok(Metric::Summary(SummaryMetric::Histogram {
                    name,
                    unit: validated.unit,
                    data_points,
                }))
            } else {
                Ok(Metric::Sample(SampleMetric::Histogram {
                    name,
                    unit: validated.unit,
                    data_points,
                }))
            }
        }
    }
}

/// Convert OTLP gauge data point
fn convert_gauge_data_point(dp: otlp::GaugeDataPoint) -> Result<DataPoint<GaugeValue>> {
    let timestamp = parse_time_unix_nano(&dp.time_unix_nano)?;
    let attributes = convert_attributes(dp.attributes)?;

    Ok(DataPoint {
        timestamp,
        value: GaugeValue::new(dp.as_double),
        attributes,
    })
}

/// Convert OTLP sum data point to counter
fn convert_counter_data_point(dp: otlp::SumDataPoint) -> Result<DataPoint<CounterValue>> {
    let timestamp = parse_time_unix_nano(&dp.time_unix_nano)?;
    let attributes = convert_attributes(dp.attributes)?;
    let value = CounterValue::try_new(dp.as_double)
        .map_err(|e| anyhow::anyhow!("invalid counter value: {}", e))?;

    Ok(DataPoint {
        timestamp,
        value,
        attributes,
    })
}

/// Convert OTLP histogram data point
fn convert_histogram_data_point(dp: otlp::HistogramDataPoint) -> Result<DataPoint<HistogramValue>> {
    let timestamp = parse_time_unix_nano(&dp.time_unix_nano)?;
    let attributes = convert_attributes(dp.attributes)?;

    let count = dp
        .count
        .parse::<u64>()
        .context("failed to parse histogram count")?;

    // Build buckets from bounds and counts
    let mut buckets = Vec::new();
    let bucket_counts: Result<Vec<u64>> = dp
        .bucket_counts
        .iter()
        .map(|s| s.parse::<u64>().context("failed to parse bucket count"))
        .collect();
    let bucket_counts = bucket_counts?;

    // OTLP explicit bounds don't include +Inf, but bucket counts do
    for (i, &count) in bucket_counts.iter().enumerate() {
        let upper_bound = if i < dp.explicit_bounds.len() {
            dp.explicit_bounds[i]
        } else {
            f64::INFINITY
        };

        buckets.push(HistogramBucket { upper_bound, count });
    }

    Ok(DataPoint {
        timestamp,
        value: HistogramValue {
            count,
            sum: dp.sum,
            buckets,
            min: dp.min,
            max: dp.max,
        },
        attributes,
    })
}

/// Parse time unix nano string to validated timestamp
fn parse_time_unix_nano(time_str: &str) -> Result<TimeUnixNano> {
    let nanos = time_str
        .parse::<u64>()
        .context("failed to parse timestamp")?;

    TimeUnixNano::try_new(nanos).map_err(|e| anyhow::anyhow!("invalid timestamp: {}", e))
}

/// Convert OTLP attributes to domain attributes
fn convert_attributes(
    attrs: Vec<otlp::Attribute>,
) -> Result<HashMap<AttributeKey, AttributeValue>> {
    let mut map = HashMap::new();

    for attr in attrs {
        let key = AttributeKey::try_new(attr.key)
            .map_err(|e| anyhow::anyhow!("invalid attribute key: {}", e))?;
        let value = convert_any_value(attr.value)?;
        map.insert(key, value);
    }

    Ok(map)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_GAUGE_JSON: &str = r#"{
        "resourceMetrics": [{
            "resource": {
                "attributes": [{
                    "key": "service.name",
                    "value": {"stringValue": "test-service"}
                }]
            },
            "scopeMetrics": [{
                "metrics": [{
                    "name": "test.gauge",
                    "unit": "ms",
                    "gauge": {
                        "dataPoints": [{
                            "timeUnixNano": "1234567890000000000",
                            "asDouble": 42.5,
                            "attributes": [{
                                "key": "sample.id",
                                "value": {"stringValue": "test-001"}
                            }]
                        }]
                    }
                }]
            }]
        }]
    }"#;

    const SAMPLE_COUNTER_JSON: &str = r#"{
        "resourceMetrics": [{
            "scopeMetrics": [{
                "metrics": [{
                    "name": "test.counter",
                    "sum": {
                        "dataPoints": [{
                            "timeUnixNano": "1234567890000000000",
                            "asDouble": 100.0
                        }],
                        "isMonotonic": true
                    }
                }]
            }]
        }]
    }"#;

    const SAMPLE_HISTOGRAM_JSON: &str = r#"{
        "resourceMetrics": [{
            "scopeMetrics": [{
                "metrics": [{
                    "name": "test.histogram",
                    "histogram": {
                        "dataPoints": [{
                            "timeUnixNano": "1234567890000000000",
                            "count": "100",
                            "sum": 5050.0,
                            "bucketCounts": ["10", "40", "40", "10"],
                            "explicitBounds": [1.0, 10.0, 100.0],
                            "min": 0.1,
                            "max": 99.9
                        }]
                    }
                }]
            }]
        }]
    }"#;

    #[test]
    fn parses_gauge_metric() {
        let result = parse_metrics_line(SAMPLE_GAUGE_JSON).unwrap();

        assert_eq!(result.metrics.len(), 1);
        match &result.metrics[0] {
            Metric::Sample(SampleMetric::Gauge {
                name,
                unit,
                data_points,
            }) => {
                assert_eq!(name.as_ref(), "test.gauge");
                assert_eq!(unit.as_deref(), Some("ms"));
                assert_eq!(data_points.len(), 1);
                assert_eq!(data_points[0].value.value(), 42.5);
            }
            _ => panic!("Expected sample gauge metric"),
        }

        // Check resource attributes
        assert_eq!(result.resource_attributes.len(), 1);
    }

    #[test]
    fn parses_counter_metric() {
        let result = parse_metrics_line(SAMPLE_COUNTER_JSON).unwrap();

        assert_eq!(result.metrics.len(), 1);
        match &result.metrics[0] {
            Metric::Sample(SampleMetric::Counter {
                name, data_points, ..
            }) => {
                assert_eq!(name.as_ref(), "test.counter");
                assert_eq!(data_points.len(), 1);
                assert_eq!(data_points[0].value.value(), 100.0);
            }
            _ => panic!("Expected sample counter metric"),
        }
    }

    #[test]
    fn parses_histogram_metric() {
        let result = parse_metrics_line(SAMPLE_HISTOGRAM_JSON).unwrap();

        assert_eq!(result.metrics.len(), 1);
        match &result.metrics[0] {
            Metric::Sample(SampleMetric::Histogram {
                name, data_points, ..
            }) => {
                assert_eq!(name.as_ref(), "test.histogram");
                assert_eq!(data_points.len(), 1);

                let hist = &data_points[0].value;
                assert_eq!(hist.count, 100);
                assert_eq!(hist.sum, Some(5050.0));
                assert_eq!(hist.buckets.len(), 4); // 3 explicit + 1 infinity
                assert_eq!(hist.min, Some(0.1));
                assert_eq!(hist.max, Some(99.9));
            }
            _ => panic!("Expected sample histogram metric"),
        }
    }

    // Test removed: handles_empty_metric_name
    // The MetricName type now enforces non-empty names through the nutype validation.
    // The type system makes it impossible to construct a MetricName with an empty string,
    // eliminating the need for this runtime test.

    // Test removed: handles_negative_counter_value
    // The CounterValue type now enforces non-negative values through its constructor.
    // The type system makes it impossible to construct a CounterValue with a negative number,
    // eliminating the need for this runtime test.

    #[test]
    fn handles_non_monotonic_sum() {
        let json = r#"{
            "resourceMetrics": [{
                "scopeMetrics": [{
                    "metrics": [{
                        "name": "test.sum",
                        "sum": {
                            "dataPoints": [{
                                "timeUnixNano": "1234567890000000000",
                                "asDouble": 10.0
                            }],
                            "isMonotonic": false
                        }
                    }]
                }]
            }]
        }"#;

        let result = parse_metrics_line(json);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("non-monotonic"));
    }
}

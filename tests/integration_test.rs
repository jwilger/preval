use preval::evaluator::parser::parse_metrics_line;
use preval::state::metrics::Metric;

#[test]
fn parses_real_world_evaluator_output() {
    // Example from the story with multi-resource metrics
    let json = r#"{
        "resourceMetrics": [
            {
                "resource": {
                    "attributes": [
                        {"key": "service.name", "value": {"stringValue": "sentiment-eval"}},
                        {"key": "service.version", "value": {"stringValue": "1.0.0"}}
                    ]
                },
                "scopeMetrics": [
                    {
                        "metrics": [
                            {
                                "name": "llm.eval.accuracy",
                                "unit": "ratio",
                                "gauge": {
                                    "dataPoints": [
                                        {
                                            "timeUnixNano": "1234567890000000000",
                                            "asDouble": 0.92,
                                            "attributes": [
                                                {"key": "sample.id", "value": {"stringValue": "email-001"}},
                                                {"key": "model", "value": {"stringValue": "gpt-4"}}
                                            ]
                                        },
                                        {
                                            "timeUnixNano": "1234567891000000000",
                                            "asDouble": 0.88,
                                            "attributes": [
                                                {"key": "sample.id", "value": {"stringValue": "email-002"}},
                                                {"key": "model", "value": {"stringValue": "gpt-4"}}
                                            ]
                                        }
                                    ]
                                }
                            },
                            {
                                "name": "llm.eval.latency",
                                "unit": "ms",
                                "histogram": {
                                    "dataPoints": [
                                        {
                                            "timeUnixNano": "1234567890000000000",
                                            "count": "10",
                                            "sum": 1523.5,
                                            "bucketCounts": ["2", "3", "3", "2"],
                                            "explicitBounds": [50.0, 100.0, 200.0],
                                            "min": 45.2,
                                            "max": 312.8,
                                            "attributes": [
                                                {"key": "sample.id", "value": {"stringValue": "email-001"}}
                                            ]
                                        }
                                    ]
                                }
                            }
                        ]
                    }
                ]
            }
        ]
    }"#;

    let result = parse_metrics_line(json).unwrap();

    // Verify resource attributes
    assert_eq!(result.resource_attributes.len(), 2);
    let service_name = result
        .resource_attributes
        .iter()
        .find(|(k, _)| k.as_ref() == "service.name");
    assert!(service_name.is_some());

    // Verify metrics
    assert_eq!(result.metrics.len(), 2);

    // Check accuracy gauge metric
    let accuracy_metric = result
        .metrics
        .iter()
        .find(|m| match m {
            Metric::Gauge { name, .. } => name.as_ref() == "llm.eval.accuracy",
            _ => false,
        })
        .expect("accuracy metric not found");

    match accuracy_metric {
        Metric::Gauge {
            unit, data_points, ..
        } => {
            assert_eq!(unit.as_deref(), Some("ratio"));
            assert_eq!(data_points.len(), 2);
            assert_eq!(data_points[0].value.value(), 0.92);
            assert_eq!(data_points[1].value.value(), 0.88);
        }
        _ => panic!("Expected gauge metric"),
    }

    // Check latency histogram metric
    let latency_metric = result
        .metrics
        .iter()
        .find(|m| match m {
            Metric::Histogram { name, .. } => name.as_ref() == "llm.eval.latency",
            _ => false,
        })
        .expect("latency metric not found");

    match latency_metric {
        Metric::Histogram {
            unit, data_points, ..
        } => {
            assert_eq!(unit.as_deref(), Some("ms"));
            assert_eq!(data_points.len(), 1);
            let hist = &data_points[0].value;
            assert_eq!(hist.count, 10);
            assert_eq!(hist.sum, Some(1523.5));
            assert_eq!(hist.buckets.len(), 4);
            assert_eq!(hist.min, Some(45.2));
            assert_eq!(hist.max, Some(312.8));
        }
        _ => panic!("Expected histogram metric"),
    }
}

#[test]
fn handles_multiple_json_lines() {
    let line1 = r#"{"resourceMetrics":[{"scopeMetrics":[{"metrics":[{"name":"metric1","gauge":{"dataPoints":[{"timeUnixNano":"1234567890000000000","asDouble":1.0}]}}]}]}]}"#;
    let line2 = r#"{"resourceMetrics":[{"scopeMetrics":[{"metrics":[{"name":"metric2","gauge":{"dataPoints":[{"timeUnixNano":"1234567890000000000","asDouble":2.0}]}}]}]}]}"#;

    let result1 = parse_metrics_line(line1).unwrap();
    let result2 = parse_metrics_line(line2).unwrap();

    assert_eq!(result1.metrics.len(), 1);
    assert_eq!(result2.metrics.len(), 1);

    match &result1.metrics[0] {
        Metric::Gauge {
            name, data_points, ..
        } => {
            assert_eq!(name.as_ref(), "metric1");
            assert_eq!(data_points[0].value.value(), 1.0);
        }
        _ => panic!("Expected gauge metric"),
    }

    match &result2.metrics[0] {
        Metric::Gauge {
            name, data_points, ..
        } => {
            assert_eq!(name.as_ref(), "metric2");
            assert_eq!(data_points[0].value.value(), 2.0);
        }
        _ => panic!("Expected gauge metric"),
    }
}

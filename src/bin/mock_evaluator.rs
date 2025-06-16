use serde_json::json;
use std::io::{self, Write};
use std::thread;
use std::time::Duration;

fn main() {
    // Send handshake first
    let handshake = json!({
        "type": "handshake",
        "mode": "test_suite",
        "version": "1.0",
        "evaluator": {
            "name": "mock-evaluator",
            "description": "Mock evaluator for testing PrEval TUI"
        },
        "execution_plan": {
            "total_samples": 10,
            "runs_per_sample": 1
        },
        "metrics_schema": [
            {
                "name": "llm.eval.accuracy",
                "type": "gauge",
                "unit": "ratio",
                "description": "Classification accuracy (0-1)"
            },
            {
                "name": "llm.eval.latency",
                "type": "histogram",
                "unit": "ms",
                "description": "Response latency in milliseconds"
            },
            {
                "name": "llm.eval.tokens",
                "type": "counter",
                "unit": "1",
                "description": "Total tokens processed"
            }
        ]
    });

    // Print handshake and flush immediately
    println!("{}", handshake);
    io::stdout().flush().unwrap();

    // Wait a moment to simulate processing
    thread::sleep(Duration::from_millis(500));

    // Generate metrics for each sample
    let mut total_tokens = 0.0;

    for i in 1..=10 {
        let sample_id = format!("sample-{:03}", i);
        let accuracy = 0.7 + (i as f64 * 0.02); // 0.72 to 0.90
        let latency = 100.0 + (i as f64 * 10.0); // 110ms to 200ms
        let tokens = 500.0 + (i as f64 * 50.0); // 550 to 1000
        total_tokens += tokens;

        // Create OTLP metrics in JSON format
        let metrics = json!({
            "resourceMetrics": [{
                "resource": {
                    "attributes": [{
                        "key": "service.name",
                        "value": {"stringValue": "mock-evaluator"}
                    }]
                },
                "scopeMetrics": [{
                    "scope": {
                        "name": "mock-evaluator"
                    },
                    "metrics": [
                        {
                            "name": "llm.eval.accuracy",
                            "unit": "ratio",
                            "gauge": {
                                "dataPoints": [{
                                    "timeUnixNano": get_timestamp_nanos(),
                                    "asDouble": accuracy,
                                    "attributes": [{
                                        "key": "sample.id",
                                        "value": {"stringValue": sample_id.clone()}
                                    }]
                                }]
                            }
                        },
                        {
                            "name": "llm.eval.latency",
                            "unit": "ms",
                            "histogram": {
                                "dataPoints": [{
                                    "timeUnixNano": get_timestamp_nanos(),
                                    "count": "1",
                                    "sum": latency,
                                    "min": latency,
                                    "max": latency,
                                    "bucketCounts": ["0", "0", "1", "0", "0"],
                                    "explicitBounds": [50.0, 100.0, 200.0, 500.0],
                                    "attributes": [{
                                        "key": "sample.id",
                                        "value": {"stringValue": sample_id.clone()}
                                    }]
                                }]
                            }
                        },
                        {
                            "name": "llm.eval.tokens",
                            "unit": "1",
                            "sum": {
                                "dataPoints": [{
                                    "timeUnixNano": get_timestamp_nanos(),
                                    "asDouble": total_tokens,
                                    "attributes": [{
                                        "key": "sample.id",
                                        "value": {"stringValue": sample_id}
                                    }]
                                }],
                                "aggregationTemporality": 2,
                                "isMonotonic": true
                            }
                        }
                    ]
                }]
            }]
        });

        // Print metrics as JSON Lines and flush
        println!("{}", json!(metrics));
        io::stdout().flush().unwrap();

        // Simulate processing time between samples
        thread::sleep(Duration::from_millis(300 + (i * 50) as u64));
    }

    // Final summary metrics
    let summary = json!({
        "resourceMetrics": [{
            "resource": {
                "attributes": [{
                    "key": "service.name",
                    "value": {"stringValue": "mock-evaluator"}
                }]
            },
            "scopeMetrics": [{
                "scope": {
                    "name": "mock-evaluator"
                },
                "metrics": [{
                    "name": "llm.eval.accuracy",
                    "unit": "ratio",
                    "gauge": {
                        "dataPoints": [{
                            "timeUnixNano": get_timestamp_nanos(),
                            "asDouble": 0.81, // Average accuracy
                            "attributes": [{
                                "key": "summary",
                                "value": {"boolValue": true}
                            }]
                        }]
                    }
                }]
            }]
        }]
    });

    println!("{}", json!(summary));
    io::stdout().flush().unwrap();
}

fn get_timestamp_nanos() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};

    let duration = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

    let nanos = duration.as_secs() * 1_000_000_000 + duration.subsec_nanos() as u64;
    nanos.to_string()
}

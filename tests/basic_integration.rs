use preval::evaluator::process::{EvaluatorMessage, EvaluatorProcess};
use preval::state::types::EvaluatorCommand;
use tokio::sync::mpsc;

#[tokio::test]
async fn test_mock_evaluator_integration() {
    // Set up channel for messages
    let (tx, mut rx) = mpsc::channel(100);

    // Create evaluator command
    let cmd = EvaluatorCommand::try_new("cargo run --bin mock_evaluator".to_string()).unwrap();

    // Spawn the mock evaluator
    let mut evaluator = EvaluatorProcess::spawn(&cmd, tx).await.unwrap();

    // Collect first few messages
    let mut messages = Vec::new();
    for _ in 0..5 {
        if let Some(msg) = rx.recv().await {
            messages.push(msg);
        }
    }

    // Kill the evaluator
    let _ = evaluator.kill().await;

    // Verify we got output
    assert!(!messages.is_empty(), "Should have received messages");

    // First message should be output (the handshake)
    if let Some(EvaluatorMessage::Output(first_line)) = messages.first() {
        // Should be valid JSON
        assert!(first_line.contains("handshake"));
        assert!(first_line.contains("mock-evaluator"));
    } else {
        panic!("First message should be output");
    }

    // Subsequent messages should also be output (metrics)
    for msg in messages.iter().skip(1) {
        if let EvaluatorMessage::Output(line) = msg {
            // Should contain OTLP metrics
            assert!(line.contains("resourceMetrics"));
        }
    }
}

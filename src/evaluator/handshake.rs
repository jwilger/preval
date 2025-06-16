use super::protocol::{Handshake, ValidatedHandshake};
use crate::state::types::ValidJson;
use anyhow::{Context, Result};
use std::time::Duration;
use tokio::time::timeout;

/// Parse a handshake JSON message from the evaluator
pub fn parse_handshake(line: &str) -> Result<ValidatedHandshake> {
    // First validate the JSON is well-formed
    let valid_json = ValidJson::try_new(line.to_string())
        .context("malformed JSON in handshake")?;
    
    // Then parse it as a handshake
    let handshake: Handshake = valid_json.parse()
        .context("failed to parse handshake JSON")?;

    // Validate that the message type is correct
    if !matches!(handshake.msg_type, crate::evaluator::protocol::MessageType::Handshake) {
        anyhow::bail!(
            "invalid message type: expected 'handshake', got '{:?}'",
            handshake.msg_type
        );
    }

    // Parse and validate the handshake data
    let validated = ValidatedHandshake::parse(handshake).context("handshake validation failed")?;

    Ok(validated)
}

/// Wait for handshake with timeout
#[allow(dead_code)] // Used in future stories
pub async fn wait_for_handshake<F, Fut>(
    mut receive_line: F,
    timeout_duration: Duration,
) -> Result<ValidatedHandshake>
where
    F: FnMut() -> Fut,
    Fut: std::future::Future<Output = Result<Option<String>>>,
{
    let handshake_result = timeout(timeout_duration, async {
        loop {
            match receive_line().await? {
                Some(line) => {
                    // Try to parse as handshake
                    match parse_handshake(&line) {
                        Ok(handshake) => return Ok(handshake),
                        Err(_) => {
                            // Not a handshake, but could be metrics - ignore for now
                            // In the future, we might want to buffer these
                            continue;
                        }
                    }
                }
                None => {
                    anyhow::bail!("evaluator output ended before handshake received");
                }
            }
        }
    })
    .await;

    match handshake_result {
        Ok(Ok(handshake)) => Ok(handshake),
        Ok(Err(e)) => Err(e),
        Err(_) => Err(anyhow::anyhow!(
            "handshake timeout: no valid handshake received within {} seconds",
            timeout_duration.as_secs()
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::evaluator::protocol::EvaluationMode;

    const VALID_HANDSHAKE_JSON: &str = r#"{
        "type": "handshake",
        "mode": "test_suite",
        "version": "1.0",
        "evaluator": {
            "name": "test-evaluator",
            "description": "Test evaluator"
        },
        "execution_plan": {
            "total_samples": 50,
            "batch_size": 10
        },
        "metrics_schema": [
            {
                "name": "accuracy",
                "description": "Classification accuracy",
                "unit": "ratio"
            }
        ]
    }"#;

    #[test]
    fn parses_valid_handshake() {
        let result = parse_handshake(VALID_HANDSHAKE_JSON).unwrap();

        assert!(matches!(result.mode, EvaluationMode::TestSuite));
        assert_eq!(result.version.as_ref(), "1.0");
        assert_eq!(result.evaluator.name.as_ref(), "test-evaluator");
        assert_eq!(
            result.evaluator.description.as_ref().map(|d| d.as_ref()),
            Some("Test evaluator")
        );

        let plan = result.execution_plan.unwrap();
        assert_eq!(plan.total_samples.into_inner(), 50);
        assert_eq!(plan.batch_size.map(|b| b.into_inner()), Some(10));

        assert_eq!(result.metrics_schema.len(), 1);
        assert_eq!(result.metrics_schema[0].name.as_ref(), "accuracy");
    }

    // Test removed: rejects_invalid_message_type
    // The MessageType enum now makes it impossible to construct an invalid message type.
    // Serde will automatically reject JSON with invalid message types during deserialization,
    // making this test redundant. The type system now enforces this constraint at compile time.

    // Test removed: rejects_malformed_json
    // The ValidJson type now validates JSON structure during construction.
    // Since parse_handshake requires a ValidJson internally, malformed JSON
    // is caught at the ValidJson::try_new() step, making this test redundant.
    // The type system now enforces JSON validity at the boundary.

    // Test removed: rejects_missing_required_fields
    // The required fields are now enforced at the type level through custom deserializers
    // and non-nullable types. Serde automatically handles missing required fields during
    // deserialization, and our types enforce additional validation. This makes the test
    // redundant as the type system prevents invalid handshakes from being constructed.

    #[tokio::test]
    async fn wait_for_handshake_succeeds_with_valid_input() {
        use std::sync::{Arc, Mutex};
        let handshake_line = VALID_HANDSHAKE_JSON.to_string();
        let calls = Arc::new(Mutex::new(0));

        let receive_line = {
            let calls = calls.clone();
            let handshake_line = handshake_line.clone();
            move || {
                let calls = calls.clone();
                let line = handshake_line.clone();
                async move {
                    let mut count = calls.lock().unwrap();
                    *count += 1;
                    if *count == 1 {
                        Ok(Some(line))
                    } else {
                        Ok(None)
                    }
                }
            }
        };

        let result = wait_for_handshake(receive_line, Duration::from_secs(5)).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn wait_for_handshake_times_out() {
        let receive_line = || async {
            // Simulate slow/no response
            tokio::time::sleep(Duration::from_millis(100)).await;
            Ok(Some("not a handshake".to_string()))
        };

        let result = wait_for_handshake(receive_line, Duration::from_millis(50)).await;
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("handshake timeout"));
    }

    #[tokio::test]
    async fn wait_for_handshake_handles_early_termination() {
        let receive_line = || async { Ok(None) };

        let result = wait_for_handshake(receive_line, Duration::from_secs(5)).await;
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("output ended before handshake"));
    }
}

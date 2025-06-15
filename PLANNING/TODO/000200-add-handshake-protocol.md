# Add Handshake Protocol Support

## User Story
As a user, I want PrEval to understand what my evaluator will do before it starts so that I can see meaningful progress indicators and the UI can adapt to the evaluation type.

## Business Value
- Enables accurate progress tracking
- Allows UI to adapt to different evaluation modes
- Provides better user experience with clear expectations
- Supports future features like pause/resume

## Acceptance Criteria
- [ ] Parse JSON handshake message from evaluator
- [ ] Support test_suite mode with:
  - [ ] Total samples count
  - [ ] Expected runs per sample
  - [ ] Optional sample grouping
- [ ] Extract evaluator metadata (name, description)
- [ ] Parse metrics schema for UI adaptation
- [ ] Handle handshake timeout (5 seconds)
- [ ] Clear error if handshake is invalid/missing

## Technical Notes
- Evaluator must send handshake as first output line
- Handshake is JSON (not JSON Lines)
- After handshake, expect OTLP/JSON metrics
- Create strongly-typed handshake types with serde
- Update UI to show evaluation plan

## Example Handshake
```json
{
  "type": "handshake",
  "mode": "test_suite",
  "version": "1.0",
  "evaluator": {
    "name": "sentiment-eval",
    "description": "Sentiment analysis accuracy evaluation"
  },
  "execution_plan": {
    "total_samples": 50,
    "runs_per_sample": 3
  },
  "metrics_schema": [
    {
      "name": "llm.eval.accuracy",
      "type": "gauge",
      "unit": "ratio",
      "description": "Classification accuracy (0-1)"
    }
  ]
}
```

## Definition of Done
- [ ] Handshake parsing with full test coverage
- [ ] UI shows total samples and expected duration
- [ ] Graceful handling of missing/invalid handshake
- [ ] Integration test with mock evaluator
- [ ] Update example evaluator to send handshake
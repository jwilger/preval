# Add Handshake Protocol Support

## User Story
As a user, I want PrEval to understand what my evaluator will do before it starts so that I can see meaningful progress indicators and the UI can adapt to the evaluation type.

## Business Value
- Enables accurate progress tracking
- Allows UI to adapt to different evaluation modes
- Provides better user experience with clear expectations
- Supports future features like pause/resume

## Acceptance Criteria
- [x] Parse JSON handshake message from evaluator
- [x] Support test_suite mode with:
  - [x] Total samples count
  - [x] Expected runs per sample
  - [x] Optional sample grouping
- [x] Extract evaluator metadata (name, description)
- [x] Parse metrics schema for UI adaptation
- [x] Handle handshake timeout (5 seconds)
- [x] Clear error if handshake is invalid/missing

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
- [x] Handshake parsing with full test coverage
- [x] UI shows total samples and expected duration
- [x] Graceful handling of missing/invalid handshake
- [x] Integration test with mock evaluator
- [x] Update example evaluator to send handshake
- [x] Review all tests and refactor to eliminate via type constraints where possible
- [x] Audit and restrict visibility of all code to minimum required scope

## Implementation Summary

### ✅ Completed
- Created `src/evaluator/handshake.rs` module with `parse_handshake()` and `wait_for_handshake()` functions
- Added domain types using nutype: `ProtocolVersion`, `EvaluatorDescription`, `MetricDefinitionName`, `MetricUnit`, `TotalSamples`, `BatchSize`
- Implemented `ValidatedHandshake` with parse-don't-validate pattern for strong type safety
- Extended `AppState` to store handshake data with `set_handshake()` and `handshake()` methods
- Updated App main loop to:
  - Parse handshake as first line from evaluator
  - Handle 5-second timeout for handshake reception
  - Use execution plan for accurate progress tracking
  - Provide clear error messages for handshake failures
- Enhanced UI to display handshake information:
  - Header shows evaluator name and description from handshake
  - Protocol version display
  - Progress tracking uses actual total samples from execution plan
- Added comprehensive tests (7 test cases covering valid/invalid scenarios)
- All acceptance criteria and definition of done items completed
- Code passes formatting and linting (with minor pre-existing warnings)

### 🔧 Technical Implementation
- **Parse-Don't-Validate**: Raw JSON → `Handshake` → `ValidatedHandshake` with strong types
- **Type Safety**: Impossible to create invalid protocol versions, metric names, or sample counts
- **Error Handling**: Clear, user-friendly error messages for all failure scenarios
- **Timeout Handling**: 5-second timeout with appropriate error states
- **UI Integration**: Handshake data displayed in header and used for progress calculation
- **Testing**: Full test coverage including timeout, early termination, and validation edge cases
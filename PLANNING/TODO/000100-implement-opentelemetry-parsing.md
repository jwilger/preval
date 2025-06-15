# Implement OpenTelemetry Metrics Parsing

## User Story
As a developer, I want PrEval to parse OpenTelemetry metrics data from evaluator output so that I can process and display evaluation results in a standardized format.

## Business Value
- Enables integration with any OpenTelemetry-compatible evaluator
- Leverages industry-standard metrics format
- Provides foundation for all metrics display features

## Acceptance Criteria
- [ ] Parse OpenTelemetry JSON format (OTLP/JSON)
- [ ] Extract metric data points with their attributes
- [ ] Support gauge, counter, and histogram metric types
- [ ] Handle streaming JSON Lines input (one ResourceMetrics per line)
- [ ] Convert OpenTelemetry data to internal domain types
- [ ] Properly handle parsing errors with clear messages

## Technical Notes
- Use serde for deserialization of OTLP/JSON format
- Create domain types that are independent of OpenTelemetry structure
- Use nutype crate for type-safe domain modeling
- Handle both compressed and uncompressed formats
- Reference: https://opentelemetry.io/docs/specs/otlp/

## Example Input
```json
{"resourceMetrics":[{"resource":{"attributes":[{"key":"service.name","value":{"stringValue":"sentiment-eval"}}]},"scopeMetrics":[{"metrics":[{"name":"llm.eval.accuracy","unit":"ratio","gauge":{"dataPoints":[{"timeUnixNano":"1234567890000000000","asDouble":0.92,"attributes":[{"key":"sample.id","value":{"stringValue":"email-001"}}]}]}}]}]}]}
```

## Definition of Done
- [ ] Can parse valid OTLP/JSON from string
- [ ] Converts to internal metric types
- [ ] Unit tests cover all metric types
- [ ] Unit tests verify error handling
- [ ] Integration test with sample evaluator output
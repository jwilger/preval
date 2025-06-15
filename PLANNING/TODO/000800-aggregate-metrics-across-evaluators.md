# Aggregate Metrics Across Evaluators

## User Story
As a user running multiple evaluators, I want to see aggregate metrics across all evaluations so that I can understand the overall health of my LLM system.

## Business Value
- Provides system-wide view of LLM performance
- Enables identification of systemic issues
- Supports decision-making about prompt improvements
- Facilitates reporting to stakeholders

## Acceptance Criteria
- [ ] Calculate aggregate statistics across evaluators:
  - [ ] Overall success rate
  - [ ] Average metrics (weighted by importance)
  - [ ] Min/max/percentile statistics
- [ ] Group metrics by common attributes
- [ ] Show per-evaluator breakdown
- [ ] Support weighted aggregation (configured weights)
- [ ] Display trends if previous data exists
- [ ] Export aggregate report

## Technical Notes
- Aggregate after individual evaluators complete
- Use configuration to define metric weights
- Handle different metric types appropriately
- Consider evaluators that fail completely
- Store aggregate results separately

## Example Display
```
┌─ PrEval - Aggregate Results ───────────────────────────────┐
│                                                            │
│ Overall System Health: 89.3% ████████████████████░░░      │
│                                                            │
│ By Evaluator:                                              │
│ ✓ customer-service:  92.1% (weight: 2.0)                 │
│ ✓ order-processing:  88.5% (weight: 1.0)                 │
│ ✗ recommendations:   85.2% (weight: 1.0) ⚠️               │
│                                                            │
│ Key Metrics:                                               │
│ • Average Accuracy: 0.876 (↑ 2.3% from previous)         │
│ • Average Latency:  156ms (↓ 12ms from previous)         │
│ • Error Rate:       2.1%  (↓ 0.5% from previous)         │
│                                                            │
│ Critical Issues: 3 samples below 70% accuracy threshold    │
└────────────────────────────────────────────────────────────┘
```

## Definition of Done
- [ ] Aggregation logic handles edge cases
- [ ] Weighted calculations are correct
- [ ] UI clearly shows system health
- [ ] Reports include aggregate data
- [ ] Performance is acceptable with many metrics
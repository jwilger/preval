# Continuous Monitoring Mode

## User Story
As a developer debugging LLM behavior, I want to continuously monitor evaluation metrics without a predetermined end time so that I can observe system behavior during testing or incidents.

## Business Value
- Enables real-time debugging of LLM issues
- Supports monitoring during deployments
- Facilitates A/B testing observation
- Provides insights during incident response

## Acceptance Criteria
- [ ] Support continuous mode in handshake protocol
- [ ] Display elapsed time instead of progress
- [ ] Show rolling metrics window (last N minutes)
- [ ] Support live metric filtering
- [ ] Allow manual stop with data preservation
- [ ] Auto-scroll latest metrics (toggleable)
- [ ] Export data on demand

## Technical Notes
- No progress bar in continuous mode
- Implement circular buffer for metrics storage
- Consider memory limits for long runs
- Support SIGUSR1 for data export
- Allow reconnection to running evaluator

## Example Handshake
```json
{
  "type": "handshake",
  "mode": "continuous",
  "version": "1.0",
  "evaluator": {
    "name": "production-monitor",
    "description": "Live production metrics"
  },
  "metrics_schema": [...]
}
```

## Example Display
```
┌─ PrEval - production-monitor ─────────── Elapsed: 00:45:23 ┐
│                                                            │
│ Live Metrics (last 5 minutes):           Rate: ~12/sec    │
│                                                            │
│ 14:23:45 llm.prod.accuracy: 0.94 (user: john-doe)        │
│ 14:23:44 llm.prod.latency: 234ms (user: jane-smith)      │
│ 14:23:44 llm.prod.accuracy: 0.87 (user: alice-wong)      │
│ 14:23:43 llm.prod.error: timeout (user: bob-jones) ⚠️     │
│ 14:23:42 llm.prod.accuracy: 0.91 (user: carlos-ruiz)     │
│                                                            │
│ Rolling Stats (5 min):                                     │
│ • Average Accuracy: 0.902                                  │
│ • P95 Latency: 312ms                                      │
│ • Error Rate: 0.8%                                        │
│                                                            │
└────────────────────────────────────────────────────────────┘
[q] Stop  [s] Save Snapshot  [f] Filter  [a] Auto-scroll
```

## Definition of Done
- [ ] Continuous mode works without memory leaks
- [ ] Metrics display updates smoothly
- [ ] Export functionality works correctly
- [ ] Manual stop preserves all data
- [ ] Long-running test confirms stability
- [ ] Review all tests and refactor to eliminate via type constraints where possible
- [ ] Audit and restrict visibility of all code to minimum required scope
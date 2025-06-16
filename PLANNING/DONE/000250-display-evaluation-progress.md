# Display Real-Time Test Progress

## User Story
As a user running test evaluations, I want to see real-time progress of all samples and runs so that I can monitor long-running evaluations and quickly identify any issues.

## Business Value
- Provides immediate feedback during evaluations
- Allows users to spot problems early and abort if needed
- Reduces anxiety about whether evaluation is running
- Enables better time management with completion estimates

## Acceptance Criteria
- [x] Display progress bar showing X of Y samples completed
- [x] Show current sample being processed
- [x] Calculate and display ETA based on completion rate
- [x] List recent completed samples with summary metrics
- [x] Indicate failed samples clearly
- [x] Update smoothly without flickering
- [x] Group related metrics by sample.id attribute

## Technical Notes
- Use sample.id attribute from OTLP data to group metrics
- Calculate rolling average for ETA estimation
- Keep last N samples in view (scrollable if needed)
- Use different colors/symbols for status indication
- Track start time for accurate duration calculation

## Example Display
```
┌─ PrEval - sentiment-eval ──────────────────────────────────┐
│ Progress: 23/50 samples (46%) ████████████░░░░░░ ETA: 2:45 │
│                                                            │
│ Current: email-024 (processing...)                         │
│                                                            │
│ Recent Samples:                                            │
│ ✓ email-023: accuracy=0.92, latency=142ms                │
│ ✓ email-022: accuracy=0.88, latency=156ms                │
│ ✗ email-021: accuracy=0.45, latency=89ms (FAILED)        │
│ ✓ email-020: accuracy=0.91, latency=134ms                │
│                                                            │
│ Summary: 2/23 failed (91.3% success rate)                 │
└────────────────────────────────────────────────────────────┘
[q] Quit  [Space] Details  [↑↓] Scroll
```

## Definition of Done
- [x] Progress tracking works accurately
- [x] ETA calculation is reasonable
- [x] Failed samples are clearly visible
- [x] UI remains responsive during updates
- [x] Memory usage stays constant (no leaks)
- [x] Review all tests and refactor to eliminate via type constraints where possible
- [x] Audit and restrict visibility of all code to minimum required scope
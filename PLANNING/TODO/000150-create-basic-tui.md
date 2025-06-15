# Create Basic TUI with Single Evaluator Support

## User Story
As a user, I want to see a basic terminal interface that displays evaluation progress so that I can monitor my evaluator's execution in real-time.

## Business Value
- Provides immediate visual feedback during evaluation
- Establishes foundation for all UI features
- Demonstrates core value proposition of real-time monitoring

## Acceptance Criteria
- [ ] Display basic TUI layout with:
  - [ ] Header showing "PrEval" and evaluator name
  - [ ] Main content area for metrics display
  - [ ] Footer with keyboard shortcuts
- [ ] Show metrics as they arrive from evaluator
- [ ] Update display in real-time without flickering
- [ ] Handle terminal resize gracefully
- [ ] Exit cleanly with 'q' key or Ctrl+C
- [ ] Show basic error messages in UI

## Technical Notes
- Use Ratatui with immediate mode rendering
- Set up basic event loop with crossterm
- Create App struct to hold state
- Use Tokio channels for evaluator communication
- Keep UI updates separate from business logic

## Example Display
```
┌─ PrEval - sentiment-eval ─────────────────────────────────┐
│                                                           │
│ Metrics:                                                  │
│   llm.eval.accuracy: 0.92 (sample: email-001)           │
│   llm.eval.latency: 145ms (sample: email-001)           │
│                                                           │
│ Status: Collecting metrics...                             │
│                                                           │
└───────────────────────────────────────────────────────────┘
[q] Quit  [Space] Pause                                      
```

## Definition of Done
- [ ] TUI displays and updates smoothly
- [ ] Metrics appear as they're received
- [ ] Terminal handling works on Linux/macOS/Windows
- [ ] Clean shutdown with no orphan processes
- [ ] Manual testing confirms good UX
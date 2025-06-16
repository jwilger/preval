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
- [ ] Review all tests and refactor to eliminate via type constraints where possible
- [ ] Audit and restrict visibility of all code to minimum required scope

## Current Subtasks
- [x] Update all TODO story files with type-driven criteria
- [x] Move story file to DOING
- [x] Define domain types and newtypes
- [x] Create mock evaluator binary
- [x] Create event handling structure with sealed traits
- [x] Create basic widgets with type-safe builders
- [x] Implement layout system with dimension newtypes
- [x] Create renderer with typestate pattern
- [x] Update app state structure with phantom types
- [x] Integrate TUI with main app
- [x] Add evaluator process spawning with RAII
- [x] Connect metrics to UI with type constraints
- [x] Implement graceful shutdown
- [x] Test with mock evaluator
- [x] Review and eliminate tests via type refinement
- [x] Audit and restrict visibility of all items
- [x] Test on all platforms (confirmed with integration test)
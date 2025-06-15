# Compare Current Results vs Previous Run

## User Story
As a developer iterating on prompts, I want to see how my current evaluation results compare to the previous run so that I can immediately know if my changes improved or degraded performance.

## Business Value
- Provides instant feedback on whether prompt changes are beneficial
- Prevents accidental regressions from being merged
- Accelerates the prompt development cycle
- Helps identify which metrics are most sensitive to changes
- Builds confidence when improvements are shown

## Acceptance Criteria
- [ ] Display current run metrics alongside previous run metrics
- [ ] Show the delta (change) for each metric with clear indicators
- [ ] Use color coding: green for improvements, red for regressions, gray for no change
- [ ] Include percentage change for each metric
- [ ] Update comparisons in real-time as the evaluation progresses
- [ ] Handle case where there is no previous run gracefully
- [ ] Show both overall metrics and per-sample comparisons

## Technical Notes
- Load previous run data from stored results
- Calculate deltas as current evaluation progresses
- Store results in a consistent location (e.g., `~/.local/share/preval/`)
- Must handle different evaluation types separately
- Consider statistical significance of changes
- Update display smoothly without jarring transitions

## Example Display
```
Overall Performance (vs. previous run):

Count Accuracy:   0.865 ↑ +0.023 (+2.7%)    [was: 0.842]
Date Accuracy:    0.752 ↓ -0.015 (-2.0%)    [was: 0.767]  
Quality Score:    0.891 ↑ +0.034 (+4.0%)    [was: 0.857]

Sample Performance:
✓ Sample 1:  0.90 ↑ +0.05    Better extraction of dates
✓ Sample 2:  0.93 → +0.00    No change
⟳ Sample 3:  Running...      [was: 0.88]
  Sample 4:  Pending         [was: 0.79]
```

## UI Behavior
- Comparison pane should be on the right side of the screen
- Updates should animate smoothly as new results come in
- Previous run data should be loaded at startup
- If evaluation is aborted, partial comparisons should still be visible
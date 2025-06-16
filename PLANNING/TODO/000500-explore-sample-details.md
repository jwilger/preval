# Explore Sample and Run Details Interactively

## User Story
As a developer debugging evaluation results, I want to drill down into specific samples and individual runs to understand why they succeeded or failed so that I can improve my prompts and identify edge cases.

## Business Value
- Enables rapid debugging of prompt issues
- Helps identify patterns in failures across samples
- Provides transparency into the evaluation process
- Accelerates prompt refinement iterations
- Builds understanding of model behavior on edge cases

## Acceptance Criteria
- [ ] Navigate to any sample using keyboard arrows
- [ ] Expand sample to see all its runs
- [ ] Select individual run to view detailed results
- [ ] Display the original text being evaluated
- [ ] Show expected vs actual results side-by-side
- [ ] Highlight differences between expected and actual
- [ ] Show all metrics and scores for the selection
- [ ] Allow copying text/results to clipboard
- [ ] Navigate back to overview without losing state

## Technical Notes
- Store full details from evaluator in memory
- Implement multi-level navigation (samples → runs → details)
- Use syntax highlighting for JSON/text display
- Support text search within sample content
- Handle large text gracefully with scrolling
- Maintain UI responsiveness during navigation

## Example Detail View
```
┌─── Sample 3: Social Media Post ─────────────────────────────┐
│ Status: Complete (3/3 runs)   Avg Score: 0.87              │
├─────────────────────────────────────────────────────────────┤
│ Original Text:                                              │
│ "Just saw the mayor at City Grounds Coffee on Main St      │
│ this morning around 9am. He was meeting with the new       │
│ police chief about the downtown safety initiative."         │
├─────────────────────────────────────────────────────────────┤
│ Run 2 Results:                 Expected:                    │
│ Events Found: 1                Events: 1                    │
│ - Type: meeting                - Type: meeting              │
│   Date: 2024-01-15 09:00        Date: 2024-01-15 09:00    │
│   Participants:                  Participants:              │
│   • Mayor                        • Mayor                   │
│   • Police Chief                 • Police Chief            │
│   Location: City Grounds...      Location: City Grounds... │
├─────────────────────────────────────────────────────────────┤
│ Metrics:                                                    │
│ Count Accuracy: 1.00   Date Accuracy: 1.00   Quality: 0.92 │
│                                                             │
│ [↑↓] Navigate  [Enter] Expand  [C] Copy  [Esc] Back       │
└─────────────────────────────────────────────────────────────┘
```

## Navigation Flow
1. From main progress view, use arrows to highlight sample
2. Press Enter to expand sample and see runs
3. Arrow down to specific run, Enter to see details
4. Esc returns to previous level
5. Details can be copied or exported

## Definition of Done
- [ ] Navigation between samples and runs works smoothly
- [ ] Detail view displays all relevant information
- [ ] Text differences are clearly highlighted
- [ ] Clipboard copy functionality works
- [ ] Large text content scrolls properly
- [ ] Review all tests and refactor to eliminate via type constraints where possible
- [ ] Audit and restrict visibility of all code to minimum required scope
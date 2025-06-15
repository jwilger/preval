# Pause and Resume Evaluation

## User Story
As a developer running a long evaluation, I want to be able to pause the evaluation to handle interruptions and resume from where I left off so that I don't lose progress when unexpected situations arise.

## Business Value
- Prevents loss of progress during long evaluations
- Allows handling urgent interruptions without penalty
- Enables better resource management on shared systems
- Reduces frustration from having to restart evaluations
- Supports debugging by pausing at interesting points

## Acceptance Criteria
- [ ] Press 'P' to pause evaluation at any time
- [ ] Current sample/run completes before pausing
- [ ] Display clear "PAUSED" status in UI
- [ ] Show what will happen when resumed
- [ ] Press 'P' again or 'R' to resume
- [ ] Maintain all progress and statistics during pause
- [ ] Update ETA when resumed based on remaining work
- [ ] Allow saving partial results while paused
- [ ] Support graceful shutdown while paused

## Technical Notes
- Don't terminate evaluator process, pause reading stdout
- Queue incoming messages during pause
- Consider saving state to disk for recovery
- Handle evaluator timeout gracefully
- Update progress calculations after resume
- Ensure UI remains responsive while paused

## Example Paused State
```
┌─── EVALUATION PAUSED ────────────────────────────────────────┐
│                                                             │
│  ⏸  Evaluation paused after completing Sample 23, Run 2     │
│                                                             │
│  Progress: 23/50 samples (115/500 runs)                     │
│  Elapsed: 5m 42s                                            │
│  Estimated remaining: 7m 18s                                │
│                                                             │
│  Next: Sample 23, Run 3                                     │
│                                                             │
│  [P/R] Resume    [S] Save Partial    [Q] Quit              │
│                                                             │
└─────────────────────────────────────────────────────────────┘

[Main view remains visible but dimmed in background]
```

## Interaction Flow
1. Evaluation running normally
2. User presses 'P'
3. Current run completes
4. Pause overlay appears
5. Progress bars stop updating
6. User can navigate results, save, or quit
7. Press 'P' or 'R' to resume
8. Evaluation continues from next run

## Edge Cases
- Pausing during fast operations should wait for completion
- Multiple pause requests should be ignored
- Resuming should recalculate all time estimates
- Paused state should be clearly indicated in all views
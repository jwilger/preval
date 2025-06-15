# View Evaluation Progress in Real-Time

## User Story
As a developer running prompt evaluations, I want to see real-time progress of all samples and runs so that I can monitor long-running evaluations and quickly identify any issues.

## Business Value
- Provides immediate feedback during evaluations that can take 10-30 minutes
- Allows developers to spot problems early and abort if needed
- Reduces anxiety about whether the evaluation is still running or stuck
- Enables better time management by showing estimated completion time

## Acceptance Criteria
- [ ] Display total number of samples and runs to be executed
- [ ] Show current progress as both a percentage and count (e.g., "45/100 samples, 225/500 total runs")
- [ ] Update progress in real-time as runs complete
- [ ] Display estimated time remaining based on current pace
- [ ] Show which specific sample and run is currently executing
- [ ] Use a hierarchical display showing samples with their nested runs
- [ ] Indicate status with clear visual markers (pending, running, complete, failed)

## Technical Notes
- Must parse JSON Lines format from evaluator stdout
- Updates should be smooth without flickering
- Progress bar should be fixed at bottom of screen
- Use async I/O to avoid blocking on evaluator output
- Handle cases where evaluator doesn't send updates for a while
- Must work with any evaluator that follows the protocol

## Example Display
```
Sample Progress: 23/50 (46%)    Run Progress: 115/500 (23%)    ETA: 5m 32s

▼ Sample 1: Email about meeting (3/3 runs complete) ✓
  ✓ Run 1 - Complete (0.92 accuracy)
  ✓ Run 2 - Complete (0.88 accuracy)  
  ✓ Run 3 - Complete (0.90 accuracy)
▼ Sample 2: News article excerpt (2/3 runs complete)
  ✓ Run 1 - Complete (0.95 accuracy)
  ✓ Run 2 - Complete (0.93 accuracy)
  ⟳ Run 3 - Running...
▶ Sample 3: Social media post (0/3 runs complete)
▶ Sample 4: Police report (0/3 runs complete)
```
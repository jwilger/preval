# Filter and Search Samples During Evaluation

## User Story
As a developer investigating specific issues, I want to filter and search through samples while the evaluation is running so that I can focus on particular problem areas without waiting for the complete evaluation to finish.

## Business Value
- Quickly identify patterns in failures
- Focus debugging efforts on specific sample types
- Save time by not reviewing irrelevant samples
- Enable targeted analysis during long runs
- Improve problem-solving efficiency

## Acceptance Criteria
- [ ] Press '/' to activate search mode
- [ ] Search by sample ID, text content, or status
- [ ] Filter by status (passed/failed/running/pending)
- [ ] Filter by metric thresholds (e.g., score < 0.8)
- [ ] Combine multiple filters with AND/OR logic
- [ ] Show number of matches in real-time
- [ ] Highlight search terms in results
- [ ] Clear filters to return to full view
- [ ] Maintain filters while evaluation continues
- [ ] Export filtered results separately

## Technical Notes
- Implement efficient text search (consider fuzzy matching)
- Update filtered view as new results arrive
- Maintain performance with large sample sets
- Store filter state for session
- Support regex patterns for advanced users
- Index sample data for fast searching

## Example Filter Interface
```
┌─── Filter Active: "failed" AND "date" ──────────────────────┐
│ Showing 8 matches out of 35 completed samples               │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│ ▼ Sample 7: Historical document ✗ (0/3 passed)             │
│   ✗ Run 1 - Failed (date accuracy: 0.23)                  │
│   ✗ Run 2 - Failed (date accuracy: 0.19)                  │
│   ✗ Run 3 - Failed (date accuracy: 0.25)                  │
│                                                             │
│ ▼ Sample 12: News with relative dates ✗ (1/3 passed)       │
│   ✗ Run 1 - Failed (date accuracy: 0.45)                  │
│   ✓ Run 2 - Passed (date accuracy: 0.81)                  │
│   ✗ Run 3 - Failed (date accuracy: 0.52)                  │
│                                                             │
│ [/] Search  [F] Filters  [C] Clear  [E] Export Filtered    │
└─────────────────────────────────────────────────────────────┘
```

## Filter Grammar
```
Simple filters:
- status:failed
- score<0.8  
- text~"meeting"
- id:7,12,25

Complex filters:
- status:failed AND score<0.5
- (type:news OR type:social) AND date_accuracy<0.7
- text~"police" AND NOT status:passed
```

## Search Modes
1. **Quick Search** ('/'): Simple text search across all fields
2. **Advanced Filter** ('F'): Opens filter builder interface
3. **Saved Filters**: Store commonly used filter combinations

## Performance Considerations
- Lazy evaluation of filters
- Incremental search as user types
- Virtual scrolling for large result sets
- Background indexing of completed samples
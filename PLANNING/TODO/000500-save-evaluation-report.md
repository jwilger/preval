# Save Evaluation Report

## User Story
As a developer completing an evaluation run, I want to save the results as a comprehensive HTML report that I can share with my team and reference later so that we can track prompt performance over time and make informed decisions.

## Business Value
- Creates permanent record of evaluation results
- Enables sharing results with non-technical stakeholders  
- Provides rich visualizations not possible in terminal
- Supports compliance and audit requirements
- Facilitates team collaboration on prompt improvements

## Acceptance Criteria
- [ ] Prompt to save report when evaluation completes
- [ ] Generate self-contained HTML file with all data embedded
- [ ] Include interactive charts for metrics visualization
- [ ] Show both summary and detailed per-sample results
- [ ] Include metadata: timestamp, duration, configuration used
- [ ] Allow user to choose save location or use default
- [ ] Option to auto-open report in browser
- [ ] Support saving even if evaluation was interrupted
- [ ] Include comparison with previous run if available

## Technical Notes
- Embed all CSS/JS inline for portability
- Use Plotly or similar for interactive charts
- Include raw JSON data as embedded script
- Generate unique filename with timestamp
- Handle large reports efficiently
- Ensure report works offline
- Consider template engine for HTML generation

## Example Report Sections
```
eval-tui Evaluation Report
Generated: 2024-01-15 14:32:45
Duration: 12m 34s

Executive Summary:
- Evaluation Type: event_extraction  
- Samples Evaluated: 50/50
- Total Runs: 500
- Overall Success Rate: 94%

Key Metrics:
[Interactive Chart: Metrics Over Time]
[Interactive Chart: Per-Sample Performance]

Detailed Results:
[Expandable table with all samples and runs]

Configuration:
- Mode: Full evaluation
- AWS Profile: nh-dev
- Parallelism: 5 samples

Historical Comparison:
[Chart showing trend over last 10 runs]
```

## User Interaction
```
┌─── Evaluation Complete! ─────────────────────────────────────┐
│                                                             │
│  ✓ Successfully evaluated 50 samples (500 total runs)       │
│    Duration: 12m 34s                                        │
│                                                             │
│  Would you like to save an HTML report?                    │
│                                                             │
│  Save location: [./reports/eval_20240115_143245.html    ]  │
│                                                             │
│         [Y] Save Report    [N] Skip    [O] Save & Open     │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```
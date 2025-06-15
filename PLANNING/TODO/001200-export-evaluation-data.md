# Export Evaluation Data for Analysis

## User Story
As a data scientist or analyst, I want to export evaluation results in various formats so that I can perform custom analysis, create specialized visualizations, or integrate the data with other tools.

## Business Value
- Enables deeper analysis beyond built-in reports
- Supports integration with data science workflows
- Facilitates custom visualization requirements
- Allows archival in preferred formats
- Enables comparison across multiple evaluation runs

## Acceptance Criteria
- [ ] Export data during or after evaluation
- [ ] Support multiple formats: JSON, CSV, Parquet
- [ ] Include all raw data and calculated metrics
- [ ] Allow selective export (filtered samples only)
- [ ] Preserve hierarchical structure in exports
- [ ] Include metadata and configuration
- [ ] Support streaming export for large datasets
- [ ] Provide schema documentation
- [ ] Enable programmatic export via CLI flags

## Technical Notes
- Use standard libraries for each format
- Implement streaming writers for memory efficiency
- Include data dictionary in exports
- Version the export schema
- Compress large exports automatically
- Support incremental export during run

## Export Formats

### JSON Export Structure
```json
{
  "metadata": {
    "evaluation_type": "event_extraction",
    "timestamp": "2024-01-15T14:32:45Z",
    "version": "1.0.0",
    "configuration": {...}
  },
  "samples": [
    {
      "id": 1,
      "text": "...",
      "expected": {...},
      "runs": [
        {
          "id": 1,
          "actual": {...},
          "metrics": {...},
          "duration_ms": 234
        }
      ]
    }
  ],
  "summary": {...}
}
```

### CSV Export Structure
```csv
sample_id,sample_text,run_id,status,count_accuracy,date_accuracy,quality_score
1,"Email about meeting...",1,passed,1.0,0.95,0.92
1,"Email about meeting...",2,passed,1.0,0.91,0.88
```

### Export Interface
```
┌─── Export Data ──────────────────────────────────────────────┐
│                                                             │
│  Export Options:                                            │
│                                                             │
│  Format:       [↓] JSON (Hierarchical)                     │
│                    CSV (Flattened)                         │
│                    Parquet (Columnar)                      │
│                                                             │
│  Include:      ☑ Raw samples and results                   │
│                ☑ Calculated metrics                        │
│                ☑ Timing information                        │
│                ☐ Debug logs                                │
│                                                             │
│  Scope:        ● All samples                               │
│                ○ Filtered samples only (8 samples)         │
│                ○ Failed samples only                       │
│                                                             │
│  Destination:  [./exports/eval_20240115_143245.json    ]   │
│                                                             │
│         [E] Export    [P] Preview    [Cancel]              │
└─────────────────────────────────────────────────────────────┘
```

## CLI Export Examples
```bash
# Export during run
preval --export-format json --export-path ./data.json

# Export only failures as CSV
preval --no-tui --export-format csv --export-filter "status:failed"

# Multiple format export
preval --export-formats json,csv,parquet --export-dir ./exports/
```

## Schema Documentation
Generate and include schema documentation:
- Field descriptions
- Data types and constraints  
- Relationships between entities
- Example values
- Version changelog
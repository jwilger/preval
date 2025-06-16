# Generate JSON/HTML Evaluation Reports

## User Story
As a user, I want to save evaluation results in JSON and HTML formats so that I can share results with my team and track improvements over time.

## Business Value
- Enables sharing results with non-technical stakeholders
- Provides data for tracking improvements over time
- Supports integration with CI/CD pipelines
- Creates audit trail of evaluation results

## Acceptance Criteria
- [ ] Save results automatically on completion
- [ ] Generate JSON report with:
  - [ ] Full metrics data
  - [ ] Evaluation metadata
  - [ ] Timestamp and duration
  - [ ] Summary statistics
- [ ] Generate HTML report with:
  - [ ] Visual summary dashboard
  - [ ] Sortable metrics table
  - [ ] Charts for key metrics
  - [ ] Mobile-responsive design
- [ ] Save to configurable output directory
- [ ] Include evaluator configuration in report

## Technical Notes
- Use serde_json for JSON generation
- Use template engine (askama or similar) for HTML
- Default output directory: `./preval-reports/`
- Filename format: `{evaluator}-{timestamp}.{json|html}`
- Include CSS inline for standalone HTML files

## Example JSON Structure
```json
{
  "evaluation": {
    "evaluator": "sentiment-eval",
    "started_at": "2024-01-15T10:30:00Z",
    "completed_at": "2024-01-15T10:35:30Z",
    "duration_seconds": 330
  },
  "summary": {
    "total_samples": 50,
    "successful_samples": 48,
    "failed_samples": 2,
    "average_accuracy": 0.876
  },
  "metrics": [
    {
      "sample_id": "email-001",
      "metrics": {
        "llm.eval.accuracy": 0.92,
        "llm.eval.latency": 142
      }
    }
  ]
}
```

## Definition of Done
- [ ] JSON reports contain all metrics data
- [ ] HTML reports are visually appealing
- [ ] Reports generate without errors
- [ ] Output directory is created if missing
- [ ] Integration test verifies report generation
- [ ] Review all tests and refactor to eliminate via type constraints where possible
- [ ] Audit and restrict visibility of all code to minimum required scope
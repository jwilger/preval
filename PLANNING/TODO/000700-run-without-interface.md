# Run Without Interface for CI/Automation

## User Story
As a CI/CD system or automation script, I want to run evaluations without any interactive interface and receive clear exit codes and file outputs so that I can integrate prompt evaluation into automated workflows and quality gates.

## Business Value
- Enables automated quality checks in CI pipelines
- Supports regression testing for prompts
- Allows scheduled evaluation runs
- Facilitates integration with monitoring systems
- Provides machine-readable outputs for further processing

## Acceptance Criteria
- [ ] --no-tui flag runs without any interface
- [ ] Exit code 0 for success, non-zero for failures
- [ ] Generate reports automatically without prompting
- [ ] Output minimal progress to stdout (CI-friendly)
- [ ] Support --quiet flag for no output except errors
- [ ] Create JSON summary file with key metrics
- [ ] Honor all command-line configuration options
- [ ] Complete gracefully even if not attached to TTY
- [ ] Support timeout parameter for CI environments

## Technical Notes
- Detect non-TTY environment and auto-enable no-tui mode
- Use structured logging that CI systems can parse
- Write reports to predictable locations
- Support environment variable configuration
- Handle signals appropriately for container environments
- Ensure all output is flushable for log aggregation

## Example CI Usage
```bash
# In CI pipeline
eval-tui --no-tui --output-dir=./test-results --timeout=20m

# Exit codes:
# 0 - All evaluations passed thresholds
# 1 - Some evaluations below threshold
# 2 - Evaluation error or timeout
# 3 - Configuration error

# Minimal output:
Starting evaluation: event_extraction
Progress: 10/50 samples complete (20%)
Progress: 20/50 samples complete (40%)
Progress: 30/50 samples complete (60%)
Progress: 40/50 samples complete (80%)
Progress: 50/50 samples complete (100%)
Evaluation complete. Report saved to: ./test-results/eval_20240115_143245.html
Summary saved to: ./test-results/eval_20240115_143245_summary.json

# Summary JSON format:
{
  "evaluation_type": "event_extraction",
  "timestamp": "2024-01-15T14:32:45Z",
  "duration_seconds": 754,
  "samples_evaluated": 50,
  "total_runs": 500,
  "metrics": {
    "count_accuracy": 0.865,
    "date_accuracy": 0.752,
    "quality_score": 0.891
  },
  "comparison": {
    "previous_run": "2024-01-14T10:15:00Z",
    "improvements": ["count_accuracy", "quality_score"],
    "regressions": ["date_accuracy"]
  },
  "passed": true
}
```

## Configuration
- Support .eval-tui.yml config file
- Override with environment variables (EVAL_TUI_*)
- Command-line flags take precedence
- Document all options clearly
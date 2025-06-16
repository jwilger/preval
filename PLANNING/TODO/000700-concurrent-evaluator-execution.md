# Support Multiple Evaluator Types

## User Story
As a developer working with various AI prompts, I want to run different types of evaluations through the same TUI interface so that I have a consistent experience regardless of what aspect of my system I'm testing.

## Business Value
- Single tool for all prompt evaluation needs
- Consistent user experience across different tests
- Easier onboarding for new evaluation types
- Shared infrastructure reduces maintenance
- Enables comprehensive testing workflows

## Acceptance Criteria
- [ ] Discover available evaluators automatically
- [ ] Support evaluators written in any language
- [ ] Display evaluator descriptions and capabilities
- [ ] Run multiple evaluator types in sequence
- [ ] Show combined results across all evaluations
- [ ] Handle different metrics for different types
- [ ] Support custom evaluator executables
- [ ] Validate evaluator compatibility before running
- [ ] Group related evaluators logically

## Technical Notes
- Define evaluator discovery mechanism (PATH, config file, directory)
- Each evaluator must support --info flag for metadata
- Evaluators must output standard JSON Lines protocol
- Support version checking for protocol compatibility
- Allow custom evaluator paths via --evaluator flag
- Cache evaluator discovery results
- Handle evaluators that require different resources

## Example Evaluator Discovery
```
# Evaluator provides metadata via --info flag
$ event-extraction-eval --info
{
  "name": "event_extraction",
  "version": "1.0.0",
  "description": "Extract events from text samples",
  "protocol_version": "1.0",
  "supported_modes": ["fast", "full"],
  "required_env": ["AWS_PROFILE"],
  "metrics": [
    {"name": "count_accuracy", "description": "Event count accuracy"},
    {"name": "date_accuracy", "description": "Date extraction accuracy"},
    {"name": "quality_score", "description": "Overall quality score"}
  ]
}
```

## Multi-Evaluator UI
```
┌─── Running Multiple Evaluations ─────────────────────────────┐
│                                                             │
│  ▼ event_extraction (23/50 samples)          ETA: 5m       │
│    Current: Sample 23 - Social media post                   │
│    Metrics: Count: 0.87  Date: 0.75  Quality: 0.89        │
│                                                             │
│  ▶ entity_recognition (pending)              ETA: 8m       │
│                                                             │
│  ▶ sentiment_analysis (pending)              ETA: 3m       │
│                                                             │
│  Overall Progress: 23/150 tasks (15%)        Total ETA: 16m │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

## Configuration Example
```yaml
# .preval.yml
evaluators:
  search_paths:
    - ~/.local/bin
    - /usr/local/bin
    - ./evaluators
  
  custom:
    my_custom_eval:
      command: "python /path/to/my_eval.py --json-output"
      description: "Custom evaluation for specific use case"
  
  groups:
    nlp:
      - event_extraction
      - entity_recognition
    quality:
      - sentiment_analysis
      - grammar_check

## Definition of Done
- [ ] Multiple evaluators run successfully
- [ ] Each evaluator's output is correctly displayed
- [ ] Protocol compatibility is validated
- [ ] Custom evaluator paths work
- [ ] Discovery mechanism is reliable
- [ ] Review all tests and refactor to eliminate via type constraints where possible
- [ ] Audit and restrict visibility of all code to minimum required scope
```
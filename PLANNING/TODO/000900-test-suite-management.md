# Test Suite Management

## User Story
As a user, I want to define and manage test suites that group multiple evaluators so that I can run comprehensive evaluations with a single command.

## Business Value
- Simplifies running multiple related evaluators
- Enables consistent testing across projects
- Supports different testing scenarios (quick, full, specific features)
- Facilitates team standardization

## Acceptance Criteria
- [ ] Support YAML/JSON configuration for test suites
- [ ] Define evaluator groups with:
  - [ ] Commands to run
  - [ ] Weights for aggregation
  - [ ] Dependencies between evaluators
  - [ ] Tags for categorization
- [ ] Run suite with: `preval suite production.yaml`
- [ ] Support suite templates/inheritance
- [ ] List available suites
- [ ] Validate suite configuration

## Technical Notes
- Store suites in `.preval/suites/` or specified directory
- Support environment variable substitution
- Allow conditional evaluator inclusion
- Handle evaluator failures gracefully
- Support parallel and sequential execution

## Example Suite Configuration
```yaml
name: production-suite
description: Full production prompt evaluation
parallel: true

evaluators:
  - name: customer-service
    command: python evals/customer_service.py
    weight: 2.0
    tags: [critical, customer-facing]
    env:
      MODEL: gpt-4
    
  - name: order-processing
    command: npm run eval:orders
    weight: 1.0
    tags: [backend]
    depends_on: []
    
  - name: recommendations
    command: ./evals/recommendations --mode full
    weight: 1.0
    tags: [ml, enhancement]
    continue_on_failure: true

aggregation:
  method: weighted_average
  failure_threshold: 0.85
  
reporting:
  format: [json, html]
  output_dir: ./reports/production/
```

## Definition of Done
- [ ] Suite configuration parsing works
- [ ] Evaluators run according to configuration
- [ ] Dependencies are respected
- [ ] Aggregation uses configured weights
- [ ] Example suites provided in docs
- [ ] Review all tests and refactor to eliminate via type constraints where possible
- [ ] Audit and restrict visibility of all code to minimum required scope
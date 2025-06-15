# Support Multiple Concurrent Evaluators

- Status: accepted
- Deciders: John Wilger
- Date: 2025-06-15
- Tags: architecture, scalability, features

## Context and Problem Statement

Users often need to evaluate multiple prompts or prompt systems in a single test run. Running evaluators sequentially would be time-consuming, especially for comprehensive test suites. Should PrEval support running multiple evaluators concurrently, and if so, how should it manage and display the results?

## Decision Drivers

- Time efficiency for comprehensive testing
- Need to test entire systems with multiple prompts
- Resource utilization on modern multi-core systems
- User experience for monitoring multiple evaluations
- Complexity of implementation and maintenance

## Considered Options

1. Single evaluator only (sequential runs)
2. Multiple evaluators with process-level concurrency
3. Multiple evaluators with thread pools
4. Distributed evaluation system

## Decision Outcome

Chosen option: "Multiple evaluators with process-level concurrency", because it provides good isolation between evaluators while being straightforward to implement. Each evaluator runs as a separate process, managed by PrEval using Tokio's async runtime.

### Positive Consequences

- Evaluators run concurrently, reducing total execution time
- Process isolation prevents evaluators from interfering with each other
- Natural mapping to Tokio's process management capabilities
- Can aggregate results across all evaluators for system-wide view
- Supports both CLI listing and configuration files

### Negative Consequences

- More complex UI to show multiple concurrent executions
- Higher memory usage with multiple processes
- Need to manage process lifecycle and cleanup
- Potential for resource contention

## Pros and Cons of the Options

### Single evaluator only

Run one evaluator at a time, users must launch multiple instances.

- Good, because simplest to implement
- Good, because easiest to understand
- Bad, because inefficient for multi-prompt systems
- Bad, because poor user experience for comprehensive testing
- Bad, because doesn't utilize available system resources

### Multiple evaluators with process-level concurrency

Each evaluator runs as a separate OS process, managed by PrEval.

- Good, because strong isolation between evaluators
- Good, because evaluators can be written in any language
- Good, because natural crash isolation
- Good, because straightforward process management with Tokio
- Bad, because higher memory overhead
- Bad, because more complex UI requirements

### Multiple evaluators with thread pools

Run evaluators in threads within the PrEval process.

- Good, because lower memory overhead
- Good, because potentially faster communication
- Bad, because requires thread-safe evaluators
- Bad, because one crash affects all evaluators
- Bad, because limits evaluator implementation languages

### Distributed evaluation system

Support running evaluators across multiple machines.

- Good, because unlimited scalability
- Good, because supports large-scale testing
- Bad, because massive complexity increase
- Bad, because requires network protocols
- Bad, because overkill for target use case

## Links

- Related to [Language-Agnostic Evaluator Interface](20250615-language-agnostic-evaluator-interface.md)
- Influences [Use Tokio for Async Process Management](20250615-use-tokio-for-async-process-management.md)
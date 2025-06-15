# Two-Phase Protocol with Handshake

- Status: accepted
- Deciders: John Wilger
- Date: 2025-06-15
- Tags: architecture, protocol, communication

## Context and Problem Statement

While OpenTelemetry provides excellent metrics reporting capabilities, it doesn't handle execution planning or progress tracking. PrEval needs to know upfront what the evaluator will do (number of samples, execution mode, etc.) to provide meaningful progress indicators and organize the UI appropriately.

## Decision Drivers

- Need for progress tracking in test suite mode
- Support for different execution modes (test suite, online collection, continuous)
- Ability to adapt UI based on what will be executed
- Clear communication of evaluator capabilities and intentions
- Maintaining separation between coordination and metrics data

## Considered Options

1. Pure OpenTelemetry with special metrics for coordination
2. Two-phase protocol: JSON handshake + OpenTelemetry metrics
3. Custom protocol that embeds OpenTelemetry
4. Multiple separate channels (stdin for commands, stdout for metrics)

## Decision Outcome

Chosen option: "Two-phase protocol: JSON handshake + OpenTelemetry metrics", because it cleanly separates concerns while maintaining simplicity. The handshake phase handles coordination and planning, while the metrics phase uses standard OpenTelemetry format.

### Positive Consequences

- Clear separation of concerns between coordination and metrics
- Evaluators can provide execution plans for better progress tracking
- UI can adapt based on execution mode
- Metrics remain pure OpenTelemetry without custom extensions
- Simple to implement in any language

### Negative Consequences

- Two different formats to parse
- Slightly more complex than a single protocol
- Need to maintain state between phases

## Pros and Cons of the Options

### Pure OpenTelemetry with special metrics

Use OpenTelemetry for everything, including coordination metrics.

- Good, because single protocol to implement
- Good, because all data in one format
- Bad, because abuses metrics for non-metric data
- Bad, because makes metrics data less pure
- Bad, because OpenTelemetry SDKs not designed for this

### Two-phase protocol: JSON handshake + OpenTelemetry metrics

Initial JSON handshake followed by OpenTelemetry metrics stream.

- Good, because clean separation of concerns
- Good, because each phase uses appropriate format
- Good, because easy to implement and understand
- Good, because allows rich coordination options
- Bad, because two formats to handle
- Bad, because slightly more complex protocol

### Custom protocol that embeds OpenTelemetry

Create a custom protocol that includes OpenTelemetry as a field.

- Good, because single consistent format
- Good, because can extend as needed
- Bad, because creates yet another protocol
- Bad, because more complex parsing
- Bad, because loses OpenTelemetry tool compatibility

### Multiple separate channels

Use stdin for sending commands, stdout for metrics, stderr for logs.

- Good, because clean separation of channels
- Good, because bidirectional communication possible
- Bad, because complex process management
- Bad, because harder to implement correctly
- Bad, because complicates evaluator implementation

## Links

- Refines [Use OpenTelemetry for Metrics Protocol](20250615-use-opentelemetry-for-metrics-protocol.md)
- Related to [Support Three Collection Modes](20250615-support-three-collection-modes.md)
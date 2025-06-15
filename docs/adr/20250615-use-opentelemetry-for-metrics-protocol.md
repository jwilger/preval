# Use OpenTelemetry for Metrics Protocol

- Status: accepted
- Deciders: John Wilger
- Date: 2025-06-15
- Tags: architecture, protocol, metrics, opentelemetry

## Context and Problem Statement

PrEval needs a protocol for evaluators to report metrics about prompt evaluation results. The initial design proposed a custom JSON Lines format, but this would be tightly coupled to specific evaluation use cases. We need a flexible, standardized protocol that can handle diverse evaluation scenarios while providing rich metrics capabilities.

## Decision Drivers

- Need for flexibility to support different types of evaluations (accuracy, latency, custom metrics)
- Desire for standardization to leverage existing tools and libraries
- Integration with production monitoring systems
- Support for both offline testing and online evaluation
- Rich ecosystem and language support

## Considered Options

1. Custom JSON Lines protocol
2. OpenTelemetry metrics format
3. Prometheus exposition format
4. StatsD protocol

## Decision Outcome

Chosen option: "OpenTelemetry metrics format", because it provides the best combination of standardization, flexibility, and ecosystem support. It allows users to define custom metrics while leveraging existing SDKs and tools.

### Positive Consequences

- Evaluators can use official OpenTelemetry SDKs in any language
- Seamless integration with existing observability infrastructure
- Support for rich metric types (gauges, counters, histograms)
- Ability to correlate offline tests with production metrics
- Future-proof as OpenTelemetry becomes the industry standard

### Negative Consequences

- More complex than a simple custom format
- Requires understanding of OpenTelemetry concepts
- May be overkill for very simple evaluations

## Pros and Cons of the Options

### Custom JSON Lines protocol

Originally proposed format with specific fields for samples, runs, and metrics.

- Good, because simple and tailored to our specific use case
- Good, because minimal learning curve
- Bad, because requires custom parsing and generation code
- Bad, because no ecosystem support
- Bad, because difficult to extend for new use cases

### OpenTelemetry metrics format

Industry-standard observability protocol with rich metrics support.

- Good, because extensive language SDK support
- Good, because integrates with existing monitoring tools
- Good, because supports complex metric types and attributes
- Good, because enables correlation between test and production data
- Bad, because more complex initial setup
- Bad, because larger payload size

### Prometheus exposition format

Simple text-based format for metrics exposition.

- Good, because simple and human-readable
- Good, because widely supported
- Bad, because limited to pull-based collection
- Bad, because no built-in support for traces or logs
- Bad, because less flexible than OpenTelemetry

### StatsD protocol

Lightweight protocol for sending metrics to aggregation services.

- Good, because very simple
- Good, because low overhead
- Bad, because limited metric types
- Bad, because no standard attributes/tags format
- Bad, because designed for real-time metrics, not batch reporting

## Links

- Refined by [Two-Phase Protocol with Handshake](20250615-two-phase-protocol-with-handshake.md)
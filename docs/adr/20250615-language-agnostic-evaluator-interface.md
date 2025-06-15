# Language-Agnostic Evaluator Interface

- Status: accepted
- Deciders: John Wilger
- Date: 2025-06-15
- Tags: architecture, protocol, interoperability

## Context and Problem Statement

PrEval needs to work with evaluators written in any programming language. Users should be able to write evaluators in Python, JavaScript, Go, or any other language that suits their needs. How should PrEval communicate with these diverse evaluators?

## Decision Drivers

- Support for any programming language
- Simple protocol that's easy to implement
- No complex dependencies or libraries required
- Works on all major operating systems
- Clear and debuggable communication

## Considered Options

1. Language-specific plugins/bindings
2. HTTP REST API
3. gRPC service interface
4. Simple stdout/stdin protocol

## Decision Outcome

Chosen option: "Simple stdout/stdin protocol", because it's universally supported and requires no special libraries. Any program that can write to stdout can be an evaluator.

### Positive Consequences

- Any language that can print to stdout can create an evaluator
- No networking complexity or security concerns
- Easy to debug by examining output
- Natural integration with command-line tools
- Can redirect output for testing

### Negative Consequences

- Limited to one-way communication
- No built-in request/response pattern
- Text-based protocol may have parsing overhead
- Need to handle buffering correctly

## Pros and Cons of the Options

### Language-specific plugins/bindings

Create plugins or bindings for each language.

- Good, because can provide rich, typed interfaces
- Good, because native performance
- Bad, because massive maintenance burden
- Bad, because limits supported languages
- Bad, because version compatibility issues

### HTTP REST API

Evaluators expose HTTP endpoints that PrEval calls.

- Good, because standard protocol
- Good, because bidirectional communication
- Bad, because requires HTTP server in evaluator
- Bad, because networking complexity
- Bad, because firewall/security concerns

### gRPC service interface

Use gRPC for structured communication.

- Good, because strongly typed protocol
- Good, because efficient binary protocol
- Bad, because requires gRPC libraries
- Bad, because complex for simple evaluators
- Bad, because not all languages have good gRPC support

### Simple stdout/stdin protocol

Evaluators write JSON lines to stdout, PrEval captures and parses.

- Good, because universally supported
- Good, because simple to implement
- Good, because easy to debug and test
- Good, because works with existing CLI tools
- Bad, because text parsing overhead
- Bad, because one-way communication only

## Links

- Supports [Use OpenTelemetry for Metrics Protocol](20250615-use-opentelemetry-for-metrics-protocol.md)
- Supports [Two-Phase Protocol with Handshake](20250615-two-phase-protocol-with-handshake.md)
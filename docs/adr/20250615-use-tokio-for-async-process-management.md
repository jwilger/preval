# Use Tokio for Async Process Management

- Status: accepted
- Deciders: John Wilger
- Date: 2025-06-15
- Tags: implementation, technology, concurrency

## Context and Problem Statement

PrEval needs to manage multiple evaluator processes concurrently while maintaining a responsive UI. We need an async runtime that can handle process spawning, I/O streaming, and concurrent operations efficiently.

## Decision Drivers

- Need to manage multiple child processes
- Real-time streaming of process output
- Non-blocking UI updates
- Cross-platform process handling
- Integration with Rust ecosystem

## Considered Options

1. Tokio async runtime
2. async-std runtime
3. Synchronous threading with std::thread
4. Custom event loop

## Decision Outcome

Chosen option: "Tokio async runtime", because it's the most mature and widely adopted async runtime in the Rust ecosystem with excellent process management support.

### Positive Consequences

- Battle-tested in production systems
- Excellent documentation and community support
- Built-in process spawning with tokio::process
- Efficient async I/O for streaming output
- Works well with Ratatui for async UI updates
- Rich ecosystem of compatible libraries

### Negative Consequences

- Additional complexity of async programming
- Compile-time overhead from macro expansions
- Learning curve for async Rust concepts

## Links

- Required by [Use Rust with Ratatui for TUI](20250615-use-rust-with-ratatui-for-tui.md)
- Enables [Support Multiple Concurrent Evaluators](20250615-support-multiple-concurrent-evaluators.md)
# Support Three Collection Modes

- Status: accepted
- Deciders: John Wilger
- Date: 2025-06-15
- Tags: features, architecture, ux

## Context and Problem Statement

PrEval needs to support different use cases: running test suites with known samples, collecting metrics from live systems for a fixed duration, and continuous monitoring without a predetermined end. How should we structure these different modes of operation?

## Decision Drivers

- Different use cases require different UI behaviors
- Progress tracking needs vary by mode
- User expectations differ for each scenario
- Need clear mental model for users
- Implementation complexity

## Considered Options

1. Single mode with configuration flags
2. Two modes: batch and streaming
3. Three distinct modes: test suite, online collection, continuous
4. Fully flexible mode defined by evaluator

## Decision Outcome

Chosen option: "Three distinct modes: test suite, online collection, continuous", because it provides clear mental models for users while covering all identified use cases. Each mode has distinct UI behavior and progress tracking.

### Positive Consequences

- Clear user expectations for each mode
- Optimized UI for each use case
- Appropriate progress indicators per mode
- Simple evaluator implementation
- Easy to document and explain

### Negative Consequences

- Three code paths to maintain
- Mode selection adds complexity
- Some edge cases might not fit neatly

## Pros and Cons of the Options

### Single mode with configuration flags

One mode with various flags to control behavior.

- Good, because single code path
- Good, because maximum flexibility
- Bad, because confusing for users
- Bad, because complex configuration
- Bad, because no clear mental model

### Two modes: batch and streaming

Separate batch processing from continuous streaming.

- Good, because simpler than three modes
- Good, because covers most use cases
- Bad, because online collection doesn't fit well
- Bad, because progress tracking is ambiguous
- Bad, because mixed metaphors

### Three distinct modes

Separate modes for test suite, online collection, and continuous monitoring.

- Good, because clear mental models
- Good, because optimized UX per mode
- Good, because matches real use cases
- Good, because easy to document
- Bad, because three implementations
- Bad, because mode selection overhead

### Fully flexible mode defined by evaluator

Let evaluator define arbitrary modes.

- Good, because maximum flexibility
- Good, because future-proof
- Bad, because inconsistent UX
- Bad, because complex protocol
- Bad, because hard to document

## Mode Definitions

### Test Suite Mode
- Known number of samples upfront
- Clear progress tracking (X of Y)
- Defined completion state
- Best for: regression testing, CI/CD

### Online Collection Mode
- Fixed duration collection
- Countdown timer progress
- Ends after specified time
- Best for: sampling production traffic

### Continuous Mode
- No predetermined end
- Elapsed time display
- Manual stop required
- Best for: monitoring, debugging

## Links

- Implements in [Two-Phase Protocol with Handshake](20250615-two-phase-protocol-with-handshake.md)
- Affects UI design in [Use Rust with Ratatui for TUI](20250615-use-rust-with-ratatui-for-tui.md)
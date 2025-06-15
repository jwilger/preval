# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Repository Overview

PrEval is a cross-platform Terminal User Interface (TUI) application written in Rust that provides an interactive interface for running and monitoring prompt evaluation tests. It's designed to be language-agnostic and work with any evaluator that outputs OpenTelemetry metrics data.

## Development Setup

**Rust Version**: Stable (latest)
**Dependency Management**: Nix flake for local development environment
**Environment**: direnv with .envrc for automatic environment loading

### Common Commands

- `nix develop` - Enter development shell with all dependencies
- `cargo test --workspace` - Run all tests
- `cargo run -- <evaluator>` - Run PrEval with evaluator
- `cargo fmt --all` - Format code
- `cargo clippy --workspace --all-targets` - Run linter
- `pre-commit install` - Install git hooks (when in nix shell)

### ADR (Architecture Decision Records) Commands

- `npm run adr:preview` - Preview ADR documentation locally (http://localhost:4004)
- `npm run adr:build` - Build static ADR documentation site
- View ADRs in `docs/adr/` directory
- Create new ADRs by copying template and following naming convention: `YYYYMMDD-title-with-dashes.md`

### Story Workflow Commands

- `ls PLANNING/todo/` - View available stories prioritized by filename
- `mv PLANNING/todo/00010-*.md PLANNING/doing/` - Start working on a story
- `mv PLANNING/doing/00010-*.md PLANNING/done/` - Mark story complete
- `ls PLANNING/doing/` - Check current work (should only be one story)

## Technical Architecture

### Core Technologies

- **Language**: Rust (latest stable)
- **TUI Framework**: Ratatui (with crossterm backend for cross-platform support)
- **Async Runtime**: Tokio for process management and concurrent operations
- **Serialization**: serde + serde_json for JSON parsing
- **CLI**: clap for argument parsing
- **Type Safety**: nutype for domain-specific types

### Key Dependencies (Cargo.toml)

```toml
[dependencies]
ratatui = "0.26"
crossterm = "0.27"
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
clap = { version = "4", features = ["derive"] }
anyhow = "1"
thiserror = "1"
tracing = "0.1"
tracing-subscriber = "0.3"
dirs = "5"
nutype = { version = "0.4", features = ["serde"] }
```

### Project Structure

```
src/
├── main.rs           # Entry point, CLI parsing
├── app.rs           # Main application state and logic
├── ui/
│   ├── mod.rs       # UI module exports
│   ├── layout.rs    # Layout calculations
│   ├── widgets/     # Custom widgets
│   └── views/       # Different view states
├── evaluator/
│   ├── mod.rs       # Evaluator process management
│   ├── process.rs   # Process spawning and monitoring
│   └── protocol.rs  # Handshake and OpenTelemetry types
├── state/
│   ├── mod.rs       # Application state
│   └── samples.rs   # Sample/run tracking
└── config.rs        # Configuration handling
```

## Architecture Principles

### Type-Driven Design

- Heavy use of algebraic types via Rust's type system
- Use `nutype` crate for all domain-specific types
- Primitive types only at system boundaries (terminal I/O, JSON parsing)
- Traits for converting domain types to external representations
- Make impossible scenarios impossible to represent

### Functional Core, Imperative Shell

- Purely functional core with side effects at boundaries
- Domain logic as pure functions with no side effects
- I/O operations (terminal, process spawning) isolated in shell layer
- Dependencies passed as function arguments for testability
- State transformations through immutable data structures

### Development Process

- **User Story Driven**: All development follows user stories in `PLANNING/` directory
- **Single Story WIP**: Never more than one story in `PLANNING/doing/` at a time
- **Vertical Slices**: Each story represents complete functionality from UI to process management
- **Production Ready**: Each story must be deployable with full tests before moving to next
- Strict TDD: red-green-refactor cycle
- Refactoring includes leveraging type system to prevent test failures
- Always consider if new domain types can eliminate error cases

### Story Management

- Stories are prioritized by 6-digit filename prefix (000010, 000020, etc.)
- Move stories: `mv PLANNING/todo/000010-*.md PLANNING/doing/` to start work
- Complete stories: `mv PLANNING/doing/000010-*.md PLANNING/done/` when deployed
- Each story has clear acceptance criteria and definition of done
- Stories build incrementally - complete dependencies before starting new stories

### Active Story Development

- **Subtask Tracking**: Maintain a "## Current Subtasks" section at the bottom of the active story
- Subtasks are ephemeral implementation details, not requirements
- Update subtasks frequently as work progresses:
  - Add new subtasks as discovered
  - Check off completed subtasks with `[x]`
  - Remove or modify subtasks as understanding evolves
  - Reorder subtasks based on implementation approach
- Subtasks help track progress but can change at any time
- Keep subtasks focused on current implementation work

## Development Guidelines

### Command-Line Interface

The CLI follows these semantics:

1. No arguments = Interactive TUI mode
2. Specifying type or --fast = Non-interactive mode (implies --no-tui)
3. --no-tui = Explicit non-interactive mode for CI

### Two-Phase Protocol

The evaluator protocol uses two phases:

1. **Handshake**: Initial JSON message with evaluation plan
2. **Metrics**: OpenTelemetry metrics data in JSON Lines format

Example handshake structure:
```rust
#[derive(Serialize, Deserialize)]
pub struct Handshake {
    #[serde(rename = "type")]
    pub msg_type: String,  // Always "handshake"
    pub mode: EvaluationMode,  // test_suite, online_collection, continuous
    pub version: String,
    pub evaluator: EvaluatorInfo,
    pub execution_plan: Option<ExecutionPlan>,
    pub metrics_schema: Vec<MetricDefinition>,
}
```

Metrics use standard OpenTelemetry OTLP/JSON format with one ResourceMetrics object per line.

### Cross-Platform Considerations

1. **Terminal Handling**: Use crossterm for consistent behavior across platforms
2. **Path Handling**: Always use `std::path::Path` and `PathBuf`
3. **Process Spawning**: Use `tokio::process::Command` with proper shell handling
4. **Home Directory**: Use the `dirs` crate for finding user directories

### Windows-Specific Notes

- Ensure ANSI color support detection
- Handle different line endings (CRLF vs LF)
- Use `cmd.exe /C` for shell commands on Windows

### State Management

The application uses a message-passing architecture:

1. UI events generate `Action` enums
2. Actions update the central `AppState`
3. State changes trigger UI redraws
4. Evaluator output updates state asynchronously

### Error Handling

- Use `anyhow::Result` for main application errors
- Use `thiserror` for domain-specific error types
- Always provide context with `.context("...")`
- Handle process termination gracefully

### Testing Approach

- Unit tests for protocol parsing and state management
- Integration tests with mock evaluators
- Manual testing on all target platforms

## User Story Implementation

When implementing user stories from PLANNING/:

1. Move the story file from TODO/ to DOING/
2. Create a feature branch named after the story number
3. Implement incrementally, keeping the app buildable
4. Move to DONE/ when complete and merged

## Build and Release

### Local Development

```bash
# Run tests
cargo test

# Check all targets
cargo check --all-targets

# Run PrEval
cargo run -- <evaluator-command>
```

### Cross-Compilation

```bash
# Install cross
cargo install cross

# Build for Linux
cross build --release --target x86_64-unknown-linux-gnu

# Build for Windows
cross build --release --target x86_64-pc-windows-gnu
```

### Release Process

1. Update version in Cargo.toml
2. Update CHANGELOG.md
3. Tag the release
4. GitHub Actions will build and upload binaries

## Important Implementation Notes

1. **Process Communication**: Always use unbuffered stdout for real-time updates
2. **UI Responsiveness**: Run evaluator I/O in separate Tokio tasks
3. **Graceful Shutdown**: Handle Ctrl+C properly, kill child processes
4. **Progress Calculation**: Track at both sample and overall levels
5. **History Storage**: Store in `~/.local/share/preval/` (or platform equivalent)

## Performance Considerations

- Use `Arc<Mutex<>>` sparingly - prefer message passing
- Batch UI updates to avoid excessive redraws
- Stream large outputs instead of buffering
- Use `SmallVec` for small collections

## Accessibility

- Support standard terminal screen readers
- Provide alternative text output modes
- Ensure keyboard-only navigation
- Follow terminal color scheme preferences

## Git and Commit Practices

- Always make high-quality git commits that explain the _why_ not just the how
- Commit whenever all tests are passing rather than waiting to complete a full story
- Use conventional commit format when appropriate
- Keep commits focused and atomic

## Code Quality Guidelines

- Never use `#[allow(...)]` annotations to bypass compiler/linter rules without explicit permission on a case-by-case basis
- All code must pass `cargo fmt` and `cargo clippy` before committing
- Write tests for all new functionality
- Document public APIs with rustdoc comments
- Use descriptive variable and function names

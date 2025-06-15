# Initialize Rust Project with Dependencies

## User Story
As a developer, I want to set up the initial Rust project structure with all necessary dependencies so that I can begin implementing PrEval features.

## Business Value
- Establishes foundation for all future development
- Ensures consistent development environment
- Enables immediate progress on core features

## Acceptance Criteria
- [ ] Create Cargo.toml with initial dependencies:
  - [ ] ratatui for TUI framework
  - [ ] crossterm for terminal handling
  - [ ] tokio with full features for async runtime
  - [ ] serde and serde_json for JSON parsing
  - [ ] clap for CLI argument parsing
  - [ ] anyhow and thiserror for error handling
  - [ ] tracing and tracing-subscriber for logging
  - [ ] dirs for cross-platform directory handling
- [ ] Create basic project structure following architecture:
  - [ ] src/main.rs with CLI parsing
  - [ ] src/app.rs for application state
  - [ ] src/ui/mod.rs for UI components
  - [ ] src/evaluator/mod.rs for process management
  - [ ] src/protocol/mod.rs for handshake and OpenTelemetry parsing
- [ ] Implement minimal CLI that accepts evaluator command
- [ ] Add basic error handling and logging setup
- [ ] Ensure project builds and runs (shows help message)

## Technical Notes
- Use workspace structure to support future expansion
- Set up proper module organization from the start
- Configure clippy and fmt rules in Cargo.toml
- Add basic tracing subscriber for debugging

## Definition of Done
- [ ] Project builds with `cargo build`
- [ ] All tests pass with `cargo test`
- [ ] Code passes `cargo fmt` and `cargo clippy`
- [ ] Can run `cargo run -- --help` and see usage information
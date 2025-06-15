# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Repository Overview

eval-tui is a cross-platform Terminal User Interface (TUI) application written in Rust that provides an interactive interface for running and monitoring prompt evaluation tests. It's designed to be language-agnostic and work with any evaluator that outputs JSON Lines format.

## Technical Architecture

### Core Technologies
- **Language**: Rust (latest stable)
- **TUI Framework**: Ratatui (with crossterm backend for cross-platform support)
- **Async Runtime**: Tokio for process management and concurrent operations
- **Serialization**: serde + serde_json for JSON parsing
- **CLI**: clap for argument parsing

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
tracing = "0.1"
tracing-subscriber = "0.3"
dirs = "5"
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
│   └── protocol.rs  # JSON protocol types
├── state/
│   ├── mod.rs       # Application state
│   └── samples.rs   # Sample/run tracking
└── config.rs        # Configuration handling
```

## Development Guidelines

### Command-Line Interface
The CLI follows these semantics:
1. No arguments = Interactive TUI mode
2. Specifying type or --fast = Non-interactive mode (implies --no-tui)
3. --no-tui = Explicit non-interactive mode for CI

### JSON Lines Protocol
The evaluator protocol uses JSON Lines (one JSON object per line):
```rust
#[derive(Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum EvaluatorMessage {
    Init { total_samples: usize, runs_per_sample: usize },
    SampleStart { sample_id: usize, expected_events: usize },
    RunStart { sample_id: usize, run_id: usize },
    RunComplete { sample_id: usize, run_id: usize, status: RunStatus, metrics: Metrics },
    SampleComplete { sample_id: usize, summary: SampleSummary },
    OverallUpdate { metrics: OverallMetrics, vs_previous: Option<Comparison> },
    Log { level: LogLevel, message: String },
    Complete { final_metrics: FinalMetrics },
}
```

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
# Run with auto-reload
cargo watch -x run

# Run tests
cargo test

# Check all targets
cargo check --all-targets
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
5. **History Storage**: Store in `~/.local/share/eval-tui/` (or platform equivalent)

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
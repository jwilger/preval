# eval-tui

A cross-platform Terminal User Interface (TUI) for running and monitoring prompt evaluation tests. This tool provides an interactive interface for executing evaluation scripts, monitoring their progress in real-time, and analyzing results.

## Overview

`eval-tui` is designed to work with any prompt evaluation system that outputs structured JSON Lines format. It provides:

- Real-time progress monitoring with hierarchical sample/run display
- Live comparison of current results vs. previous runs
- Interactive exploration of evaluation details
- Cross-platform support (Linux, macOS, Windows)
- Both interactive TUI and non-interactive CI modes

## Installation

### Pre-built Binaries

Download the latest release for your platform from the [Releases](https://github.com/johnwilger/eval-tui/releases) page.

### Building from Source

```bash
# Clone the repository
git clone https://github.com/johnwilger/eval-tui.git
cd eval-tui

# Build for your current platform
cargo build --release

# Build for all platforms (requires cross-compilation setup)
make release-all
```

## Usage

### Interactive Mode (Default)

```bash
# Launch the TUI for interactive evaluation selection
eval-tui

# With specific AWS profile
eval-tui --aws-profile nh-dev
```

In interactive mode, you can:
- Select evaluation type(s) to run
- Choose between Fast (3 samples) or Full mode
- Navigate results with keyboard shortcuts
- Save HTML reports on completion

### Non-Interactive Mode

When you specify evaluation options via command line, the TUI is automatically disabled:

```bash
# Run specific evaluation type without TUI
eval-tui event_extraction

# Run in fast mode (3 samples, no TUI)
eval-tui --fast

# Run specific type in fast mode
eval-tui event_extraction --fast

# Explicit non-interactive mode for CI
eval-tui --no-tui
```

### Custom Evaluators

```bash
# Run a custom evaluator command
eval-tui --evaluator "python my_custom_eval.py --json-output"
```

## Architecture

The system follows a simple process model:

```
┌─────────────┐     JSON Lines      ┌──────────────┐
│   eval-tui  │ ← ─ ─ ─ ─ ─ ─ ─ ─ ─ │  Evaluator   │
│   (Rust)    │     via stdout       │  (Any Lang)  │
└─────────────┘                      └──────────────┘
      │
      ├── Spawns process
      ├── Parses output
      ├── Updates UI
      └── Handles interaction
```

## Evaluator Protocol

Evaluators must output JSON Lines format (one JSON object per line) to stdout:

```jsonl
{"type": "init", "total_samples": 50, "runs_per_sample": 10}
{"type": "sample_start", "sample_id": 1, "expected_events": 3}
{"type": "run_start", "sample_id": 1, "run_id": 1}
{"type": "run_complete", "sample_id": 1, "run_id": 1, "status": "success", "metrics": {...}}
{"type": "sample_complete", "sample_id": 1, "summary": {...}}
{"type": "overall_update", "metrics": {...}, "vs_previous": {...}}
{"type": "log", "level": "info", "message": "Processing sample..."}
{"type": "complete", "final_metrics": {...}}
```

See [PROTOCOL.md](PROTOCOL.md) for complete protocol documentation.

## Keyboard Shortcuts

In the TUI:
- `↑/↓` - Navigate samples
- `Enter` - Expand/collapse sample details
- `Space` - View detailed sample information
- `r` - View specific run details
- `p` - Pause/resume evaluation
- `s` - Save current results
- `q` - Quit (with confirmation if running)

## Building for Multiple Platforms

The project supports cross-compilation for:
- `x86_64-unknown-linux-gnu`
- `x86_64-apple-darwin`
- `aarch64-apple-darwin` (Apple Silicon)
- `x86_64-pc-windows-msvc`

Using GitHub Actions for automated releases is recommended. See `.github/workflows/release.yml` for the build configuration.

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

See [CONTRIBUTING.md](CONTRIBUTING.md) for detailed guidelines.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
# PrEval

A cross-platform Terminal User Interface (TUI) for running and monitoring prompt evaluation tests. This tool provides an interactive interface for executing evaluation scripts, monitoring their progress in real-time, and analyzing results.

## Overview

PrEval is designed to work with any prompt evaluation system that outputs OpenTelemetry metrics data. It provides:

- Real-time progress monitoring with OpenTelemetry metrics display
- Support for multiple concurrent evaluators
- Test suite, online collection, and continuous monitoring modes
- Cross-platform support (Linux, macOS, Windows)
- Both interactive TUI and non-interactive CI modes

## Architecture Documentation

This project uses Architecture Decision Records (ADRs) to document important design decisions. You can browse our ADRs:

- [View ADRs locally](http://localhost:4004) - Run `npm run adr:preview` after setting up the development environment
- [View ADRs on GitHub Pages](https://johnwilger.github.io/preval/adr/) (once published)

Key architectural decisions include:
- Using OpenTelemetry for metrics protocol
- Two-phase handshake protocol for coordination
- Support for multiple concurrent evaluators
- Language-agnostic evaluator interface

## Installation

### Pre-built Binaries

Download the latest release for your platform from the [Releases](https://github.com/johnwilger/preval/releases) page.

### Building from Source

```bash
# Clone the repository
git clone https://github.com/johnwilger/preval.git
cd preval

# Build for your current platform
cargo build --release

# Build for all platforms (requires cross-compilation setup)
make release-all
```

## Usage

### Interactive Mode (Default)

```bash
# Launch PrEval for interactive evaluation selection
preval

# With specific AWS profile
preval --aws-profile my-profile
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
preval event_extraction

# Run in fast mode (3 samples, no TUI)
preval --fast

# Run specific type in fast mode
preval event_extraction --fast

# Explicit non-interactive mode for CI
preval --no-tui
```

### Custom Evaluators

```bash
# Run a custom evaluator command
preval --evaluator "python my_custom_eval.py --json-output"
```

## Architecture

The system follows a simple process model:

```
┌─────────────┐     JSON Lines      ┌──────────────┐
│   PrEval    │ ← ─ ─ ─ ─ ─ ─ ─ ─ ─ │  Evaluator   │
│   (Rust)    │     via stdout       │  (Any Lang)  │
└─────────────┘                      └──────────────┘
      │
      ├── Spawns process
      ├── Parses output
      ├── Updates UI
      └── Handles interaction
```

## Evaluator Protocol

PrEval uses a two-phase protocol for communication with evaluators:

1. **Handshake Phase**: Initial JSON message describing the evaluation plan
2. **Metrics Phase**: OpenTelemetry metrics data in JSON Lines format

Example handshake:
```json
{
  "type": "handshake",
  "mode": "test_suite",
  "evaluator": {"name": "my-eval", "description": "My evaluation"},
  "execution_plan": {"total_samples": 50}
}
```

Example metrics (OTLP/JSON format):
```jsonl
{"resourceMetrics":[{"resource":{"attributes":[{"key":"service.name","value":{"stringValue":"my-eval"}}]},"scopeMetrics":[{"metrics":[{"name":"llm.eval.accuracy","gauge":{"dataPoints":[{"asDouble":0.92,"attributes":[{"key":"sample.id","value":{"stringValue":"001"}}]}]}}]}]}]}
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

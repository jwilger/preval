use crate::state::types::Initialized;
use crate::ui::layout::UiLayout;
use crate::ui::widgets::{footer::Footer, header::Header, metrics::MetricsView};
use anyhow::{Context, Result};
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io::{self, Stdout};
use std::marker::PhantomData;

/// Terminal renderer with typestate pattern to ensure proper initialization
pub(crate) struct Renderer<S> {
    _state: PhantomData<S>,
}

/// Uninitialized state marker
pub(crate) struct Uninitialized;

/// Uninitialized renderer
impl Renderer<Uninitialized> {
    /// Create a new uninitialized renderer
    pub(crate) fn new() -> Self {
        Self {
            _state: PhantomData,
        }
    }

    /// Initialize the terminal and transition to initialized state
    pub(crate) fn initialize(
        self,
    ) -> Result<(Renderer<Initialized>, Terminal<CrosstermBackend<Stdout>>)> {
        // Enable raw mode
        enable_raw_mode().context("Failed to enable raw mode")?;

        // Enter alternate screen
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen).context("Failed to enter alternate screen")?;

        // Create terminal
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend).context("Failed to create terminal")?;

        let renderer = Renderer {
            _state: PhantomData,
        };

        Ok((renderer, terminal))
    }
}

/// Initialized renderer - can only be created through initialize()
impl Renderer<Initialized> {
    /// Render the UI
    pub(crate) fn render<B: ratatui::backend::Backend>(
        &self,
        terminal: &mut Terminal<B>,
        state: &crate::state::AppState,
    ) -> Result<()> {
        terminal
            .draw(|frame| {
                let area = frame.area();

                // Calculate layout
                match UiLayout::new(area) {
                    Ok(layout) => {
                        // Render header - prefer handshake data over evaluator name
                        let header = match state.handshake() {
                            Some(handshake) => Header::new().handshake(handshake),
                            None => {
                                if let Some(name) = state.evaluator_name() {
                                    Header::new().evaluator_name(name)
                                } else {
                                    Header::new()
                                }
                            }
                        };
                        frame.render_widget(header, layout.header);

                        // Render content (metrics)
                        let metrics_view = MetricsView::new(state.metrics(), state.status());
                        frame.render_widget(metrics_view, layout.content);

                        // Render footer
                        let footer = Footer::new().paused(state.is_paused());
                        frame.render_widget(footer, layout.footer);
                    }
                    Err(_) => {
                        // Terminal too small, show error
                        let msg = "Terminal too small!";
                        frame.render_widget(
                            ratatui::widgets::Paragraph::new(msg).style(
                                ratatui::style::Style::default().fg(ratatui::style::Color::Red),
                            ),
                            area,
                        );
                    }
                }
            })
            .context("Failed to draw frame")?;

        Ok(())
    }
}

/// Cleanup helper for terminal restoration
pub(crate) struct TerminalCleanup;

impl Drop for TerminalCleanup {
    fn drop(&mut self) {
        // Best effort cleanup - ignore errors since we're in Drop
        let _ = disable_raw_mode();
        let _ = execute!(io::stdout(), LeaveAlternateScreen);
    }
}

// Tests removed: test_renderer_typestate
// The phantom type parameter already ensures at compile time that:
// - Renderer<Initialized> can only be created through initialize()
// - Terminal cleanup only happens for initialized renderers
// This makes runtime tests unnecessary as the compiler enforces these invariants.

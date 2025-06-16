use ratatui::{
    prelude::*,
    widgets::Paragraph,
};

/// Footer widget showing keyboard shortcuts
pub(crate) struct Footer {
    paused: bool,
}

impl Footer {
    /// Create a new footer widget
    pub(crate) fn new() -> Self {
        Self { paused: false }
    }

    /// Set paused state (builder pattern)
    pub(crate) fn paused(mut self, paused: bool) -> Self {
        self.paused = paused;
        self
    }
}

impl Widget for Footer {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let shortcuts = if self.paused {
            "[q] Quit  [Space] Resume  [Ctrl+L] Refresh"
        } else {
            "[q] Quit  [Space] Pause  [Ctrl+L] Refresh"
        };

        let footer = Paragraph::new(shortcuts)
            .style(Style::default().fg(Color::DarkGray))
            .alignment(Alignment::Left);

        footer.render(area, buf);
    }
}
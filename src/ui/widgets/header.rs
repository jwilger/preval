use crate::state::types::EvaluatorName;
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph},
};

/// Header widget showing app name and evaluator
pub(crate) struct Header<'a> {
    evaluator_name: Option<&'a EvaluatorName>,
}

impl<'a> Header<'a> {
    /// Create a new header widget
    pub(crate) fn new() -> Self {
        Self {
            evaluator_name: None,
        }
    }

    /// Set the evaluator name (builder pattern)
    pub(crate) fn evaluator_name(mut self, name: &'a EvaluatorName) -> Self {
        self.evaluator_name = Some(name);
        self
    }
}

impl<'a> Widget for Header<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = match self.evaluator_name {
            Some(name) => format!("PrEval - {}", name),
            None => "PrEval".to_string(),
        };

        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Cyan))
            .border_type(ratatui::widgets::BorderType::Rounded);

        let paragraph = Paragraph::new(title)
            .block(block)
            .style(Style::default().fg(Color::White).add_modifier(Modifier::BOLD))
            .alignment(Alignment::Center);

        paragraph.render(area, buf);
    }
}
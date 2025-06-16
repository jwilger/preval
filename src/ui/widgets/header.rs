use crate::evaluator::protocol::ValidatedHandshake;
use crate::state::types::EvaluatorName;
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph},
};

/// Header widget showing app name and evaluator
pub(crate) struct Header<'a> {
    evaluator_name: Option<&'a EvaluatorName>,
    handshake: Option<&'a ValidatedHandshake>,
}

impl<'a> Header<'a> {
    /// Create a new header widget
    pub(crate) fn new() -> Self {
        Self {
            evaluator_name: None,
            handshake: None,
        }
    }

    /// Set the evaluator name (builder pattern)
    pub(crate) fn evaluator_name(mut self, name: &'a EvaluatorName) -> Self {
        self.evaluator_name = Some(name);
        self
    }

    /// Set the handshake data (builder pattern)
    pub(crate) fn handshake(mut self, handshake: &'a ValidatedHandshake) -> Self {
        self.handshake = Some(handshake);
        self
    }
}

impl<'a> Widget for Header<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Build title from handshake or evaluator name
        let (title, subtitle) = match self.handshake {
            Some(handshake) => {
                let title = format!("PrEval - {}", handshake.evaluator.name);
                let subtitle = match &handshake.evaluator.description {
                    Some(desc) => format!(
                        "{}  •  Protocol v{}",
                        desc.as_ref(),
                        handshake.version.as_ref()
                    ),
                    None => format!("Protocol v{}", handshake.version.as_ref()),
                };
                (title, Some(subtitle))
            }
            None => {
                let title = match self.evaluator_name {
                    Some(name) => format!("PrEval - {}", name),
                    None => "PrEval".to_string(),
                };
                (title, None)
            }
        };

        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Cyan))
            .border_type(ratatui::widgets::BorderType::Rounded);

        // Create text with title and optional subtitle
        let text = match subtitle {
            Some(sub) => Text::from(vec![
                Line::from(title).style(
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD),
                ),
                Line::from(sub).style(Style::default().fg(Color::Gray)),
            ]),
            None => Text::from(title).style(
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
        };

        let paragraph = Paragraph::new(text)
            .block(block)
            .alignment(Alignment::Center);

        paragraph.render(area, buf);
    }
}

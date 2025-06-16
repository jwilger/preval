use crate::state::{types::SampleStatus, AppState};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Gauge, List, ListItem, Paragraph},
};
use std::fmt::Write as _;

/// Progress display widget showing real-time evaluation progress
pub(crate) struct ProgressView<'a> {
    state: &'a AppState,
}

impl<'a> ProgressView<'a> {
    /// Create a new progress view
    pub(crate) fn new(state: &'a AppState) -> Self {
        Self { state }
    }

    /// Format duration as MM:SS
    fn format_duration(duration: std::time::Duration) -> String {
        let total_seconds = duration.as_secs();
        let minutes = total_seconds / 60;
        let seconds = total_seconds % 60;
        format!("{}:{:02}", minutes, seconds)
    }

    /// Format a sample result for display
    fn format_sample_result(&self, sample: &crate::state::types::SampleResult) -> String {
        let status_icon = match &sample.status {
            SampleStatus::Processing => "⟳",
            SampleStatus::Completed => "✓",
            SampleStatus::Failed(_) => "✗",
        };

        let mut line = format!("{} {}", status_icon, sample.sample_id);

        // Add key metrics (limit to 2-3 most important ones)
        if !sample.metrics.is_empty() {
            let mut metrics_str = String::new();
            for (i, (name, value)) in sample.metrics.iter().take(3).enumerate() {
                if i > 0 {
                    metrics_str.push_str(", ");
                }
                write!(&mut metrics_str, "{}={:.2}", name, value).ok();
            }
            line.push_str(&format!(": {}", metrics_str));
        }

        // Add error message for failed samples
        if let SampleStatus::Failed(error) = &sample.status {
            line.push_str(&format!(" ({})", error));
        }

        line
    }
}

impl<'a> Widget for ProgressView<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Split the area into sections
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(0)
            .constraints([
                Constraint::Length(3), // Progress bar
                Constraint::Length(3), // Current sample
                Constraint::Min(5),    // Recent samples
                Constraint::Length(3), // Summary
            ])
            .split(area);

        // Render progress bar section
        self.render_progress_bar(chunks[0], buf);

        // Render current sample section
        self.render_current_sample(chunks[1], buf);

        // Render recent samples section
        self.render_recent_samples(chunks[2], buf);

        // Render summary section
        self.render_summary(chunks[3], buf);
    }
}

impl<'a> ProgressView<'a> {
    /// Render the progress bar with completion percentage and ETA
    fn render_progress_bar(&self, area: Rect, buf: &mut Buffer) {
        let (completed, total, percentage) = self.state.progress();

        let title = match total {
            Some(t) => format!("Progress: {}/{} samples ({:.1}%)", completed, t, percentage),
            None => format!("Progress: {} samples", completed),
        };

        // Add ETA if available
        let title_with_eta = if let Some(eta) = self.state.calculate_eta() {
            format!("{} - ETA: {}", title, Self::format_duration(eta))
        } else {
            title
        };

        let progress_ratio = if percentage > 0.0 {
            percentage / 100.0
        } else {
            0.0
        };

        let gauge = Gauge::default()
            .block(Block::default().borders(Borders::ALL).title(title_with_eta))
            .gauge_style(Style::default().fg(Color::Cyan))
            .ratio(progress_ratio);

        Widget::render(gauge, area, buf);
    }

    /// Render current sample being processed
    fn render_current_sample(&self, area: Rect, buf: &mut Buffer) {
        let current_text = match self.state.current_sample() {
            Some(sample_id) => format!("Current: {} (processing...)", sample_id),
            None => "Current: (none)".to_string(),
        };

        let paragraph = Paragraph::new(current_text)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Current Sample"),
            )
            .style(Style::default().fg(Color::Yellow));

        Widget::render(paragraph, area, buf);
    }

    /// Render recent completed samples
    fn render_recent_samples(&self, area: Rect, buf: &mut Buffer) {
        let recent_samples = self.state.recent_samples();

        let mut items =
            vec![ListItem::new("Recent Samples:")
                .style(Style::default().add_modifier(Modifier::BOLD))];

        if recent_samples.is_empty() {
            items.push(
                ListItem::new("  No samples completed yet...")
                    .style(Style::default().fg(Color::DarkGray)),
            );
        } else {
            // Show recent samples in reverse order (most recent first)
            for sample in recent_samples.iter().rev() {
                let line = self.format_sample_result(sample);
                let style = match &sample.status {
                    SampleStatus::Completed => Style::default().fg(Color::Green),
                    SampleStatus::Failed(_) => Style::default().fg(Color::Red),
                    SampleStatus::Processing => Style::default().fg(Color::Yellow),
                };
                items.push(ListItem::new(format!("  {}", line)).style(style));
            }
        }

        let list = List::new(items).block(Block::default().borders(Borders::ALL));

        Widget::render(list, area, buf);
    }

    /// Render summary statistics
    fn render_summary(&self, area: Rect, buf: &mut Buffer) {
        let (failed_count, total_completed, success_rate) = self.state.summary_stats();
        let elapsed = Self::format_duration(self.state.elapsed_time());

        let summary_text = if total_completed > 0 {
            format!(
                "Summary: {}/{} failed ({:.1}% success rate) | Elapsed: {}",
                failed_count, total_completed, success_rate, elapsed
            )
        } else {
            format!("Summary: No samples completed | Elapsed: {}", elapsed)
        };

        let summary_style = if failed_count > 0 {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default().fg(Color::Green)
        };

        let paragraph = Paragraph::new(summary_text)
            .block(Block::default().borders(Borders::ALL).title("Summary"))
            .style(summary_style);

        Widget::render(paragraph, area, buf);
    }
}

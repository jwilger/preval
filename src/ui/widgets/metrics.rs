use crate::state::metrics::{AttributeKey, AttributeValue, Metric, MetricData};
use crate::state::types::EvaluationStatus;
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, List, ListItem},
};
use std::fmt::Write as _;

/// Metrics display widget
#[allow(dead_code)] // Used in future stories
pub(crate) struct MetricsView<'a> {
    metrics: &'a [MetricData],
    status: &'a EvaluationStatus,
}

impl<'a> MetricsView<'a> {
    /// Create a new metrics view
    #[allow(dead_code)] // Used in future stories
    pub(crate) fn new(metrics: &'a [MetricData], status: &'a EvaluationStatus) -> Self {
        Self { metrics, status }
    }

    /// Format a metric value for display
    #[allow(dead_code)] // Used in future stories
    fn format_metric_line(&self, metric: &Metric) -> Vec<String> {
        let mut lines = Vec::new();

        match metric {
            Metric::Gauge {
                name,
                data_points,
                unit,
            } => {
                for point in data_points {
                    let mut line = format!("  {}: {:.2}", name, point.value.value());

                    if let Some(unit) = unit {
                        write!(&mut line, " {}", unit).ok();
                    }

                    // Add sample ID if present
                    if let Some(sample_id) = self.get_sample_id(&point.attributes) {
                        write!(&mut line, " (sample: {})", sample_id).ok();
                    }

                    lines.push(line);
                }
            }
            Metric::Counter {
                name,
                data_points,
                unit,
            } => {
                for point in data_points {
                    let mut line = format!("  {}: {:.0}", name, point.value.value());

                    if let Some(unit) = unit {
                        write!(&mut line, " {}", unit).ok();
                    }

                    // Add sample ID if present
                    if let Some(sample_id) = self.get_sample_id(&point.attributes) {
                        write!(&mut line, " (sample: {})", sample_id).ok();
                    }

                    lines.push(line);
                }
            }
            Metric::Histogram {
                name,
                data_points,
                unit,
            } => {
                for point in data_points {
                    let avg = if point.value.count > 0 {
                        point.value.sum.unwrap_or(0.0) / point.value.count as f64
                    } else {
                        0.0
                    };

                    let mut line = format!("  {}: {:.0}", name, avg);

                    if let Some(unit) = unit {
                        write!(&mut line, "{}", unit).ok();
                    }

                    // Add sample ID if present
                    if let Some(sample_id) = self.get_sample_id(&point.attributes) {
                        write!(&mut line, " (sample: {})", sample_id).ok();
                    }

                    lines.push(line);
                }
            }
        }

        lines
    }

    /// Extract sample ID from attributes
    #[allow(dead_code)] // Used in future stories
    fn get_sample_id<'b>(
        &self,
        attributes: &'b std::collections::HashMap<AttributeKey, AttributeValue>,
    ) -> Option<&'b str> {
        // Try to find sample.id attribute
        for (key, value) in attributes {
            if key.as_ref() == "sample.id" {
                if let AttributeValue::StringValue(s) = value {
                    return Some(s.as_str());
                }
            }
        }
        None
    }
}

impl<'a> Widget for MetricsView<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Gray));

        // Collect all metric lines
        let mut items =
            vec![ListItem::new("Metrics:").style(Style::default().add_modifier(Modifier::BOLD))];

        if self.metrics.is_empty() {
            items.push(
                ListItem::new("  No metrics received yet...")
                    .style(Style::default().fg(Color::DarkGray)),
            );
        } else {
            // Show latest metrics
            for metric_data in self.metrics.iter().rev().take(10) {
                for metric in &metric_data.metrics {
                    for line in self.format_metric_line(metric) {
                        items.push(ListItem::new(line));
                    }
                }
            }
        }

        // Add status line
        items.push(ListItem::new(""));
        let status_line = match self.status {
            EvaluationStatus::Starting => "Status: Starting evaluator...".to_string(),
            EvaluationStatus::WaitingForHandshake => "Status: Waiting for handshake...".to_string(),
            EvaluationStatus::CollectingMetrics { received, total } => match total {
                Some(t) => format!("Status: Collecting metrics... ({}/{})", received, t),
                None => format!("Status: Collecting metrics... ({})", received),
            },
            EvaluationStatus::Completed => "Status: Evaluation completed".to_string(),
            EvaluationStatus::Failed(err) => format!("Status: Failed - {}", err),
        };

        let status_style = match self.status {
            EvaluationStatus::Failed(_) => Style::default().fg(Color::Red),
            EvaluationStatus::Completed => Style::default().fg(Color::Green),
            _ => Style::default().fg(Color::Yellow),
        };

        items.push(ListItem::new(status_line).style(status_style));

        let list = List::new(items).block(block);
        Widget::render(list, area, buf);
    }
}

use crate::evaluator::{
    handshake::parse_handshake,
    parser::parse_metrics_line,
    process::{EvaluatorMessage, EvaluatorProcess},
};
use crate::state::{
    types::{EvaluationStatus, EvaluatorCommand, EvaluatorName, UiAction},
    AppState,
};
use crate::ui::{
    events::EventHandler,
    renderer::{Renderer, TerminalCleanup, Uninitialized},
};
use anyhow::{Context, Result};
use std::time::Duration;
use tokio::sync::mpsc;

/// Main application
pub struct App {
    /// The evaluator command to run
    evaluator_command: Option<String>,
    /// Application state
    state: AppState,
}

impl App {
    /// Create a new App instance
    pub fn new(evaluator_command: Option<String>) -> Self {
        Self {
            evaluator_command,
            state: AppState::new(),
        }
    }

    /// Run the application
    pub async fn run(&mut self) -> Result<()> {
        if let Some(cmd) = &self.evaluator_command {
            // Set up TUI
            let (action_tx, mut action_rx) = mpsc::channel(100);

            // Initialize terminal
            let renderer = Renderer::<Uninitialized>::new();
            let (renderer, mut terminal) = renderer
                .initialize()
                .context("Failed to initialize terminal")?;

            // Create cleanup guard
            let _cleanup = TerminalCleanup;

            // Start event handler in background
            let mut event_handler = EventHandler::new(action_tx);
            tokio::spawn(async move {
                if let Err(e) = event_handler.run().await {
                    tracing::error!("Event handler error: {}", e);
                }
            });

            // Set evaluator name from command
            if let Ok(name) = EvaluatorName::try_new(cmd.clone()) {
                self.state.set_evaluator_name(name)?;
            }

            // Update status to waiting for handshake
            self.state
                .update_status(EvaluationStatus::WaitingForHandshake)?;

            // Spawn evaluator process
            let (eval_tx, mut eval_rx) = mpsc::channel(100);
            let eval_cmd =
                EvaluatorCommand::try_new(cmd.clone()).context("Invalid evaluator command")?;

            let mut evaluator = EvaluatorProcess::spawn(&eval_cmd, eval_tx)
                .await
                .context("Failed to spawn evaluator")?;

            let mut handshake_received = false;
            let handshake_timeout = Duration::from_secs(5);
            let handshake_start = std::time::Instant::now();

            // Main event loop
            loop {
                // Render UI
                renderer.render(&mut terminal, &self.state)?;

                // Use select! to handle multiple channels
                tokio::select! {
                    // Handle UI actions
                    action = action_rx.recv() => {
                        match action {
                            Some(UiAction::Quit) => {
                                tracing::info!("User requested quit");
                                break;
                            }
                            Some(UiAction::TogglePause) => {
                                self.state.toggle_pause();
                            }
                            Some(UiAction::Resize(size)) => {
                                tracing::debug!("Terminal resized to {}x{}", size.width(), size.height());
                                // Terminal will be redrawn on next iteration
                            }
                            Some(UiAction::Refresh) => {
                                // Just redraw on next iteration
                            }
                            None => {
                                // Channel closed, exit
                                break;
                            }
                        }
                    }

                    // Handle evaluator messages
                    msg = eval_rx.recv() => {
                        match msg {
                            Some(EvaluatorMessage::Output(line)) => {
                                if !handshake_received {
                                    // Try to parse as handshake
                                    match parse_handshake(&line) {
                                        Ok(validated_handshake) => {
                                            tracing::info!("Received handshake from evaluator: {}", validated_handshake.evaluator.name);

                                            // Store handshake in state
                                            self.state.set_handshake(validated_handshake)?;
                                            handshake_received = true;

                                            // Move to collecting metrics status
                                            let total = self.state.handshake()
                                                .and_then(|h| h.execution_plan.as_ref())
                                                .map(|plan| plan.total_samples.into_inner() as usize);

                                            self.state.update_status(EvaluationStatus::CollectingMetrics {
                                                received: 0,
                                                total,
                                            })?;
                                        }
                                        Err(e) => {
                                            // Not a handshake - check if we're past timeout
                                            if handshake_start.elapsed() > handshake_timeout {
                                                self.state.update_status(EvaluationStatus::Failed(
                                                    "Handshake timeout: no valid handshake received within 5 seconds".to_string()
                                                ))?;
                                            } else {
                                                tracing::debug!("Received non-handshake line while waiting: {}", e);
                                                // Continue waiting for handshake
                                            }
                                        }
                                    }
                                } else {
                                    // Try to parse as OTLP metrics
                                    match parse_metrics_line(&line) {
                                        Ok(metrics) => {
                                            self.state.add_metrics(metrics)?;
                                        }
                                        Err(e) => {
                                            tracing::warn!("Failed to parse metrics: {}", e);
                                        }
                                    }
                                }
                            }
                            Some(EvaluatorMessage::Exited(status)) => {
                                if !handshake_received {
                                    self.state.update_status(EvaluationStatus::Failed(
                                        "Evaluator exited before sending handshake".to_string()
                                    ))?;
                                } else if status.success() {
                                    self.state.update_status(EvaluationStatus::Completed)?;
                                } else {
                                    self.state.update_status(EvaluationStatus::Failed(
                                        format!("Evaluator exited with code {:?}", status.code())
                                    ))?;
                                }
                            }
                            None => {
                                // Evaluator channel closed
                                if !self.state.is_terminal() {
                                    let error_msg = if !handshake_received {
                                        "Evaluator terminated before sending handshake"
                                    } else {
                                        "Evaluator terminated unexpectedly"
                                    };
                                    self.state.update_status(EvaluationStatus::Failed(
                                        error_msg.to_string()
                                    ))?;
                                }
                            }
                        }
                    }

                    // Check handshake timeout
                    _ = tokio::time::sleep(Duration::from_millis(100)) => {
                        if !handshake_received && handshake_start.elapsed() > handshake_timeout {
                            self.state.update_status(EvaluationStatus::Failed(
                                "Handshake timeout: no valid handshake received within 5 seconds".to_string()
                            ))?;
                        }
                    }
                }

                // Exit if in terminal state
                if self.state.is_terminal() {
                    // Wait a moment for user to see final state
                    tokio::time::sleep(std::time::Duration::from_secs(2)).await;
                    break;
                }
            }

            // Kill evaluator if still running
            let _ = evaluator.kill().await;
        } else {
            // No evaluator specified, just return
            return Ok(());
        }

        Ok(())
    }
}

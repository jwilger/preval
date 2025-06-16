use crate::state::types::EvaluatorCommand;
use anyhow::{Context, Result};
use std::process::Stdio;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::{Child, Command};
use tokio::sync::mpsc;

/// Message from evaluator process
#[derive(Debug)]
pub enum EvaluatorMessage {
    /// Output line from stdout
    Output(String),
    /// Process exited
    Exited(ExitStatus),
}

/// Exit status of evaluator
#[derive(Debug)]
pub struct ExitStatus {
    success: bool,
    code: Option<i32>,
}

impl ExitStatus {
    /// Whether the process exited successfully
    pub fn success(&self) -> bool {
        self.success
    }

    /// Exit code if available
    pub fn code(&self) -> Option<i32> {
        self.code
    }
}

/// Evaluator process handle with RAII cleanup
pub struct EvaluatorProcess {
    child: Child,
}

impl EvaluatorProcess {
    /// Spawn a new evaluator process
    pub async fn spawn(
        command: &EvaluatorCommand,
        message_tx: mpsc::Sender<EvaluatorMessage>,
    ) -> Result<Self> {
        // Parse command into program and args
        let parts: Vec<&str> = command.as_ref().split_whitespace().collect();
        if parts.is_empty() {
            anyhow::bail!("Empty evaluator command");
        }

        let program = parts[0];
        let args = &parts[1..];

        // Spawn the process
        let mut child = Command::new(program)
            .args(args)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped()) // Capture stderr to filter out cargo messages
            .stdin(Stdio::null())
            .kill_on_drop(true) // Ensure cleanup
            .spawn()
            .with_context(|| format!("Failed to spawn evaluator: {}", command))?;

        // Get stdout and stderr handles
        let stdout = child.stdout.take().context("Failed to capture stdout")?;
        let stderr = child.stderr.take().context("Failed to capture stderr")?;

        // Spawn task to read stdout
        let tx = message_tx.clone();
        tokio::spawn(async move {
            let reader = BufReader::new(stdout);
            let mut lines = reader.lines();

            while let Ok(Some(line)) = lines.next_line().await {
                if tx.send(EvaluatorMessage::Output(line)).await.is_err() {
                    // Receiver dropped, stop reading
                    break;
                }
            }
        });

        // Spawn task to read stderr and filter cargo messages
        let tx_stderr = message_tx.clone();
        tokio::spawn(async move {
            let reader = BufReader::new(stderr);
            let mut lines = reader.lines();

            while let Ok(Some(line)) = lines.next_line().await {
                // Filter out cargo build messages that pollute the terminal
                if line.trim().starts_with("Compiling")
                    || line.trim().starts_with("Finished")
                    || line.trim().starts_with("Running")
                    || line.trim().contains("target/debug/deps/")
                    || line.trim().is_empty()
                {
                    continue; // Skip cargo build output
                }

                // Send actual stderr as output (for real errors)
                if tx_stderr
                    .send(EvaluatorMessage::Output(format!("stderr: {}", line)))
                    .await
                    .is_err()
                {
                    break;
                }
            }
        });

        // Spawn task to monitor process exit
        let child_id = child.id();
        let tx_exit = message_tx;
        tokio::spawn(async move {
            // Monitor using the same child process we spawned
            // We need to get a handle to wait on the process
            if let Some(id) = child_id {
                // Wait a bit for the process to potentially exit
                loop {
                    tokio::time::sleep(std::time::Duration::from_millis(500)).await;

                    // Check if process still exists by trying to send signal 0
                    match std::process::Command::new("kill")
                        .args(["-0", &id.to_string()])
                        .output()
                    {
                        Ok(output) if !output.status.success() => {
                            // Process no longer exists
                            let exit_status = ExitStatus {
                                success: false, // We don't know the actual exit code
                                code: None,
                            };
                            let _ = tx_exit.send(EvaluatorMessage::Exited(exit_status)).await;
                            break;
                        }
                        _ => {
                            // Process still running or we couldn't check
                        }
                    }
                }
            }
        });

        Ok(Self { child })
    }

    /// Kill the evaluator process
    pub async fn kill(&mut self) -> Result<()> {
        self.child
            .kill()
            .await
            .context("Failed to kill evaluator")?;
        Ok(())
    }
}

impl Drop for EvaluatorProcess {
    fn drop(&mut self) {
        // Try to kill the process if it's still running
        // This is best-effort since we're in Drop
        if let Ok(Some(_)) = self.child.try_wait() {
            // Process already exited
            return;
        }

        // Try to kill it
        let _ = self.child.start_kill();
    }
}

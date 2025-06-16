use anyhow::Result;

/// Main application state and logic
pub struct App {
    /// The evaluator command to run
    pub evaluator_command: Option<String>,
}

impl App {
    /// Create a new App instance
    pub fn new(evaluator_command: Option<String>) -> Self {
        Self { evaluator_command }
    }

    /// Run the application
    pub async fn run(&mut self) -> Result<()> {
        if let Some(cmd) = &self.evaluator_command {
            tracing::info!("Will run evaluator: {}", cmd);
            todo!("Implement evaluator execution in future stories");
        }
        Ok(())
    }
}

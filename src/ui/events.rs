use crate::state::types::{TerminalSize, UiAction};
use anyhow::{Context, Result};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use std::time::Duration;
use tokio::sync::mpsc;

/// Event handler for terminal events
pub(crate) struct EventHandler {
    /// Channel to send actions to the main app
    action_tx: mpsc::Sender<UiAction>,
}

impl EventHandler {
    /// Create a new event handler
    pub(crate) fn new(action_tx: mpsc::Sender<UiAction>) -> Self {
        Self { action_tx }
    }

    /// Start listening for events
    pub(crate) async fn run(&mut self) -> Result<()> {
        loop {
            // Check for events with a small timeout to allow for cancellation
            if event::poll(Duration::from_millis(100))
                .context("Failed to poll for terminal events")?
            {
                let event = event::read().context("Failed to read terminal event")?;

                if let Some(action) = self.handle_event(event)? {
                    // Send action to main app
                    if self.action_tx.send(action).await.is_err() {
                        // Channel closed, exit gracefully
                        break;
                    }
                }
            }

            // Allow tokio to process other tasks
            tokio::task::yield_now().await;
        }

        Ok(())
    }

    /// Handle a terminal event and convert to an action
    fn handle_event(&self, event: Event) -> Result<Option<UiAction>> {
        match event {
            Event::Key(key_event) => Ok(self.handle_key_event(key_event)),
            Event::Resize(width, height) => {
                // Try to create valid terminal size
                match TerminalSize::try_new(width, height) {
                    Ok(size) => Ok(Some(UiAction::Resize(size))),
                    Err(_) => {
                        // Invalid size, ignore the event
                        Ok(None)
                    }
                }
            }
            _ => Ok(None), // Ignore other events
        }
    }

    /// Handle keyboard events
    fn handle_key_event(&self, key: KeyEvent) -> Option<UiAction> {
        match (key.code, key.modifiers) {
            // Quit on 'q' or Ctrl+C
            (KeyCode::Char('q'), KeyModifiers::NONE) => Some(UiAction::Quit),
            (KeyCode::Char('c'), KeyModifiers::CONTROL) => Some(UiAction::Quit),

            // Pause/resume on space
            (KeyCode::Char(' '), KeyModifiers::NONE) => Some(UiAction::TogglePause),

            // Force refresh on Ctrl+L
            (KeyCode::Char('l'), KeyModifiers::CONTROL) => Some(UiAction::Refresh),

            _ => None, // Ignore other keys
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_quit_on_q_key() {
        let (tx, _rx) = mpsc::channel(1);
        let handler = EventHandler::new(tx);

        let key_event = KeyEvent {
            code: KeyCode::Char('q'),
            modifiers: KeyModifiers::NONE,
            kind: event::KeyEventKind::Press,
            state: event::KeyEventState::NONE,
        };

        let action = handler.handle_key_event(key_event);
        assert_eq!(action, Some(UiAction::Quit));
    }

    #[tokio::test]
    async fn test_quit_on_ctrl_c() {
        let (tx, _rx) = mpsc::channel(1);
        let handler = EventHandler::new(tx);

        let key_event = KeyEvent {
            code: KeyCode::Char('c'),
            modifiers: KeyModifiers::CONTROL,
            kind: event::KeyEventKind::Press,
            state: event::KeyEventState::NONE,
        };

        let action = handler.handle_key_event(key_event);
        assert_eq!(action, Some(UiAction::Quit));
    }

    #[tokio::test]
    async fn test_toggle_pause_on_space() {
        let (tx, _rx) = mpsc::channel(1);
        let handler = EventHandler::new(tx);

        let key_event = KeyEvent {
            code: KeyCode::Char(' '),
            modifiers: KeyModifiers::NONE,
            kind: event::KeyEventKind::Press,
            state: event::KeyEventState::NONE,
        };

        let action = handler.handle_key_event(key_event);
        assert_eq!(action, Some(UiAction::TogglePause));
    }

    // Note: Tests for invalid terminal sizes are unnecessary because
    // the type system prevents creating TerminalSize with invalid dimensions
}

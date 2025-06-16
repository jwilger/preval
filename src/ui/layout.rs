use nutype::nutype;
use ratatui::prelude::*;

/// Non-zero layout height
#[nutype(
    validate(greater = 0),
    derive(Debug, Clone, Copy, PartialEq, Eq, AsRef, Into)
)]
struct LayoutHeight(u16);

/// Layout areas for the UI
#[derive(Debug, Clone, Copy)]
pub(crate) struct UiLayout {
    pub(crate) header: Rect,
    pub(crate) content: Rect,
    pub(crate) footer: Rect,
}

impl UiLayout {
    /// Calculate layout from terminal area
    pub(crate) fn new(area: Rect) -> Result<Self, LayoutError> {
        // Ensure we have minimum space
        const MIN_HEIGHT: u16 = 5; // header (3) + content (1) + footer (1)
        
        if area.height < MIN_HEIGHT {
            return Err(LayoutError::TooSmall {
                required: MIN_HEIGHT,
                actual: area.height,
            });
        }

        // Header is always 3 lines
        let header_height = LayoutHeight::try_new(3).unwrap(); // Safe: 3 > 0
        
        // Footer is always 1 line
        let footer_height = LayoutHeight::try_new(1).unwrap(); // Safe: 1 > 0
        
        // Content gets remaining space
        let content_height = area.height - header_height.into_inner() - footer_height.into_inner();
        
        // Create layout constraints
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(header_height.into_inner()),
                Constraint::Min(content_height),
                Constraint::Length(footer_height.into_inner()),
            ])
            .split(area);

        Ok(Self {
            header: chunks[0],
            content: chunks[1],
            footer: chunks[2],
        })
    }
}

#[derive(Debug, thiserror::Error)]
pub(crate) enum LayoutError {
    #[error("terminal too small: need at least {required} rows, got {actual}")]
    TooSmall { required: u16, actual: u16 },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_layout_minimum_size() {
        let area = Rect::new(0, 0, 80, 5);
        let layout = UiLayout::new(area).unwrap();
        
        assert_eq!(layout.header.height, 3);
        assert_eq!(layout.content.height, 1);
        assert_eq!(layout.footer.height, 1);
    }

    #[test]
    fn test_layout_too_small() {
        let area = Rect::new(0, 0, 80, 4);
        let result = UiLayout::new(area);
        
        assert!(result.is_err());
    }

    // Note: Tests for negative dimensions are unnecessary because
    // Rect from ratatui already ensures non-negative dimensions
}
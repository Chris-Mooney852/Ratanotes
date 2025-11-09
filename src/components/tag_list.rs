// Ratanotes/src/components/tag_list.rs

use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Paragraph};

pub struct TagListWidget;

impl Widget for TagListWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::default().title("Tags").borders(Borders::ALL);
        Paragraph::new("Tag list placeholder")
            .block(block)
            .render(area, buf);
    }
}

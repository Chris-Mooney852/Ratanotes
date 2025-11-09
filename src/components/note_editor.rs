// Ratanotes/src/components/note_editor.rs

use crate::app::state::{Mode, Note};
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Paragraph};

pub struct NoteEditorWidget<'a> {
    pub note: &'a Note,
    pub mode: &'a Mode,
}

impl<'a> Widget for NoteEditorWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let border_style = if let Mode::Insert = self.mode {
            Style::default().fg(Color::Blue)
        } else {
            Style::default()
        };
        let block = Block::default()
            .title(self.note.title.as_str())
            .borders(Borders::ALL)
            .border_style(border_style);
        Paragraph::new(self.note.content.as_str())
            .block(block)
            .render(area, buf);
    }
}

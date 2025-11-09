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

        let tags_text = if self.note.tags.is_empty() {
            String::new()
        } else {
            // Joins tags with a separator and adds a little flair
            let tags_str = self.note.tags.join(" | ");
            format!(" [ {} ]", tags_str)
        };

        let title = Line::from(vec![
            Span::raw(self.note.title.as_str()),
            Span::styled(
                tags_text,
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::ITALIC),
            ),
        ]);

        let block = Block::default()
            .title(title)
            .borders(Borders::ALL)
            .border_style(border_style);
        Paragraph::new(self.note.content.as_str())
            .block(block)
            .render(area, buf);
    }
}

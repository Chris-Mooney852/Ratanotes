// Ratanotes/src/components/note_list.rs

use crate::app::state::Note;
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, List, ListItem, ListState},
};

pub struct NoteListWidget<'a> {
    pub notes: &'a [Note],
    pub has_focus: bool,
}

impl<'a> StatefulWidget for NoteListWidget<'a> {
    type State = ListState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let items: Vec<ListItem> = self
            .notes
            .iter()
            .map(|note| ListItem::new(note.title.clone()))
            .collect();

        let border_style = if self.has_focus {
            Style::default().fg(Color::Green)
        } else {
            Style::default()
        };

        let list = List::new(items)
            .block(
                Block::default()
                    .title("Notes")
                    .borders(Borders::ALL)
                    .border_style(border_style),
            )
            .highlight_style(
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .bg(Color::Blue),
            );

        StatefulWidget::render(list, area, buf, state);
    }
}

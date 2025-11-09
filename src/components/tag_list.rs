// Ratanotes/src/components/tag_list.rs

use ratatui::{
    prelude::*,
    widgets::{Block, Borders, List, ListItem, ListState},
};

pub struct TagListWidget<'a> {
    pub tags: &'a [String],
    pub has_focus: bool,
}

impl<'a> StatefulWidget for TagListWidget<'a> {
    type State = ListState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let items: Vec<ListItem> = self
            .tags
            .iter()
            .map(|tag| ListItem::new(tag.clone()))
            .collect();

        let border_style = if self.has_focus {
            Style::default().fg(Color::Green)
        } else {
            Style::default()
        };

        let list = List::new(items)
            .block(
                Block::default()
                    .title("Tags")
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

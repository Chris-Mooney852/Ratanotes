// Ratanotes/src/components/command_bar.rs

use ratatui::{prelude::*, widgets::Paragraph};

pub struct CommandBarWidget<'a> {
    pub text: &'a str,
}

impl<'a> Widget for CommandBarWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let paragraph = Paragraph::new(self.text).style(Style::default());
        paragraph.render(area, buf);
    }
}

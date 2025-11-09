// Ratanotes/src/components/status_bar.rs

use ratatui::prelude::*;
use ratatui::widgets::Paragraph;

pub struct StatusBarWidget<'a> {
    pub message: &'a str,
}

impl<'a> Widget for StatusBarWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let style = Style::default();
        let paragraph = Paragraph::new(self.message).style(style);
        paragraph.render(area, buf);
    }
}

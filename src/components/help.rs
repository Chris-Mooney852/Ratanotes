// Ratanotes/src/components/help.rs

use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Cell, Row, Table},
};

pub struct HelpWidget;

impl Widget for HelpWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let key_style = Style::default().fg(Color::LightCyan);
        let description_style = Style::default().fg(Color::White);
        let header_style = Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD);

        let header_cells = ["Key(s)", "Action", "Mode(s) / View(s)"]
            .iter()
            .map(|h| Cell::from(*h).style(header_style));
        let header = Row::new(header_cells).height(1).bottom_margin(1);

        let rows = vec![
            // Global
            Row::new(vec![
                Cell::from("q").style(key_style),
                Cell::from("Quit the application").style(description_style),
                Cell::from("Normal (Global)").style(description_style),
            ]),
            Row::new(vec![
                Cell::from(":").style(key_style),
                Cell::from("Enter Command Mode").style(description_style),
                Cell::from("Normal (Global)").style(description_style),
            ]),
            Row::new(vec![
                Cell::from("/").style(key_style),
                Cell::from("Enter Search Mode").style(description_style),
                Cell::from("Normal (Global)").style(description_style),
            ]),
            Row::new(vec![
                Cell::from("?").style(key_style),
                Cell::from("Show this help view").style(description_style),
                Cell::from("Normal (Global)").style(description_style),
            ]),
            Row::new(vec![
                Cell::from("Esc").style(key_style),
                Cell::from("Exit current mode or view").style(description_style),
                Cell::from("All").style(description_style),
            ]),
            Row::new(vec![
                Cell::from("n, c, T").style(key_style),
                Cell::from("Switch to Notes, Calendar, Tasks views").style(description_style),
                Cell::from("Normal (Global)").style(description_style),
            ]),
            // Note List
            Row::new(vec![
                Cell::from("j / ↓").style(key_style),
                Cell::from("Move selection down").style(description_style),
                Cell::from("Note List").style(description_style),
            ]),
            Row::new(vec![
                Cell::from("k / ↑").style(key_style),
                Cell::from("Move selection up").style(description_style),
                Cell::from("Note List").style(description_style),
            ]),
            Row::new(vec![
                Cell::from("Enter").style(key_style),
                Cell::from("Open selected note").style(description_style),
                Cell::from("Note List").style(description_style),
            ]),
            Row::new(vec![
                Cell::from("a").style(key_style),
                Cell::from("Create a new note").style(description_style),
                Cell::from("Note List").style(description_style),
            ]),
            Row::new(vec![
                Cell::from("r").style(key_style),
                Cell::from("Rename selected note").style(description_style),
                Cell::from("Note List").style(description_style),
            ]),
            Row::new(vec![
                Cell::from("d").style(key_style),
                Cell::from("Delete selected note").style(description_style),
                Cell::from("Note List").style(description_style),
            ]),
            // Note Editor
            Row::new(vec![
                Cell::from("i").style(key_style),
                Cell::from("Enter Insert Mode").style(description_style),
                Cell::from("Note Editor (Normal)").style(description_style),
            ]),
            Row::new(vec![
                Cell::from("r").style(key_style),
                Cell::from("Rename the current note").style(description_style),
                Cell::from("Note Editor (Normal)").style(description_style),
            ]),
            // Calendar
            Row::new(vec![
                Cell::from("← / →").style(key_style),
                Cell::from("Navigate between months").style(description_style),
                Cell::from("Calendar").style(description_style),
            ]),
            // Command Mode
            Row::new(vec![
                Cell::from("w, write").style(key_style),
                Cell::from("Save all changes").style(description_style),
                Cell::from("Command").style(description_style),
            ]),
            Row::new(vec![
                Cell::from("q, quit").style(key_style),
                Cell::from("Quit the application").style(description_style),
                Cell::from("Command").style(description_style),
            ]),
            Row::new(vec![
                Cell::from("wq").style(key_style),
                Cell::from("Save all changes and quit").style(description_style),
                Cell::from("Command").style(description_style),
            ]),
        ];

        let table = Table::new(
            rows,
            [
                Constraint::Percentage(20),
                Constraint::Percentage(50),
                Constraint::Percentage(30),
            ],
        )
        .header(header)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Help - Keybindings"),
        )
        .widths([
            Constraint::Length(15),
            Constraint::Length(35),
            Constraint::Length(25),
        ]);

        ratatui::prelude::Widget::render(table, area, buf);
    }
}

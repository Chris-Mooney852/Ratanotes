// Ratanotes/src/components/task_editor.rs

use crate::app::state::{Priority, Task, TaskEditFocus};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Clear, Paragraph},
};

pub struct TaskEditorWidget<'a> {
    pub task: &'a Task,
    pub edit_buffer: &'a str,
    pub focus: &'a TaskEditFocus,
}

impl<'a> Widget for TaskEditorWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let popup_area = centered_rect(60, 30, area);

        // Clear the area behind the popup before rendering
        Clear.render(popup_area, buf);

        let block = Block::default()
            .title(" Edit Task ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Yellow));

        let editor_layout = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints(
                [
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Length(3),
                ]
                .as_ref(),
            )
            .split(block.inner(popup_area));

        block.render(popup_area, buf);

        // -- Description Field --
        let description_border_style = if let TaskEditFocus::Description = self.focus {
            Style::default().fg(Color::Cyan)
        } else {
            Style::default()
        };

        let description_p = Paragraph::new(self.edit_buffer).block(
            Block::default()
                .title("Description")
                .borders(Borders::ALL)
                .border_style(description_border_style),
        );
        description_p.render(editor_layout[0], buf);

        // -- Priority Field --
        let priority_border_style = if let TaskEditFocus::Priority = self.focus {
            Style::default().fg(Color::Cyan)
        } else {
            Style::default()
        };

        // Display the priority with arrows to indicate it can be changed
        let priority_text = format!("< {:?} >", self.task.priority);
        let priority_p = Paragraph::new(priority_text)
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .title("Priority")
                    .borders(Borders::ALL)
                    .border_style(priority_border_style),
            );
        priority_p.render(editor_layout[1], buf);

        // -- Due Date Field --
        let due_date_border_style = if let TaskEditFocus::DueDate = self.focus {
            Style::default().fg(Color::Cyan)
        } else {
            Style::default()
        };

        let due_date_text = if let TaskEditFocus::DueDate = self.focus {
            self.edit_buffer.to_string()
        } else {
            self.task
                .due_date
                .map(|d| d.format("%Y-%m-%d").to_string())
                .unwrap_or_else(|| "".to_string())
        };

        let due_date_p = Paragraph::new(due_date_text).block(
            Block::default()
                .title("Due Date (YYYY-MM-DD)")
                .borders(Borders::ALL)
                .border_style(due_date_border_style),
        );
        due_date_p.render(editor_layout[2], buf);
    }
}

/// Helper function to create a centered rect for the popup.
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
}

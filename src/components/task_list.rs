// Ratanotes/src/components/task_list.rs

use crate::app::state::Task;
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, List, ListItem, ListState};

pub struct TaskListWidget<'a> {
    pub tasks: &'a [Task],
}

impl<'a> StatefulWidget for TaskListWidget<'a> {
    type State = ListState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let items: Vec<ListItem> = self
            .tasks
            .iter()
            .map(|task| {
                let completed_marker = if task.completed { "[x]" } else { "[ ]" };
                let priority = format!("[{:?}]", task.priority);
                let due_date = task
                    .due_date
                    .map(|d| d.format(" (%Y-%m-%d)").to_string())
                    .unwrap_or_default();

                let line = format!(
                    "{} {} {}{}",
                    completed_marker, priority, task.description, due_date
                );
                ListItem::new(line)
            })
            .collect();

        let list = List::new(items)
            .block(Block::default().title("Tasks").borders(Borders::ALL))
            .highlight_style(
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .bg(Color::Blue),
            );

        StatefulWidget::render(list, area, buf, state);
    }
}

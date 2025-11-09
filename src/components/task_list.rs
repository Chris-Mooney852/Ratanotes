// Ratanotes/src/components/task_list.rs

use crate::app::state::Task;
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, List, ListItem};

pub struct TaskListWidget<'a> {
    pub tasks: &'a [Task],
}

impl<'a> Widget for TaskListWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let items: Vec<ListItem> = self
            .tasks
            .iter()
            .map(|task| {
                let completed_marker = if task.completed { "[x]" } else { "[ ]" };
                let line = format!("{} {}", completed_marker, task.description);
                ListItem::new(line)
            })
            .collect();

        let list = List::new(items).block(Block::default().title("Tasks").borders(Borders::ALL));

        ratatui::prelude::Widget::render(list, area, buf);
    }
}

use crate::components::{
    calendar::CalendarWidget, note_editor::NoteEditorWidget, note_list::NoteListWidget,
    status_bar::StatusBarWidget, task_list::TaskListWidget,
};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, List, ListItem, Paragraph},
};

use super::state::AppState;

/// Renders the user interface.
pub fn ui(frame: &mut Frame, app: &mut AppState) {
    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(1)])
        .split(frame.size());

    let content_area = main_layout[0];
    let status_bar_area = main_layout[1];

    // Render the main content based on the current view
    match app.current_view {
        super::state::View::NoteList => {
            let note_list = NoteListWidget { notes: &app.notes };
            frame.render_stateful_widget(note_list, content_area, &mut app.note_list_state);
        }
        super::state::View::NoteEditor => {
            if let Some(selected_index) = app.note_list_state.selected() {
                if let Some(note) = app.notes.get(selected_index) {
                    let note_editor = NoteEditorWidget {
                        note,
                        mode: &app.mode,
                    };
                    frame.render_widget(note_editor, content_area);
                }
            } else {
                let placeholder = Paragraph::new("No note selected.")
                    .block(Block::default().title("Notes").borders(Borders::ALL));
                frame.render_widget(placeholder, content_area);
            }
        }
        super::state::View::Calendar => {
            let calendar = CalendarWidget {
                year: app.calendar_year,
                month: app.calendar_month,
                notes: &app.notes,
            };
            frame.render_widget(calendar, content_area);
        }
        super::state::View::Tasks => {
            let task_list = TaskListWidget { tasks: &app.tasks };
            frame.render_widget(task_list, content_area);
        }
        super::state::View::Search => {
            let search_results: Vec<ListItem> = app
                .search_results
                .iter()
                .filter_map(|&index| app.notes.get(index))
                .map(|note| ListItem::new(note.title.as_str()))
                .collect();

            let results_list = List::new(search_results).block(
                Block::default()
                    .title("Search Results")
                    .borders(Borders::ALL),
            );

            frame.render_widget(results_list, content_area);
        }
    };

    // Render the status bar
    let status_bar = StatusBarWidget {
        message: &app.status_message,
    };
    frame.render_widget(status_bar, status_bar_area);
}

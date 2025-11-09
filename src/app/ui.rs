use crate::app::app::{App, Focus};
use crate::components::{
    calendar::CalendarWidget, help::HelpWidget, note_editor::NoteEditorWidget,
    note_list::NoteListWidget, status_bar::StatusBarWidget, tag_list::TagListWidget,
    task_list::TaskListWidget,
};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, List, ListItem, Paragraph},
};

use super::state::AppState;

/// Renders the user interface.
pub fn ui(frame: &mut Frame, app: &mut App, cursor_position: Option<(u16, u16)>) {
    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(1)])
        .split(frame.size());

    let content_area = main_layout[0];
    let status_bar_area = main_layout[1];

    // Render the main content based on the current view
    match app.state.current_view {
        super::state::View::NoteList => {
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
                .split(content_area);

            // TODO: This clones the notes, which is inefficient. A better approach would be
            // to store filtered indices in the app state.
            let notes_to_display: Vec<crate::app::state::Note> =
                if let Some(tag) = &app.state.active_tag {
                    app.state
                        .notes
                        .iter()
                        .filter(|note| note.tags.contains(tag))
                        .cloned()
                        .collect()
                } else {
                    app.state.notes.clone()
                };

            let note_list = NoteListWidget {
                notes: &notes_to_display,
                has_focus: matches!(app.focus, Focus::NoteList),
            };
            frame.render_stateful_widget(note_list, chunks[0], &mut app.state.note_list_state);

            let tag_list = TagListWidget {
                tags: &app.state.tags,
                has_focus: matches!(app.focus, Focus::TagList),
                active_tag: &app.state.active_tag,
            };
            frame.render_stateful_widget(tag_list, chunks[1], &mut app.state.tag_list_state);
        }
        super::state::View::NoteEditor => {
            if let Some(selected_index) = app.state.note_list_state.selected() {
                if let Some(note) = app.state.notes.get(selected_index) {
                    let note_editor = NoteEditorWidget {
                        note,
                        mode: &app.state.mode,
                    };
                    frame.render_widget(note_editor, content_area);
                    if let Some((cursor_x, cursor_y)) = cursor_position {
                        // Position the cursor. The text area is inside the block's borders.
                        frame.set_cursor(
                            content_area.x + 1 + cursor_x,
                            content_area.y + 1 + cursor_y,
                        );
                    }
                }
            } else {
                let placeholder = Paragraph::new("No note selected.")
                    .block(Block::default().title("Notes").borders(Borders::ALL));
                frame.render_widget(placeholder, content_area);
            }
        }
        super::state::View::Calendar => {
            let calendar = CalendarWidget {
                year: app.state.calendar_year,
                month: app.state.calendar_month,
                notes: &app.state.notes,
            };
            frame.render_widget(calendar, content_area);
        }
        super::state::View::Tasks => {
            let task_list = TaskListWidget {
                tasks: &app.state.tasks,
            };
            frame.render_widget(task_list, content_area);
        }
        super::state::View::Search => {
            let search_results: Vec<ListItem> = app
                .state
                .search_results
                .iter()
                .filter_map(|&index| app.state.notes.get(index))
                .map(|note| ListItem::new(note.title.as_str()))
                .collect();

            let results_list = List::new(search_results).block(
                Block::default()
                    .title("Search Results")
                    .borders(Borders::ALL),
            );

            frame.render_widget(results_list, content_area);
        }
        super::state::View::Help => {
            let help_widget = HelpWidget;
            frame.render_widget(help_widget, content_area);
        }
    };

    // Render the status bar
    let status_bar = StatusBarWidget {
        message: &app.state.status_message,
    };
    frame.render_widget(status_bar, status_bar_area);
}

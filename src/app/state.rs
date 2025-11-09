use chrono::{DateTime, Datelike, Local, NaiveDate, Utc};
use ratatui::widgets::ListState;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Represents the priority of a task.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum Priority {
    Low,
    Medium,
    High,
}

/// Represents a single to-do item.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: u64,
    pub description: String,
    pub project: Option<String>,
    pub priority: Priority,
    pub due_date: Option<NaiveDate>,
    pub completed: bool,
    pub created_at: DateTime<Utc>,
    pub sub_tasks: Vec<Task>,
}

/// Represents a single Markdown note.
#[derive(Debug, Clone)]
pub struct Note {
    pub path: PathBuf,
    pub title: String,
    pub content: String,
    pub tags: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Represents the current active view of the application.
#[derive(Clone, Debug)]
pub enum View {
    NoteList,
    NoteEditor,
    Calendar,
    Tasks,
    Search,
    Help,
}

/// Represents the current operational mode of the application.
pub enum Mode {
    Normal,
    Insert,
    Command,
    TitleInput,
    ConfirmDeletion,
}

/// The main application state.
pub struct AppState {
    pub notes: Vec<Note>,
    pub tasks: Vec<Task>,
    pub current_view: View,
    pub previous_view: Option<Box<View>>,
    pub search_query: String,
    pub status_message: String,
    pub running: bool,
    pub dirty: bool,
    pub calendar_year: i32,
    pub calendar_month: u32,
    pub mode: Mode,
    pub command_input: String,
    pub search_results: Vec<usize>,
    pub note_list_state: ListState,
    pub tags: Vec<String>,
    pub tag_list_state: ListState,
    pub active_tag: Option<String>,
    pub cursor_position: (u16, u16),
}

impl AppState {
    /// Creates a new instance of `AppState`.
    pub fn new() -> Self {
        let sample_note = Note {
            path: PathBuf::from("sample-note.md"),
            title: "Sample Note".to_string(),
            content: "This is the content of the sample note.".to_string(),
            tags: vec!["sample".to_string(), "rust".to_string()],
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let sample_tasks = vec![
            Task {
                id: 1,
                description: "Implement the task list view".to_string(),
                project: Some("Ratanotes".to_string()),
                priority: Priority::High,
                due_date: None,
                completed: false,
                created_at: Utc::now(),
                sub_tasks: vec![],
            },
            Task {
                id: 2,
                description: "Add sample data".to_string(),
                project: Some("Ratanotes".to_string()),
                priority: Priority::Medium,
                due_date: None,
                completed: true,
                created_at: Utc::now(),
                sub_tasks: vec![],
            },
        ];

        let now = Local::now();

        let daily_note_filename = now.format("%Y-%m-%d.md").to_string();
        let daily_note = Note {
            path: PathBuf::from(daily_note_filename),
            title: "Daily Note for today".to_string(),
            content: "This is a sample daily note for today.".to_string(),
            tags: vec!["daily".to_string()],
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let notes = vec![sample_note, daily_note];
        let mut note_list_state = ListState::default();
        if !notes.is_empty() {
            note_list_state.select(Some(0));
        }

        let mut tags: Vec<String> = notes.iter().flat_map(|note| note.tags.clone()).collect();
        tags.sort_unstable();
        tags.dedup();

        let tag_list_state = ListState::default();

        Self {
            notes,
            tasks: sample_tasks,
            current_view: View::NoteList,
            previous_view: None,
            search_query: String::new(),
            status_message: "Welcome to Ratanotes! Press 'q' to quit.".to_string(),
            running: true,
            dirty: false,
            calendar_year: now.year(),
            calendar_month: now.month(),
            mode: Mode::Normal,
            command_input: String::new(),
            search_results: Vec::new(),
            note_list_state,
            tags,
            tag_list_state,
            active_tag: None,
            cursor_position: (0, 0),
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}

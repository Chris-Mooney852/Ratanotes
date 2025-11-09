use crate::app::state::{AppState, Mode, Note, View};
use crate::app::ui::ui;
use crate::utils::data_handler::DataHandler;
use chrono::Utc;
use crossterm::{
    event::{
        self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind, KeyModifiers,
    },
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Terminal,
    backend::{Backend, CrosstermBackend},
};
use std::{
    io::{self, Result},
    path::PathBuf,
};

pub enum Focus {
    NoteList,
    TagList,
}

/// Represents the messages that can be sent to the update function.
pub enum Message {
    Quit,
    SwitchToNoteList,
    SwitchToCalendar,
    SwitchToTasks,
    PreviousMonth,
    NextMonth,
    Save,
    Char(char),
    Backspace,
    EnterSearch,
    ExitSearch,
    PreviousNote,
    NextNote,
    OpenNote,
    NewNote,
    RenameNote,
    SetNoteTitle,
    DeleteNote,
    ConfirmDelete,
    ToggleHelp,
    ToggleFocus,
    PreviousTag,
    NextTag,
    SelectTag,
    NewLine,
    CursorLeft,
    CursorRight,
    CursorUp,
    CursorDown,
    EnterInsertMode,
    EnterNormalMode,
    EnterCommandMode,
    ExecuteCommand,
}

/// The main application struct.
pub struct App {
    /// The application's state.
    pub(crate) state: AppState,
    /// Handles data persistence.
    pub(crate) data_handler: DataHandler,
    pub(crate) focus: Focus,
}

impl App {
    /// Creates a new `App`.
    pub fn new() -> Self {
        let data_handler = DataHandler::new().expect("Failed to initialize data handler");
        let mut state = AppState::new();

        let notes_result = data_handler.load_notes();
        let tasks_result = data_handler.load_tasks();

        let mut errors = vec![];

        match notes_result {
            Ok(notes) => state.notes = notes,
            Err(e) => errors.push(format!("notes ({})", e)),
        }

        match tasks_result {
            Ok(tasks) => state.tasks = tasks,
            Err(e) => errors.push(format!("tasks ({})", e)),
        }

        if !errors.is_empty() {
            state.status_message =
                format!("Error loading {}. Using sample data.", errors.join(", "));
        }

        let mut app = Self {
            state,
            data_handler,
            focus: Focus::NoteList,
        };
        app.update_tags();
        app
    }

    /// Runs the application's main loop.
    pub fn run<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> Result<()> {
        while self.state.running {
            // Draw the UI
            let cursor_position = if let Mode::Insert = self.state.mode {
                self.get_cursor_position()
            } else {
                None
            };
            terminal.draw(|frame| ui(frame, self, cursor_position))?;

            // Show/hide cursor based on mode
            match self.state.mode {
                Mode::Insert => {
                    if let Some(pos) = self.get_cursor_position() {
                        // We show the cursor before drawing to avoid flicker
                        terminal.set_cursor(pos.0 + 1, pos.1 + 1)?;
                    }
                    terminal.show_cursor()?
                }
                _ => terminal.hide_cursor()?,
            }

            // Handle events and get a message
            if let Some(message) = self.handle_events()? {
                // Update the state
                self.update(message);
            }
        }
        Ok(())
    }

    /// Updates the search results based on the current query.
    fn update_search_results(&mut self) {
        let query = self.state.search_query.to_lowercase();
        if query.is_empty() {
            self.state.search_results.clear();
        } else {
            self.state.search_results = self
                .state
                .notes
                .iter()
                .enumerate()
                .filter(|(_, note)| {
                    note.title.to_lowercase().contains(&query)
                        || note.content.to_lowercase().contains(&query)
                        || note
                            .tags
                            .iter()
                            .any(|tag| tag.to_lowercase().contains(&query))
                })
                .map(|(i, _)| i)
                .collect();
        }
    }

    /// Handles terminal events and returns a message if an action is required.
    /// Calculates the cursor (x, y) position based on the character offset.
    fn get_cursor_position(&self) -> Option<(u16, u16)> {
        if let Some(index) = self.state.note_list_state.selected() {
            if let Some(note) = self.state.notes.get(index) {
                let content = &note.content;
                let offset = self.state.cursor_offset.min(content.chars().count());

                let mut x = 0;
                let mut y = 0;

                for (i, c) in content.chars().enumerate() {
                    if i == offset {
                        break;
                    }
                    if c == '\n' {
                        x = 0;
                        y += 1;
                    } else {
                        x += 1; // Does not handle wide characters
                    }
                }

                return Some((x as u16, y as u16));
            }
        }
        None
    }

    /// Updates the global tag list from all notes.
    fn update_tags(&mut self) {
        let mut tags: Vec<String> = self
            .state
            .notes
            .iter()
            .flat_map(|note| note.tags.clone())
            .collect();
        tags.sort_unstable();
        tags.dedup();
        self.state.tags = tags;
    }

    fn handle_events(&self) -> Result<Option<Message>> {
        if event::poll(std::time::Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                if key.kind != KeyEventKind::Press {
                    return Ok(None);
                }

                // Handle modes first
                match self.state.mode {
                    Mode::Insert => {
                        return match key.code {
                            KeyCode::Esc => Ok(Some(Message::EnterNormalMode)),
                            KeyCode::Enter => Ok(Some(Message::NewLine)),
                            KeyCode::Left => Ok(Some(Message::CursorLeft)),
                            KeyCode::Right => Ok(Some(Message::CursorRight)),
                            KeyCode::Up => Ok(Some(Message::CursorUp)),
                            KeyCode::Down => Ok(Some(Message::CursorDown)),
                            KeyCode::Char(c) => Ok(Some(Message::Char(c))),
                            KeyCode::Backspace => Ok(Some(Message::Backspace)),
                            _ => Ok(None),
                        };
                    }
                    Mode::TitleInput => {
                        return match key.code {
                            KeyCode::Esc => Ok(Some(Message::EnterNormalMode)),
                            KeyCode::Enter => Ok(Some(Message::SetNoteTitle)),
                            KeyCode::Char(c) => Ok(Some(Message::Char(c))),
                            KeyCode::Backspace => Ok(Some(Message::Backspace)),
                            _ => Ok(None),
                        };
                    }
                    Mode::ConfirmDeletion => {
                        return match key.code {
                            KeyCode::Char('y') => Ok(Some(Message::ConfirmDelete)),
                            KeyCode::Char('n') | KeyCode::Esc => Ok(Some(Message::EnterNormalMode)),
                            _ => Ok(None),
                        };
                    }
                    Mode::Command => {
                        return match key.code {
                            KeyCode::Esc => Ok(Some(Message::EnterNormalMode)),
                            KeyCode::Enter => Ok(Some(Message::ExecuteCommand)),
                            KeyCode::Char(c) => Ok(Some(Message::Char(c))),
                            KeyCode::Backspace => Ok(Some(Message::Backspace)),
                            _ => Ok(None),
                        };
                    }
                    Mode::Normal => {
                        // Fall through to view-specific and global handlers
                    }
                }

                // Handle special views like Search that have their own input
                if let View::Search = self.state.current_view {
                    return match key.code {
                        KeyCode::Esc => Ok(Some(Message::ExitSearch)),
                        KeyCode::Char(c) => Ok(Some(Message::Char(c))),
                        KeyCode::Backspace => Ok(Some(Message::Backspace)),
                        _ => Ok(None),
                    };
                }

                if let View::Help = self.state.current_view {
                    return match key.code {
                        KeyCode::Char('?') | KeyCode::Esc => Ok(Some(Message::ToggleHelp)),
                        _ => Ok(None),
                    };
                }

                // View-specific keybindings in Normal mode
                match self.state.current_view {
                    View::NoteList => {
                        if let KeyCode::Tab = key.code {
                            return Ok(Some(Message::ToggleFocus));
                        }
                        match self.focus {
                            Focus::NoteList => match key.code {
                                KeyCode::Char('j') | KeyCode::Down => {
                                    return Ok(Some(Message::NextNote));
                                }
                                KeyCode::Char('k') | KeyCode::Up => {
                                    return Ok(Some(Message::PreviousNote));
                                }
                                KeyCode::Enter => return Ok(Some(Message::OpenNote)),
                                KeyCode::Char('a') => return Ok(Some(Message::NewNote)),
                                KeyCode::Char('r') => return Ok(Some(Message::RenameNote)),
                                KeyCode::Char('d') => return Ok(Some(Message::DeleteNote)),
                                _ => {}
                            },
                            Focus::TagList => match key.code {
                                KeyCode::Char('j') | KeyCode::Down => {
                                    return Ok(Some(Message::NextTag));
                                }
                                KeyCode::Char('k') | KeyCode::Up => {
                                    return Ok(Some(Message::PreviousTag));
                                }
                                KeyCode::Enter => return Ok(Some(Message::SelectTag)),
                                _ => {}
                            },
                        }
                    }
                    View::NoteEditor => match key.code {
                        KeyCode::Char('i') => return Ok(Some(Message::EnterInsertMode)),
                        KeyCode::Char('r') => return Ok(Some(Message::RenameNote)),
                        KeyCode::Esc => return Ok(Some(Message::SwitchToNoteList)),
                        _ => {}
                    },
                    View::Calendar => match key.code {
                        KeyCode::Left => return Ok(Some(Message::PreviousMonth)),
                        KeyCode::Right => return Ok(Some(Message::NextMonth)),
                        _ => {}
                    },
                    _ => {}
                }

                // Global keybindings in Normal mode
                match key.code {
                    KeyCode::Char(':') => return Ok(Some(Message::EnterCommandMode)),
                    KeyCode::Char('/') => return Ok(Some(Message::EnterSearch)),
                    KeyCode::Char('?') => return Ok(Some(Message::ToggleHelp)),
                    KeyCode::Char('q') => return Ok(Some(Message::Quit)),
                    KeyCode::Char('n') => return Ok(Some(Message::SwitchToNoteList)),
                    KeyCode::Char('c') => return Ok(Some(Message::SwitchToCalendar)),
                    KeyCode::Char('T') => return Ok(Some(Message::SwitchToTasks)),
                    _ => {}
                }
            }
        }
        Ok(None)
    }

    /// Updates the application state based on a message.
    fn update(&mut self, message: Message) {
        match message {
            Message::Quit => {
                self.state.running = false;
            }
            Message::SwitchToNoteList => self.state.current_view = View::NoteList,
            Message::SwitchToCalendar => self.state.current_view = View::Calendar,
            Message::SwitchToTasks => self.state.current_view = View::Tasks,
            Message::PreviousMonth => {
                if self.state.calendar_month == 1 {
                    self.state.calendar_month = 12;
                    self.state.calendar_year -= 1;
                } else {
                    self.state.calendar_month -= 1;
                }
            }
            Message::NextMonth => {
                if self.state.calendar_month == 12 {
                    self.state.calendar_month = 1;
                    self.state.calendar_year += 1;
                } else {
                    self.state.calendar_month += 1;
                }
            }
            Message::Save => {
                if self.state.dirty {
                    let notes_result = self.data_handler.save_notes(&self.state.notes);
                    let tasks_result = self.data_handler.save_tasks(&self.state.tasks);

                    let mut errors = vec![];
                    if let Err(e) = notes_result {
                        errors.push(format!("notes ({})", e));
                    }
                    if let Err(e) = tasks_result {
                        errors.push(format!("tasks ({})", e));
                    }

                    if errors.is_empty() {
                        self.state.status_message = "Saved successfully!".to_string();
                        self.state.dirty = false;
                        self.update_tags();
                    } else {
                        self.state.status_message = format!("Error saving {}.", errors.join(", "));
                    }
                } else {
                    self.state.status_message = "No changes to save.".to_string();
                }
            }
            Message::EnterInsertMode => {
                self.state.mode = Mode::Insert;
                if let Some(index) = self.state.note_list_state.selected() {
                    if let Some(note) = self.state.notes.get(index) {
                        self.state.cursor_offset = note.content.chars().count();
                    }
                }
                self.state.status_message = "-- INSERT --".to_string();
            }
            Message::EnterNormalMode => {
                if let Mode::Insert = self.state.mode {
                    self.state.dirty = true;
                }
                self.state.mode = Mode::Normal;
                self.state.status_message = "".to_string();
                self.state.command_input.clear();
            }
            Message::EnterCommandMode => {
                self.state.mode = Mode::Command;
                self.state.command_input.push(':');
                self.state.status_message = self.state.command_input.clone();
            }
            Message::ExecuteCommand => {
                let command = self.state.command_input.drain(1..).collect::<String>();
                match command.as_str() {
                    "w" | "write" => self.update(Message::Save),
                    "q" | "quit" => self.update(Message::Quit),
                    "wq" => {
                        self.update(Message::Save);
                        if !self.state.dirty {
                            // only quit if save was successful
                            self.update(Message::Quit);
                        }
                    }
                    _ => self.state.status_message = format!("Not a command: {}", command),
                }
                if self.state.running {
                    // if not quitting, return to normal mode
                    self.state.mode = Mode::Normal;
                    if !self.state.status_message.starts_with("Error")
                        && !self.state.status_message.starts_with("Not a command")
                    {
                        self.state.status_message = "".to_string();
                    }
                }
            }
            Message::Char(c) => match self.state.mode {
                Mode::Insert => {
                    if let Some(index) = self.state.note_list_state.selected() {
                        if let Some(note) = self.state.notes.get_mut(index) {
                            let offset = self.state.cursor_offset.min(note.content.chars().count());
                            let mut content: Vec<char> = note.content.chars().collect();
                            content.insert(offset, c);
                            note.content = content.into_iter().collect();
                            self.state.cursor_offset += 1;
                        }
                    }
                }
                Mode::Command => {
                    self.state.command_input.push(c);
                    self.state.status_message = self.state.command_input.clone();
                }
                Mode::TitleInput => {
                    let prefix = if self.state.note_list_state.selected().is_none() {
                        "New note title: "
                    } else {
                        "Rename note to: "
                    };
                    self.state.command_input.push(c);
                    self.state.status_message = format!("{}{}", prefix, self.state.command_input);
                }
                Mode::Normal => {
                    if let View::Search = self.state.current_view {
                        self.state.search_query.push(c);
                        self.update_search_results();
                        self.state.status_message = format!("/{}", self.state.search_query);
                    }
                }
                Mode::ConfirmDeletion => {}
            },
            Message::Backspace => match self.state.mode {
                Mode::Insert => {
                    if let Some(index) = self.state.note_list_state.selected() {
                        if let Some(note) = self.state.notes.get_mut(index) {
                            if self.state.cursor_offset > 0 {
                                let offset =
                                    self.state.cursor_offset.min(note.content.chars().count());
                                let mut content: Vec<char> = note.content.chars().collect();
                                content.remove(offset - 1);
                                note.content = content.into_iter().collect();
                                self.state.cursor_offset -= 1;
                            }
                        }
                    }
                }
                Mode::Command => {
                    self.state.command_input.pop();
                    if self.state.command_input.is_empty() {
                        self.update(Message::EnterNormalMode);
                    } else {
                        self.state.status_message = self.state.command_input.clone();
                    }
                }
                Mode::TitleInput => {
                    let prefix = if self.state.note_list_state.selected().is_none() {
                        "New note title: "
                    } else {
                        "Rename note to: "
                    };
                    self.state.command_input.pop();
                    self.state.status_message = format!("{}{}", prefix, self.state.command_input);
                }
                Mode::Normal => {
                    if let View::Search = self.state.current_view {
                        self.state.search_query.pop();
                        self.update_search_results();
                        self.state.status_message = format!("/{}", self.state.search_query);
                    }
                }
                Mode::ConfirmDeletion => {}
            },
            Message::EnterSearch => {
                self.state.current_view = View::Search;
                self.state.search_query.clear();
                self.state.status_message = "/".to_string();
                self.update_search_results();
            }
            Message::ExitSearch => {
                self.state.current_view = View::NoteList;
                self.state.search_query.clear();
                self.state.status_message = "".to_string();
                self.state.search_results.clear();
            }
            Message::PreviousNote => {
                if !self.state.notes.is_empty() {
                    let i = self.state.note_list_state.selected().unwrap_or(0);
                    let new_i = if i == 0 {
                        self.state.notes.len() - 1
                    } else {
                        i - 1
                    };
                    self.state.note_list_state.select(Some(new_i));
                }
            }
            Message::NextNote => {
                if !self.state.notes.is_empty() {
                    let i = self.state.note_list_state.selected().unwrap_or(0);
                    let new_i = if i >= self.state.notes.len() - 1 {
                        0
                    } else {
                        i + 1
                    };
                    self.state.note_list_state.select(Some(new_i));
                }
            }
            Message::OpenNote => {
                if self.state.note_list_state.selected().is_some() {
                    self.state.cursor_offset = 0;
                    self.state.current_view = View::NoteEditor;
                    self.state.status_message = "".to_string();
                }
            }
            Message::NewNote => {
                self.state.note_list_state.select(None); // Deselect to indicate new note
                self.state.mode = Mode::TitleInput;
                self.state.command_input.clear();
                self.state.status_message = "New note title: ".to_string();
            }
            Message::RenameNote => {
                if let Some(index) = self.state.note_list_state.selected() {
                    if let Some(note) = self.state.notes.get(index) {
                        self.state.mode = Mode::TitleInput;
                        self.state.command_input = note.title.clone();
                        self.state.status_message =
                            format!("Rename note to: {}", self.state.command_input);
                    }
                }
            }
            Message::SetNoteTitle => {
                let new_title = self.state.command_input.clone();
                if new_title.is_empty() {
                    self.state.status_message = "Title cannot be empty".to_string();
                    self.state.mode = Mode::Normal;
                    return;
                }

                if let Some(index) = self.state.note_list_state.selected() {
                    // This is a rename of an existing note
                    if let Some(note) = self.state.notes.get_mut(index) {
                        note.title = new_title;
                        self.state.dirty = true;
                    }
                } else {
                    // This is a new note
                    let timestamp = Utc::now().timestamp();
                    // A more robust path generation
                    let safe_title: String = new_title
                        .chars()
                        .filter(|c| c.is_alphanumeric() || *c == ' ')
                        .collect::<String>()
                        .replace(' ', "_");
                    let path = self
                        .data_handler
                        .notes_dir
                        .join(format!("{}_{}.md", safe_title, timestamp));
                    let new_note = Note {
                        path,
                        title: new_title,
                        content: String::new(),
                        tags: vec![],
                        created_at: Utc::now(),
                        updated_at: Utc::now(),
                    };

                    self.state.notes.push(new_note);
                    let new_note_index = self.state.notes.len() - 1;
                    self.state.note_list_state.select(Some(new_note_index));
                    self.state.current_view = View::NoteEditor;
                    self.state.mode = Mode::Insert;
                    self.state.status_message = "-- INSERT --".to_string();
                    return; // Skip returning to normal mode
                }
                self.update(Message::EnterNormalMode);
            }
            Message::DeleteNote => {
                if let Some(index) = self.state.note_list_state.selected() {
                    if let Some(note) = self.state.notes.get(index) {
                        self.state.mode = Mode::ConfirmDeletion;
                        self.state.status_message = format!("Delete '{}'? (y/n)", note.title);
                    }
                }
            }
            Message::ConfirmDelete => {
                if let Some(index) = self.state.note_list_state.selected() {
                    let note_to_delete = &self.state.notes[index].clone();
                    if let Err(e) = self.data_handler.delete_note(note_to_delete) {
                        self.state.status_message = format!("Error deleting note: {}", e);
                    } else {
                        self.state.notes.remove(index);
                        self.state.dirty = true; // The list of notes has changed
                        self.state.status_message = format!("'{}' deleted.", note_to_delete.title);

                        if self.state.notes.is_empty() {
                            self.state.note_list_state.select(None);
                        } else if index >= self.state.notes.len() {
                            // if it was the last one, select the new last one
                            self.state
                                .note_list_state
                                .select(Some(self.state.notes.len() - 1));
                        }
                    }
                }
                self.update(Message::EnterNormalMode);
            }
            Message::ToggleHelp => {
                if let View::Help = self.state.current_view {
                    if let Some(previous_view) = self.state.previous_view.take() {
                        self.state.current_view = *previous_view;
                    } else {
                        // Fallback if there's no previous view
                        self.state.current_view = View::NoteList;
                    }
                } else {
                    self.state.previous_view = Some(Box::new(self.state.current_view.clone()));
                    self.state.current_view = View::Help;
                }
            }
            Message::ToggleFocus => {
                self.focus = match self.focus {
                    Focus::NoteList => Focus::TagList,
                    Focus::TagList => Focus::NoteList,
                };
            }
            Message::PreviousTag => {
                if !self.state.tags.is_empty() {
                    let i = self.state.tag_list_state.selected().unwrap_or(0);
                    let new_i = if i == 0 {
                        self.state.tags.len() - 1
                    } else {
                        i - 1
                    };
                    self.state.tag_list_state.select(Some(new_i));
                }
            }
            Message::NextTag => {
                if !self.state.tags.is_empty() {
                    let i = self.state.tag_list_state.selected().unwrap_or(0);
                    let new_i = if i >= self.state.tags.len() - 1 {
                        0
                    } else {
                        i + 1
                    };
                    self.state.tag_list_state.select(Some(new_i));
                }
            }
            Message::SelectTag => {
                if let Some(index) = self.state.tag_list_state.selected() {
                    let tag = &self.state.tags[index];
                    if self.state.active_tag.as_ref() == Some(tag) {
                        self.state.active_tag = None; // Deselect if already active
                    } else {
                        self.state.active_tag = Some(tag.clone());
                    }
                    // Reset note list selection
                    if !self.state.notes.is_empty() {
                        self.state.note_list_state.select(Some(0));
                    } else {
                        self.state.note_list_state.select(None);
                    }
                }
            }
            Message::NewLine => {
                if let Mode::Insert = self.state.mode {
                    if let Some(index) = self.state.note_list_state.selected() {
                        if let Some(note) = self.state.notes.get_mut(index) {
                            let offset = self.state.cursor_offset.min(note.content.chars().count());
                            let mut content: Vec<char> = note.content.chars().collect();
                            content.insert(offset, '\n');
                            note.content = content.into_iter().collect();
                            self.state.cursor_offset += 1;
                        }
                    }
                }
            }
            Message::CursorLeft => {
                self.state.cursor_offset = self.state.cursor_offset.saturating_sub(1);
            }
            Message::CursorRight => {
                if let Some(index) = self.state.note_list_state.selected() {
                    if let Some(note) = self.state.notes.get(index) {
                        if self.state.cursor_offset < note.content.chars().count() {
                            self.state.cursor_offset += 1;
                        }
                    }
                }
            }
            Message::CursorUp => {
                if let Some(index) = self.state.note_list_state.selected() {
                    if let Some(note) = self.state.notes.get(index) {
                        let offset = self.state.cursor_offset;
                        let content_chars: Vec<char> = note.content.chars().collect();
                        let line_starts: Vec<usize> = std::iter::once(0)
                            .chain(
                                content_chars
                                    .iter()
                                    .enumerate()
                                    .filter(|&(_, &c)| c == '\n')
                                    .map(|(i, _)| i + 1),
                            )
                            .collect();

                        let current_line_index = line_starts
                            .iter()
                            .rposition(|&start| start <= offset)
                            .unwrap_or(0);

                        if current_line_index > 0 {
                            let current_col = offset - line_starts[current_line_index];
                            let prev_line_index = current_line_index - 1;
                            let prev_line_start = line_starts[prev_line_index];
                            let prev_line_end = line_starts[current_line_index] - 1;
                            let prev_line_len = prev_line_end - prev_line_start;
                            self.state.cursor_offset =
                                prev_line_start + current_col.min(prev_line_len);
                        }
                    }
                }
            }
            Message::CursorDown => {
                if let Some(index) = self.state.note_list_state.selected() {
                    if let Some(note) = self.state.notes.get(index) {
                        let offset = self.state.cursor_offset;
                        let content_chars: Vec<char> = note.content.chars().collect();

                        let line_starts: Vec<usize> = std::iter::once(0)
                            .chain(
                                content_chars
                                    .iter()
                                    .enumerate()
                                    .filter(|&(_, &c)| c == '\n')
                                    .map(|(i, _)| i + 1),
                            )
                            .collect();

                        let current_line_index = line_starts
                            .iter()
                            .rposition(|&start| start <= offset)
                            .unwrap_or(0);

                        if current_line_index < line_starts.len() - 1 {
                            let current_col = offset - line_starts[current_line_index];
                            let next_line_index = current_line_index + 1;
                            let next_line_start = line_starts[next_line_index];
                            let next_line_end = if next_line_index + 1 < line_starts.len() {
                                line_starts[next_line_index + 1] - 1
                            } else {
                                content_chars.len()
                            };
                            let next_line_len = next_line_end - next_line_start;
                            self.state.cursor_offset =
                                next_line_start + current_col.min(next_line_len);
                        }
                    }
                }
            }
        }
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

/// Sets up the terminal for TUI rendering.
pub fn setup_terminal() -> Result<Terminal<CrosstermBackend<io::Stdout>>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    Terminal::new(backend)
}

/// Restores the terminal to its original state.
pub fn restore_terminal(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> Result<()> {
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    Ok(())
}

# Ratanotes Design Document

## 1. Introduction

Ratanotes is a terminal-based user interface (TUI) application for taking and managing notes in Markdown format. It allows users to create, search, and organize notes efficiently within the terminal.

Key features include:
- **Markdown Note-Taking**: Create and edit notes using Markdown syntax.
- **Advanced Search**: Find notes by title, content, or tags.
- **Calendar Integration**: A calendar view to access daily notes, with visual indicators for days with existing entries.
- **Task Management**: Add tasks with due dates and priorities, displayed in a dedicated, sortable, and filterable component.
- **Tagging**: Organize notes using tags within the YAML front matter of the markdown files.

## 2. Goals and Objectives

- **Primary Goal**: To provide a fast, efficient, and keyboard-centric note-taking and task management application for developers and power users within the terminal.
- **Target Audience**: Developers, writers, and anyone comfortable working in a terminal environment, particularly those familiar with Vim.
- **Core Objectives**:
    - **Usability**: Offer an intuitive and efficient user experience with Vim-like keybindings for navigation and interaction.
    - **Functionality**: Implement robust features for note-taking, task management, and searching.
    - **Performance**: Ensure the application is fast and responsive, even with a large number of notes.
    - **Extensibility**: Design the application to be modular and easy to extend with new features in the future.

## 3. High-Level Architecture

The application will follow the principles of The Elm Architecture, which is a variant of Model-View-Controller (MVC). This architecture promotes a clear and unidirectional data flow, making the application state predictable and easier to manage.

The core components of this architecture are:

- **Model**: Represents the state of the application. In Ratanotes, this will be a single struct that holds all application data, such as the list of notes, tasks, current view, user input, etc.
- **View**: A function that takes the `Model` and renders the user interface. This will be responsible for drawing all the TUI components using the `ratatui` library. The view is a pure function of the state.
- **Update**: A function that handles messages (events) and updates the `Model` accordingly. All user input, file I/O results, and other events will be processed as messages. The `update` function takes a message and the current model, and returns a new model. This is the only place where the application state can be modified.

The data flow will be as follows:
1. The application starts with an initial `Model`.
2. The `view` function renders the initial UI based on the `Model`.
3. The main application loop waits for user input or other events.
4. An event is translated into a `Message`.
5. The `Message` is sent to the `update` function along with the current `Model`.
6. The `update` function processes the `Message` and returns a new, updated `Model`.
7. The main loop calls the `view` function with the new `Model` to re-render the UI.
8. The process repeats.

## 4. Components

### 4.1. Main Application

The `Main Application` component is the entry point and core of Ratanotes. It is responsible for:
- **Initialization**: Setting up the terminal for TUI rendering, initializing the application state (`Model`), and loading initial data (e.g., notes and tasks).
- **Main Loop**: Running the main application loop that listens for events, processes them, updates the state, and redraws the UI.
- **Event Handling**: Capturing keyboard inputs and other terminal events and translating them into `Messages` to be processed by the `update` function.
- **Shutdown**: Restoring the terminal to its original state when the application exits.

### 4.2. UI Components

The UI will be composed of several reusable components, each responsible for rendering a specific part of the application.

- **Note Editor**: A text area for creating and editing Markdown notes. It should support basic text editing features and potentially syntax highlighting.
- **Calendar View**: A component that displays a calendar for the current month. Days with notes will be visually distinct. Users can navigate between months and select a day to open the corresponding daily note.
- **Task List**: A list that displays tasks with their due date and priority. It will provide functionality to sort and filter the tasks.
- **Search Input**: An input field for entering search queries. This could be part of a larger search view that displays results.
- **Status Bar**: A bar at the bottom of the screen to display application status, messages, and keyboard shortcuts.
- **Tag List**: A component that displays a list of all tags found in the notes. Selecting a tag will filter the visible notes.

### 4.3. State Management

The application's state will be managed in a central `AppState` struct, following the Elm architecture's `Model` principle. This struct will hold all the data necessary to render the UI and manage the application's logic.

A simplified representation of the `AppState` might look like this:

```rust
// A simplified representation of a note.
struct Note {
    path: PathBuf,
    title: String,
    content: String,
    tags: Vec<String>,
}

// A simplified representation of a task.
struct Task {
    description: String,
    due_date: Option<Date>,
    priority: Priority,
    completed: bool,
}

// Enum to represent the current active view.
enum View {
    Notes,
    Calendar,
    Tasks,
    Search,
}

// The main application state.
struct AppState {
    notes: Vec<Note>,
    tasks: Vec<Task>,
    current_view: View,
    search_query: String,
    status_message: String,
    running: bool, // To control the main loop.
    // ... other state fields like selected note/task index, calendar date, etc.
}
```

This centralized state makes it easy to reason about the application's data flow. The `update` function will be the only place where this state is mutated, ensuring predictable state transitions.

### 4.4. Data Persistence

Data will be stored on the local filesystem in a dedicated directory. A possible structure would be:

```
~/.config/ratanotes/
├── notes/
│   ├── daily-notes/
│   │   ├── 2023-10-27.md
│   │   └── ...
│   ├── another-note.md
│   └── ...
└── tasks.json
```

- **Notes**: Each note will be a separate Markdown file (`.md`). The title of the note can be derived from the filename or from a level 1 header in the file.
- **Daily Notes**: Notes associated with a specific date on the calendar will be stored in the `notes/daily-notes/` directory with a `YYYY-MM-DD.md` filename format.
- **Tasks**: All tasks will be stored in a single `tasks.json` file (or a similar format like YAML or TOML) for easy parsing and management.
- **Tags**: Note tags will be stored in the YAML front matter of each Markdown file. For example:

```yaml
---
tags: [rust, tui, project]
---

# My Note Title

This is the content of my note.
```

The application will be responsible for reading these files on startup and writing any changes back to the filesystem.

## 5. Data Model

This section provides a more detailed definition of the core data structures used in the application.

### 5.1. Note

The `Note` struct represents a single Markdown note.

```rust
use std::path::PathBuf;
use chrono::{DateTime, Utc};

struct Note {
    path: PathBuf,
    title: String,
    content: String,
    tags: Vec<String>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}
```

- `path`: The absolute path to the `.md` file on the filesystem.
- `title`: The title of the note, extracted from the filename or H1 header.
- `content`: The full Markdown content of the note.
- `tags`: A list of tags extracted from the YAML front matter.
- `created_at`: The timestamp when the note was created. This could be derived from file metadata.
- `updated_at`: The timestamp when the note was last modified. This could be derived from file metadata.

### 5.2. Task

The `Task` struct represents a single to-do item.

```rust
use chrono::{Date, DateTime, Utc};

enum Priority {
    Low,
    Medium,
    High,
}

struct Task {
    id: u64, // Unique identifier
    description: String,
    project: Option<String>,
    priority: Priority,
    due_date: Option<Date<Utc>>,
    completed: bool,
    created_at: DateTime<Utc>,
    sub_tasks: Vec<Task>,
}
```

- `id`: A unique identifier for the task to distinguish it from others.
- `description`: The text content of the task.
- `project`: An optional string to associate the task with a project.
- `priority`: The priority level of the task (`Low`, `Medium`, `High`).
- `due_date`: An optional due date for the task.
- `completed`: A boolean flag indicating whether the task is completed.
- `created_at`: The timestamp when the task was created.
- `sub_tasks`: A vector of nested `Task` structs, allowing for hierarchical task management.

## 6. UI/UX Flow

This section describes the key user interactions and workflows within the application. The keybindings are designed to be familiar to Vim users.

### 6.1. General Navigation
- **`<Tab>`**: Move focus between different UI components (e.g., from the note list to the note editor).
- **`<Shift+Tab>`**: Move focus in the reverse direction.
- **`q`**: Quit the application or go back from a specific view/mode.

### 6.2. Creating a New Note
1. From the main note view, the user presses `a`.
2. A new, empty buffer is opened in the note editor component.
3. The user writes the note content using Markdown.
4. The user presses `:w` (or a similar save command) to save the note.
5. The application prompts for a filename.
6. After entering a filename, the note is saved to the filesystem in the `notes/` directory.
7. The note list is updated to include the new note.

### 6.3. Searching for a Note
1. The user presses `/` to enter search mode.
2. The focus shifts to a search input field.
3. The user types their search query.
4. The note list is dynamically filtered to show only notes that match the query (by title, content, or tags).
5. The user can press `<Enter>` to confirm the search and move focus to the note list, or `<Esc>` to cancel.

### 6.4. Filtering Notes/Tasks
1. The user presses `f` to open filter options.
2. A context menu or popup appears with filtering criteria (e.g., by tag for notes, by project or priority for tasks).
3. The user selects a filter option.
4. The view is updated to show only the items that match the filter.

### 6.5. Navigating the Calendar
1. The user navigates to the Calendar view (e.g., by pressing `c`).
2. The current month is displayed. Days with existing notes are highlighted.
3. The user can use the arrow keys (`←`, `→`, `↑`, `↓`) to navigate between days.
4. Pressing `<Enter>` on a selected day opens the corresponding daily note in the editor. If no note exists, a new one is created for that day.

### 6.6. Adding a New Task
1. The user navigates to the Tasks view (e.g., by pressing `T`).
2. The user presses `t` to add a new task.
3. A form or input prompt appears to enter the task details (description, project, priority, due date).
4. After confirming, the new task is added to the task list and saved to `tasks.json`.

## 7. Future Considerations

This section lists potential features and enhancements that could be added to Ratanotes in the future.

- **Treesitter Integration**: Add support for `tree-sitter` for robust and efficient Markdown syntax highlighting and potentially for navigating the document structure.
- **Plugin System**: Develop a plugin architecture to allow users to extend the functionality of the application with custom features.
- **Customizable Themes**: Allow users to customize the colors and appearance of the TUI to their liking.
- **Git Integration**: Integrate with Git for versioning and syncing notes across different machines.
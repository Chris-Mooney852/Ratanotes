# Ratanotes

Ratanotes is a simple, fast, and Vim-inspired note-taking application that runs in your terminal. Built with Rust and the [`ratatui`](https://ratatui.rs/) library, it's designed for developers, writers, and anyone who loves the efficiency of a keyboard-centric workflow.

## Features

-   **Vim-like Keybindings**: Navigate, edit, and manage your notes without leaving the keyboard.
-   **Markdown Support**: Write your notes in Markdown, with support for YAML front matter for tagging.
-   **Note Management**: Easily create, rename, and delete notes.
-   **Command Mode**: A familiar command mode for actions like saving (`:w`) and quitting (`:q`).
-   **Note List**: A filterable and searchable list of all your notes for quick access.
-   **Full-text Search**: Instantly search through the title, content, and tags of all your notes.
-   **Calendar View**: A monthly calendar view to access your daily notes. Days with notes are highlighted.
-   **Task Management**: A dedicated view to manage your tasks (feature in progress).

## Installation

### Prerequisites

-   [Rust and Cargo](https://www.rust-lang.org/tools/install)

### Steps

1.  Clone the repository:
    ```sh
    git clone https://github.com/your-username/ratanotes.git
    cd ratanotes
    ```

2.  Build and run the application:
    ```sh
    cargo run
    ```
    The application will create a `~/.config/ratanotes` directory to store your notes and tasks.

## Usage

Ratanotes uses different "modes" for interaction, similar to Vim.

-   **Normal Mode**: The default mode for navigation and executing commands.
-   **Insert Mode**: For typing and editing text in your notes.
-   **Command Mode**: For entering commands like `:w` (write/save) and `:q` (quit).

### Keybindings

| Key(s)                  | Action                                            | Mode(s) / View(s)          |
| ----------------------- | ------------------------------------------------- | -------------------------- |
| `q`                     | Quit the application                              | Normal (Global)            |
| `:`                     | Enter Command Mode                                | Normal (Global)            |
| `/`                     | Enter Search Mode                                 | Normal (Global)            |
| `Esc`                   | Exit current mode or view                         | All                        |
| `n`, `c`, `T`           | Switch to Notes, Calendar, Tasks views            | Normal (Global)            |
| **Note List**           |                                                   |                            |
| `j` / `↓`               | Move selection down                               | Normal                     |
| `k` / `↑`               | Move selection up                                 | Normal                     |
| `Enter`                 | Open selected note                                | Normal                     |
| `a`                     | Create a new note                                 | Normal                     |
| `r`                     | Rename selected note                              | Normal                     |
| `d`                     | Delete selected note (with confirmation)          | Normal                     |
| **Note Editor**         |                                                   |                            |
| `i`                     | Enter Insert Mode                                 | Normal                     |
| `r`                     | Rename the current note                           | Normal                     |
| `Esc`                   | Exit Insert Mode, return to Normal Mode           | Insert                     |
| `Esc`                   | Exit editor, return to Note List                  | Normal                     |
| **Calendar**            |                                                   |                            |
| `←` / `→`               | Navigate between months                           | Normal                     |
| **Command Mode**        |                                                   |                            |
| `w`, `write`            | Save all changes                                  | Command                    |
| `q`, `quit`             | Quit the application                              | Command                    |
| `wq`                    | Save all changes and quit                         | Command                    |

## Configuration

Ratanotes stores all its data in `~/.config/ratanotes/`:

-   **Notes**: `~/.config/ratanotes/notes/` - Each note is a separate Markdown file.
-   **Daily Notes**: `~/.config/ratanotes/notes/daily-notes/` - Daily notes are named `YYYY-MM-DD.md`.
-   **Tasks**: `~/.config/ratanotes/tasks.json` - All tasks are stored in a single JSON file.

## Future Development

Ratanotes is under active development. Some features planned for the future include:

-   Full task management (add, edit, delete, prioritize).
-   `tree-sitter` integration for better syntax highlighting.
-   Customizable themes.
-   A plugin system.
-   Git integration for versioning notes.

## License

This project is licensed under the MIT License.
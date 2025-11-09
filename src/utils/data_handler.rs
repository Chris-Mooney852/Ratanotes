// Ratanotes/src/utils/data_handler.rs

use crate::app::state::{Note, Task};
use chrono::{DateTime, Utc};
use glob::glob;
use serde_yaml;
use std::{
    fs::{self, File},
    io::{Read, Write},
    path::{Path, PathBuf},
};

/// Handles data persistence for the application.
pub struct DataHandler {
    pub notes_dir: PathBuf,
    tasks_file: PathBuf,
}

impl DataHandler {
    /// Creates a new `DataHandler` and ensures the necessary directories and files exist.
    pub fn new() -> Result<Self, std::io::Error> {
        let home_dir = dirs::home_dir().ok_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Could not find home directory",
            )
        })?;
        let config_dir = home_dir.join(".config").join("ratanotes");
        let notes_dir = config_dir.join("notes");
        let daily_notes_dir = notes_dir.join("daily-notes");
        let tasks_file = config_dir.join("tasks.json");

        fs::create_dir_all(&daily_notes_dir)?;

        if !tasks_file.exists() {
            File::create(&tasks_file)?;
        }

        Ok(Self {
            notes_dir,
            tasks_file,
        })
    }

    /// Loads all notes from the filesystem.
    pub fn load_notes(&self) -> Result<Vec<Note>, std::io::Error> {
        let mut notes = Vec::new();
        let pattern = self.notes_dir.join("**/*.md");
        let pattern_str = pattern.to_str().unwrap_or_default();

        for entry in glob(pattern_str).expect("Failed to read glob pattern") {
            if let Ok(path) = entry {
                if let Ok(note) = self.parse_note(&path) {
                    notes.push(note);
                }
            }
        }
        Ok(notes)
    }

    /// Parses a single note file.
    fn parse_note(&self, path: &Path) -> Result<Note, std::io::Error> {
        let mut file = File::open(path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;

        let metadata = fs::metadata(path)?;
        let created_at: DateTime<Utc> = metadata.created()?.into();
        let updated_at: DateTime<Utc> = metadata.modified()?.into();

        let (tags, parsed_title, body) = self.parse_front_matter(&content);

        let title = if !parsed_title.is_empty() && parsed_title != "Untitled" {
            parsed_title
        } else {
            path.file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("Untitled")
                .to_string()
        };

        Ok(Note {
            path: path.to_path_buf(),
            title,
            content: body,
            tags,
            created_at,
            updated_at,
        })
    }

    /// Parses YAML front matter and extracts title from the content.
    fn parse_front_matter<'a>(&self, content: &'a str) -> (Vec<String>, String, String) {
        if content.starts_with("---") {
            if let Some(end_front_matter) = content[3..].find("---") {
                let front_matter_str = &content[3..3 + end_front_matter];
                let body = content[3 + end_front_matter + 3..].trim_start();
                if let Ok(front_matter) =
                    serde_yaml::from_str::<serde_yaml::Value>(front_matter_str)
                {
                    let tags = front_matter["tags"]
                        .as_sequence()
                        .unwrap_or(&vec![])
                        .iter()
                        .filter_map(|v| v.as_str().map(String::from))
                        .collect();
                    let (title, body_content) = self.extract_title_from_body(body);
                    return (tags, title, body_content.to_string());
                }
            }
        }
        let (title, body_content) = self.extract_title_from_body(content);
        (vec![], title, body_content.to_string())
    }

    /// Extracts the first H1 header as the title.
    fn extract_title_from_body<'a>(&self, body: &'a str) -> (String, &'a str) {
        if let Some(h1_line_end) = body.find('\n') {
            let first_line = &body[..h1_line_end];
            if first_line.starts_with("# ") {
                let title = first_line[2..].trim().to_string();
                let body_without_h1 = body[h1_line_end..].trim_start();
                return (title, body_without_h1);
            }
        } else if body.starts_with("# ") {
            return (body[2..].trim().to_string(), "");
        }
        ("Untitled".to_string(), body)
    }

    /// Loads all tasks from the filesystem.
    pub fn load_tasks(&self) -> Result<Vec<Task>, std::io::Error> {
        let mut file = File::open(&self.tasks_file)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;

        if content.is_empty() {
            return Ok(Vec::new());
        }

        let tasks = serde_json::from_str(&content)?;
        Ok(tasks)
    }

    /// Saves all tasks to the filesystem.
    pub fn save_tasks(&self, tasks: &[Task]) -> Result<(), std::io::Error> {
        let mut file = File::create(&self.tasks_file)?;
        let content = serde_json::to_string_pretty(tasks)?;
        file.write_all(content.as_bytes())?;
        Ok(())
    }

    /// Saves all notes to the filesystem.
    pub fn save_notes(&self, notes: &[Note]) -> Result<(), std::io::Error> {
        for note in notes {
            let mut file = File::create(&note.path)?;
            let mut content = String::new();

            if !note.tags.is_empty() {
                let mut front_matter = "---\ntags:\n".to_string();
                for tag in &note.tags {
                    front_matter.push_str(&format!("  - {}\n", tag));
                }
                front_matter.push_str("---\n\n");
                content.push_str(&front_matter);
            }

            let title_header = format!("# {}\n\n", note.title);
            content.push_str(&title_header);
            content.push_str(&note.content);

            file.write_all(content.as_bytes())?;
        }
        Ok(())
    }

    /// Deletes a note file from the filesystem.
    pub fn delete_note(&self, note: &Note) -> Result<(), std::io::Error> {
        fs::remove_file(&note.path)
    }
}

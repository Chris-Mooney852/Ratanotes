// Ratanotes/src/components/calendar.rs

use crate::app::state::Note;
use chrono::{Datelike, Local, NaiveDate};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph},
};
use std::collections::HashSet;

pub struct CalendarWidget<'a> {
    pub year: i32,
    pub month: u32,
    pub notes: &'a [Note],
}

impl<'a> Widget for CalendarWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::default()
            .title(format!("{} {}", month_name(self.month), self.year))
            .borders(Borders::ALL);
        let inner_area = block.inner(area);
        block.render(area, buf);

        // Layout for weekday headers and the days grid
        let layout = Layout::vertical([
            Constraint::Length(1), // For "Mo", "Tu", etc.
            Constraint::Min(0),    // For the days
        ])
        .split(inner_area);

        let weekday_headers_area = layout[0];
        let days_area = layout[1];

        // Render weekday headers
        let weekdays = ["Mo", "Tu", "We", "Th", "Fr", "Sa", "Su"];
        let weekday_layout = Layout::horizontal(vec![Constraint::Ratio(1, 7); 7]);
        let weekday_cells = weekday_layout.split(weekday_headers_area);
        for (i, weekday) in weekdays.iter().enumerate() {
            Paragraph::new(*weekday)
                .alignment(Alignment::Center)
                .render(weekday_cells[i], buf);
        }

        // Layout for the grid of days (6 weeks to cover all possibilities)
        let weeks_layout = Layout::vertical(vec![Constraint::Ratio(1, 6); 6]).split(days_area);

        let first_day_of_month = NaiveDate::from_ymd_opt(self.year, self.month, 1).unwrap();
        let weekday_of_first = first_day_of_month.weekday(); // Monday=1, Sunday=7
        let start_offset = weekday_of_first.num_days_from_monday() as usize;

        let days_in_month = days_in_month(self.year, self.month);
        let today = Local::now().date_naive();

        let days_with_notes: HashSet<u32> = self
            .notes
            .iter()
            .filter_map(|note| {
                if let Some(file_name) = note.path.file_stem() {
                    if let Some(file_name_str) = file_name.to_str() {
                        if let Ok(date) = NaiveDate::parse_from_str(file_name_str, "%Y-%m-%d") {
                            if date.year() == self.year && date.month() == self.month {
                                return Some(date.day());
                            }
                        }
                    }
                }
                None
            })
            .collect();

        let mut day_counter = 1;
        for (week_index, week_row) in weeks_layout.into_iter().enumerate() {
            let day_cells = weekday_layout.split(*week_row);
            for (day_index, cell) in day_cells.into_iter().enumerate() {
                let current_grid_pos = week_index * 7 + day_index;
                if current_grid_pos >= start_offset && day_counter <= days_in_month {
                    let mut style = Style::default();

                    if days_with_notes.contains(&day_counter) {
                        style = style.fg(Color::Green);
                    }

                    // Highlight today's date
                    if self.year == today.year()
                        && self.month == today.month()
                        && day_counter == today.day()
                    {
                        style = style.add_modifier(Modifier::BOLD).bg(Color::Blue);
                    }

                    Paragraph::new(day_counter.to_string())
                        .alignment(Alignment::Center)
                        .style(style)
                        .render(*cell, buf);
                    day_counter += 1;
                }
            }
        }
    }
}

/// Helper function to get the number of days in a given month and year.
fn days_in_month(year: i32, month: u32) -> u32 {
    NaiveDate::from_ymd_opt(
        if month == 12 { year + 1 } else { year },
        if month == 12 { 1 } else { month + 1 },
        1,
    )
    .unwrap()
    .signed_duration_since(NaiveDate::from_ymd_opt(year, month, 1).unwrap())
    .num_days() as u32
}

/// Helper function to get the name of a month from its number.
fn month_name(month: u32) -> &'static str {
    match month {
        1 => "January",
        2 => "February",
        3 => "March",
        4 => "April",
        5 => "May",
        6 => "June",
        7 => "July",
        8 => "August",
        9 => "September",
        10 => "October",
        11 => "November",
        12 => "December",
        _ => "Unknown",
    }
}

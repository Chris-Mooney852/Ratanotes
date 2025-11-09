mod app;
mod components;
mod utils;

use app::app::{App, restore_terminal, setup_terminal};
use std::io;

fn main() -> io::Result<()> {
    // Setup the terminal
    let mut terminal = setup_terminal()?;

    // Create and run the app
    let mut app = App::new();
    let result = app.run(&mut terminal);

    // Restore the terminal
    restore_terminal(&mut terminal)?;

    if let Err(err) = result {
        println!("Error: {:?}", err);
    }

    Ok(())
}

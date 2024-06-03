use std::io::{self, stdout, Stdout};

use crossterm::{execute, terminal::*};
use ratatui::prelude::*;
pub type Tui = Terminal<CrosstermBackend<Stdout>>;

// initialise terminal
pub fn init() -> io::Result<Tui> {
    //! Clears the terminal preparing it for the TUI, and returns a sesult of the Tui struct
    execute!(stdout(), EnterAlternateScreen)?;
    enable_raw_mode()?;
    Terminal::new(CrosstermBackend::new(stdout()))
}

// unuinitialise terminal at end of the program
pub fn restore() -> io::Result<()> {
    //! Restores terminal to its previous state, returning an empty result
    execute!(stdout(), LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}

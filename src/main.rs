use std::{env, io};

pub mod app;
pub mod tui;

// long term i need to have some kind of queue for api requests to be handled non-blockingly. rn im just happy to have something useable

fn main() -> io::Result<()> {
    // initialise terminal ready for render
    let mut terminal = tui::init()?;
    // initialise app and api client
    let token = match env::var("TODOIST_TOKEN") {
        Err(_) => panic!("Check that TODOIST_TOKEN environment variable is set"),
        Ok(token) => token,
    };
    let mut app = app::App::new(token);
    // run application
    let app_result = app.run(&mut terminal);
    // return terminal to default state
    tui::restore().unwrap();
    app_result
}

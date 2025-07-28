use std::env;
mod app;
mod cache;
mod tui;

// new api https://developer.todoist.com/api/v1 update needed to comply
// also i'm gonna finally do non-blocking SOON i swear

fn exit_in_tui(message: &str, code: u16) {
    tui::restore().unwrap();
    println!("{message}");
    std::process::exit(code as i32);
}

fn main() {
    // read token
    let token = match env::var("TODOIST_TOKEN") {
        Err(_) => {
            println!("Error reading token.\nPlease set the TODOIST_TOKEN environment variable to your API token found in your Todoist settings.\nGo to: https://app.todoist.com/app/settings/integrations/developer");
            std::process::exit(1);
        }
        Ok(token) => token,
    };
    // initialise terminal ready for render
    let mut terminal = match tui::init() {
        Ok(termbackend) => termbackend,
        Err(_) => {
            println!(
                "Error creating TUI terminal backend. Is your terminal compatible with crossterm?"
            );
            std::process::exit(5)
        }
    };
    // initialise app and api client
    let mut app_client = app::App::new(token);
    // run application
    match app_client.run(&mut terminal) {
        Ok(_) => {},
        Err(2) => {exit_in_tui("Response from server was unexpected, and could not be parsed from JSON into the nessasary objects", 2)}
        Err(3) => {exit_in_tui("HTTP POST failed, maybe check your internet connection?", 3)}
        Err(4) => {exit_in_tui("Crossterm incorrectly read an event, is your terminal supported by crossterm?", 4)}
        Err(147) => {exit_in_tui("Authentication failed in an API request, please check that your token (echo $TODOIST_TOKEN) is valid", 147)}
        Err(401) => {exit_in_tui("Authentication failed in an API request, please check that your token (echo $TODOIST_TOKEN) is valid", 401)}
        Err(x) if x > 200 => {exit_in_tui("HTTP error code if between 200 and 600, otherwise unknown (inspect error code with $?, $status or whatever your shell uses to find out what one) (if the exit code is 147 it's probably your token being invalid idk why my error stuff isn't working.)", x)}
        Err(x) => {exit_in_tui("unknown error :(", x)}
    };
    // return terminal to default state
    exit_in_tui("bye bye!", 0);
}

use std::env;

mod app;
mod tui;

// long term i need to have some kind of queue for api requests to be handled non-blockingly. rn im just happy to have something useable

// error code list
//
// 1*: TODOIST_TOKEN not able to be read by env::var -> make sure TODOIST_TOKEN is set to your API token
// 2: json reponse from get_tasks API call does not match expected response and could not be serialized
// 3: reqwest post could not send(), most lightly a network error -> check internet connection
// 4: crossterm failed to read event correctly
// 5*: Tui could not be initialised
// 6: response.text() failed in post request
// 200-600: represent their respective HTTP status codes. Only 200 OK does not produce an error

fn exit_in_tui(message: &str, code: u16) {
    tui::restore().unwrap();
    println!("{}", message);
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
        Err(6) => {exit_in_tui("HTTP POST response failed to be read using text(), idk why sorry", 6)}
        Err(401) => {exit_in_tui("Authentication failed in an API request, please check that your token (echo $TODOIST_TOKEN) is valid", 401)}
        Err(x) => {exit_in_tui("HTTP error code if between 200 and 600, otherwise unknown (inspect error code with $?, $status or whatever your shell uses to find out what one)", x)}
    };
    // return terminal to default state
    exit_in_tui("bye bye!", 0);
}

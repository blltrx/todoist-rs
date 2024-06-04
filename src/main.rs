use std::{env, io};

mod app;
mod tui;

// long term i need to have some kind of queue for api requests to be handled non-blockingly. rn im just happy to have something useable

// error code list
//
// 1: TODOIST_TOKEN not able to be read by env::var -> make sure TODOIST_TOKEN is set to your API token
// 2: json reponse from get_tasks API call does not match expected response and could not be serialized
// 3: reqwest post could not send(), most lightly a network error -> check internet connection
// 200-600: represent their respective HTTP status codes. Only 200 OK does not produce an error

fn main() -> io::Result<()> {
    // read token
    let token = match env::var("TODOIST_TOKEN") {
        Err(_) => {
            println!("Error reading token.\nPlease set the TODOIST_TOKEN environment variable to your API token found in your Todoist settings.\nGo to: https://app.todoist.com/app/settings/integrations/developer");
            std::process::exit(1);
        }
        Ok(token) => token,
    };
    // initialise terminal ready for render
    let mut terminal = tui::init()?;
    // initialise app and api client
    let mut app_client = app::App::new(token);
    // run application
    let app_result = app_client.run(&mut terminal);
    // return terminal to default state
    tui::restore().unwrap();
    app_result
}

use std::env;
pub mod api;
pub mod tui;

fn main() {
    let token = env::var("TODOIST_TOKEN").unwrap();
    let client = api::Api::new(token);

    let task_list = client.get_tasks();
    for task in &task_list {
        println!("{:?}", task);
    }
}

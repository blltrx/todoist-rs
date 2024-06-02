# Todoist Rust TUI

This is a Rust based TUI for the Todoist app. It (mostly) uses the REST API and all requests are blocking so it's a bit slow rn. It does however work.

to install, use 
```
cargo install --git https://www.github.com/blltrx/todoist-rs
```

To use it, set the environment variable `TODOIST_TOKEN` to your [Todoist API token](https://todoist.com/help/articles/find-your-api-token-Jpzx9IIlB) and run the executable `todoist-rs`.


todo list
+ allow input for description on task creation
+ consistent API

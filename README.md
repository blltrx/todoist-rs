# Todoist Rust TUI

this is a Rust based TUI for the Todoist app. it uses the Todoist Sync API and all requests are blocking so it's a bit slow rn. it does however work.

to install, use 
```
cargo install --git https://www.github.com/blltrx/todoist-rs
```

to use it, set the environment variable `TODOIST_TOKEN` to your [Todoist API token](https://todoist.com/help/articles/find-your-api-token-Jpzx9IIlB) and run the executable `todoist-rs`.

more information can be found [on my website](https://bellatrix.dev/projects/todoist-rs).

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::widgets::*;

use crate::tui;
mod api;
mod ui;

/// App client struct containing all app state variables
pub struct App {
    client: api::Api,
    position: ListState,
    tasks: Vec<api::Task>,
    current_sync_token: String,
    mode: Mode,
    create_task_input: String,
    exit: bool,
}

enum Mode {
    Normal,
    Create,
    Info,
}

impl App {
    pub fn new(todoist_token: String) -> App {
        //! Returns a newly created App struct, including initiating the API client.
        //! Consumes a String that is the API Token for the Todoist API.
        App {
            client: api::Api::new(todoist_token),
            position: ListState::default(),
            tasks: Vec::new(),
            mode: Mode::Normal,
            current_sync_token: String::from("*"),
            create_task_input: String::new(),
            exit: false,
        }
    }

    pub fn run(&mut self, terminal: &mut tui::Tui) -> Result<(), u16> {
        //! Starts the main loop for the app, returning an empty result.
        //! Takes a &mut tui::Tui used to render the UI.
        //! ```
        //! let mut app = App::new(token);
        //! let app_result = app.run(terminal);
        //! ```
        (self.tasks, self.current_sync_token) = self.client.get_tasks("*")?;
        self.decrement_selection();
        while !self.exit {
            // calls the ui module to create and render widgets
            let _ = terminal.draw(|frame| self.render_frame(frame));
            self.handle_events()?;
        }
        Ok(())
    }

    // renders the task list widget
    fn render_frame(&mut self, frame: &mut ratatui::Frame) {
        let tasks = &self
            .tasks
            .iter()
            .map(|task| task.to_list_string(frame.size().width))
            .collect();

        match self.mode {
            // normal mode just displays the task list
            Mode::Normal => ui::render_normal_ui(frame, tasks, &mut self.position),

            // create task mode
            Mode::Create => {
                ui::render_create_ui(frame, tasks, &mut self.position, &self.create_task_input)
            }
            Mode::Info => {
                let mut taskinfo = String::new();
                if let Some(current_task_index) = self.position.selected() {
                    taskinfo = self.tasks[current_task_index].to_info_string()
                };
                ui::render_info_ui(frame, tasks, &mut self.position, taskinfo)
            }
        }
    }

    fn handle_events(&mut self) -> Result<(), u16> {
        match event::read() {
            Ok(Event::Key(key_event)) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)?
            }
            Ok(_) => {}
            Err(_) => return Err(4),
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) -> Result<(), u16> {
        match self.mode {
            Mode::Normal | Mode::Info => match key_event.code {
                KeyCode::Char('q') => self.exit = true,
                KeyCode::Esc => self.exit = true,

                KeyCode::Char('j') => self.increment_selection(),
                KeyCode::Down => self.increment_selection(),

                KeyCode::Char('k') => self.decrement_selection(),
                KeyCode::Up => self.decrement_selection(),

                KeyCode::Char('U') => {
                    self.current_sync_token = String::from("*");
                    self.sync_tasks()?
                }

                KeyCode::Char('c') => self.complete_current_task()?,

                KeyCode::Enter => self.mode = Mode::Info,

                KeyCode::Char('n') => self.mode = Mode::Create,

                KeyCode::Backspace => self.mode = Mode::Normal,
                _ => {}
            },
            // mode to allow typing for input
            Mode::Create => match key_event.code {
                KeyCode::Enter => {
                    self.add_task()?;
                    self.mode = Mode::Normal
                }
                // transmitts any character types to the input attribute
                KeyCode::Char(input_character) => self.create_task_input.push(input_character),
                // delete last character from input attribute
                KeyCode::Backspace => _ = self.create_task_input.pop(),
                KeyCode::Delete => self.mode = Mode::Normal,
                _ => {}
            },
        };
        Ok(())
    }

    /// selection interaction

    fn increment_selection(&mut self) {
        if self.tasks.is_empty() {
            self.position.select(None);
            return;
        }
        let current = self.position.selected().unwrap_or(0);
        let length = self.tasks.len();
        if current == length - 1 {
            return;
        }
        self.position.select(Some(current + 1));
    }
    fn decrement_selection(&mut self) {
        if self.tasks.is_empty() {
            self.position.select(None);
            return;
        }
        let current = self.position.selected().unwrap_or(0);
        if current == 0 {
            self.position.select(Some(0));
            return;
        }
        self.position.select(Some(current - 1));
    }

    /// API interaction

    fn sync_tasks(&mut self) -> Result<(), u16> {
        let (new_tasks, sync_token) = loop {
            match self.client.get_tasks(&self.current_sync_token) {
                Ok(result) => break result,
                Err(500..=600) => continue,
                Err(error_code) => return Err(error_code),
            }
        };
        if self.current_sync_token == "*" {
            self.tasks = new_tasks
        } else {
            self.tasks.extend(new_tasks);
        }
        self.current_sync_token = sync_token;
        Ok(())
    }

    fn complete_current_task(&mut self) -> Result<(), u16> {
        let current_index = match self.position.selected() {
            Some(index) => index,
            None => return Ok(()),
        };
        self.current_sync_token = loop {
            match self.client.complete_task(&self.tasks[current_index]) {
                Ok(result) => break result,
                Err(500..=600) => continue,
                Err(error_code) => return Err(error_code),
            }
        };
        self.tasks.remove(current_index);
        self.decrement_selection();
        Ok(())
    }

    fn add_task(&mut self) -> Result<(), u16> {
        let new_task = loop {
            match self.client.quick_add(self.create_task_input.clone()) {
                Ok(result) => break result,
                Err(500..=600) => continue,
                Err(error_code) => return Err(error_code),
            }
        };
        self.tasks.push(new_task);
        self.create_task_input = String::new();
        self.current_sync_token = String::from("*");
        Ok(())
    }
}

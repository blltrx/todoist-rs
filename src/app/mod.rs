use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
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
    inputs: Vec<String>,
    input_position: usize,
    exit: bool,
}

enum Mode {
    Normal,
    Create,
    Info,
    Edit,
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
            inputs: vec![
                String::new(),
                String::new(),
                String::new(),
                String::new(),
                String::new(),
            ],
            input_position: 0,
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
            Mode::Create => ui::render_create_ui(frame, tasks, &mut self.position, &self.inputs[0]),
            Mode::Info => {
                let taskinfo = match self.position.selected() {
                    Some(index) => self.tasks[index].to_info_string(),
                    None => {
                        self.mode = Mode::Normal;
                        return;
                    }
                };
                ui::render_info_ui(frame, tasks, &mut self.position, taskinfo)
            }

            // edit mode to edit currently selected task
            Mode::Edit => {
                let (_, title, description, labels, date, priority) = match self.position.selected()
                {
                    Some(index) => self.tasks[index].get_details(),
                    None => {
                        self.mode = Mode::Normal;
                        return;
                    }
                };
                if self
                    .inputs
                    .iter()
                    .map(|x| x == &String::new())
                    .reduce(|a, b| a & b)
                    .unwrap()
                {
                    self.inputs = vec![
                        title,
                        description,
                        labels.join(", "),
                        date,
                        format!("{}", priority),
                    ]
                };
                ui::render_edit_ui(
                    frame,
                    &self.inputs[0],
                    &self.inputs[1],
                    &self.inputs[2],
                    &self.inputs[3],
                    &self.inputs[4],
                )
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

                KeyCode::Char('e') => self.mode = Mode::Edit,

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
                KeyCode::Char(input_character) => self.inputs[0].push(input_character),
                // delete last character from input attribute
                KeyCode::Backspace => _ = self.inputs[0].pop(),
                KeyCode::Delete => self.mode = Mode::Normal,
                _ => {}
            },
            Mode::Edit => match (key_event.code, key_event.modifiers) {
                (KeyCode::Enter, KeyModifiers::NONE) => {
                    self.edit_task()?;
                    self.inputs = self.inputs.iter().map(|_| String::new()).collect();
                    self.input_position = 0;
                    self.mode = Mode::Normal;
                }

                (KeyCode::Tab, KeyModifiers::NONE) | (KeyCode::Down, KeyModifiers::NONE) => {
                    if self.input_position == 4 {
                        self.input_position = 0
                    } else {
                        self.input_position += 1
                    }
                }
                // SHIFT is not working idk why i cant be asked sry
                (KeyCode::Tab, KeyModifiers::SHIFT) | (KeyCode::Up, KeyModifiers::NONE) => {
                    if self.input_position == 0 {
                        self.input_position = 4
                    } else {
                        self.input_position -= 1
                    }
                }

                // transmitts any character types to the input attribute
                (KeyCode::Char(input_character), KeyModifiers::NONE) => {
                    self.inputs[self.input_position].push(input_character)
                }
                // delete last character from input attribute
                (KeyCode::Backspace, KeyModifiers::NONE) => {
                    _ = self.inputs[self.input_position].pop()
                }
                (KeyCode::Delete, KeyModifiers::NONE) | (KeyCode::Esc, KeyModifiers::NONE) => {
                    self.inputs = self.inputs.iter().map(|_| String::new()).collect();
                    self.input_position = 0;
                    self.mode = Mode::Normal;
                }
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
            match self.client.quick_add(self.inputs[0].clone()) {
                Ok(result) => break result,
                Err(500..=600) => continue,
                Err(error_code) => return Err(error_code),
            }
        };
        self.tasks.push(new_task);
        self.inputs[0] = String::new();
        self.current_sync_token = String::from("*");
        Ok(())
    }

    fn edit_task(&mut self) -> Result<(), u16> {
        // get all data
        let id = self.tasks[self.position.selected().unwrap()].get_id();
        let content = self.inputs[0].clone();
        self.inputs[0] = String::new();
        let description = self.inputs[1].clone();
        self.inputs[1] = String::new();
        let labels = self.inputs[2]
            .clone()
            .split(',')
            .map(String::from)
            .collect();
        self.inputs[2] = String::new();
        let date = self.inputs[3].clone();
        self.inputs[3] = String::new();
        let priority = match self.inputs[4].clone().parse() {
            Ok(x) => x,
            Err(_) => return Ok(()),
        }; //TODO
        self.inputs[0] = String::new();

        // create task object
        let task = api::Task::create_task_obj(id, content, description, date, labels, priority);

        // modify task api request
        match self.client.edit(task.clone()) {
            Ok(sync) => {
                self.current_sync_token = sync;
                self.tasks[self.position.selected().unwrap()] = task.clone();
            }
            // Err(2) => {}
            Err(x) => return Err(x),
        };
        Ok(())
    }
}

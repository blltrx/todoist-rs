use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::widgets::*;

use std::io;

use crate::tui;
pub mod api;
pub mod ui;

pub struct App {
    client: api::Api,
    position: ListState,
    tasks: Vec<api::Task>,
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
    /// initialise app struct with api client
    pub fn new(todoist_token: String) -> App {
        App {
            client: api::Api::new(todoist_token),
            position: ListState::default(),
            tasks: Vec::new(),
            mode: Mode::Normal,
            create_task_input: String::new(),
            exit: false,
        }
    }

    /// runs the main loop for the app
    pub fn run(&mut self, terminal: &mut tui::Tui) -> io::Result<()> {
        self.tasks = self.client.get_tasks();
        self.position.select(Some(0));
        while !self.exit {
            // calls the ui module to create and render widgets
            terminal.draw(|frame| self.render_frame(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    // renders the task list widget
    fn render_frame(&mut self, frame: &mut ratatui::Frame) {
        match self.mode {
            // normal mode just displays the task list
            Mode::Normal => ui::render_normal_ui(
                frame,
                &api::tasklist_to_strings(&self.tasks, frame.size().width),
                &mut self.position,
            ),

            // create task mode
            Mode::Create => ui::render_create_ui(
                frame,
                &api::tasklist_to_strings(&self.tasks, frame.size().width),
                &mut self.position,
                &self.create_task_input,
            ),
            Mode::Info => {
                let mut taskinfo = String::new();
                if let Some(current_task_index) = self.position.selected() {
                    taskinfo = api::task_to_string(&self.tasks[current_task_index])
                };
                ui::render_info_ui(
                    frame,
                    &api::tasklist_to_strings(&self.tasks, frame.size().width),
                    &mut self.position,
                    taskinfo,
                )
            }
        }
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match self.mode {
            Mode::Normal => match key_event.code {
                KeyCode::Char('q') => self.exit = true,
                KeyCode::Esc => self.exit = true,

                KeyCode::Char('j') => self.increment_selection(),
                KeyCode::Down => self.increment_selection(),

                KeyCode::Char('k') => self.decrement_selection(),
                KeyCode::Up => self.decrement_selection(),

                KeyCode::Char('u') => self.tasks = self.client.get_tasks(),

                KeyCode::Char('c') => self.complete_current_task(),
                KeyCode::Delete => self.complete_current_task(),

                KeyCode::Enter => self.mode = Mode::Info,

                KeyCode::Char('n') => self.mode = Mode::Create,
                _ => {}
            },
            // mode to allow typing for input
            Mode::Create => match key_event.code {
                KeyCode::Enter => {
                    self.add_task();
                    self.mode = Mode::Normal
                }
                // transmitts any character types to the input attribute
                KeyCode::Char(input_character) => self.create_task_input.push(input_character),
                // delete last character from input attribute
                KeyCode::Backspace => _ = self.create_task_input.pop(),
                KeyCode::Delete => self.mode = Mode::Normal,
                _ => {}
            },
            Mode::Info => match key_event.code {
                KeyCode::Char('q') => self.exit = true,
                KeyCode::Esc => self.exit = true,

                KeyCode::Char('j') => self.increment_selection(),
                KeyCode::Down => self.increment_selection(),

                KeyCode::Char('k') => self.decrement_selection(),
                KeyCode::Up => self.decrement_selection(),

                KeyCode::Char('u') => self.tasks = self.client.get_tasks(),

                KeyCode::Char('c') => self.complete_current_task(),
                KeyCode::Delete => self.complete_current_task(),

                KeyCode::Char('n') => self.mode = Mode::Create,

                KeyCode::Backspace => self.mode = Mode::Normal,
                _ => {}
            },
        }
    }

    /// selection interaction

    fn increment_selection(&mut self) {
        let current = self.position.selected().unwrap_or(0);
        let length = self.tasks.len();
        if current == length - 1 {
            return;
        }
        self.position.select(Some(current + 1));
    }
    fn decrement_selection(&mut self) {
        let current = self.position.selected().unwrap_or(0);
        if current == 0 {
            return;
        }
        self.position.select(Some(current - 1));
    }

    /// API interaction

    fn complete_current_task(&mut self) {
        let current = self.position.selected().unwrap_or(0);
        if self.tasks.is_empty() {
            return;
        };
        self.client.complete_task(&self.tasks[current]);
        self.tasks = self.client.get_tasks();
    }

    fn add_task(&mut self) {
        self.client.quick_add(self.create_task_input.to_owned());
        self.create_task_input = String::new();
        self.tasks = self.client.get_tasks();
    }
}

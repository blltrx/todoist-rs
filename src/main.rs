use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{prelude::*, widgets::*};

use std::{env, io};

pub mod api;
pub mod tui;
pub mod ui;

// mostly based on the basic tutorial on the ratatui docs
pub struct App {
    client: api::Api,
    position: ListState,
    tasks: Vec<api::Task>,
    exit: bool,
}

impl App {
    // initialise app struct with api client
    pub fn new(todoist_token: String) -> App {
        return App {
            client: api::Api::new(todoist_token),
            position: ListState::default(),
            tasks: Vec::new(),
            exit: false,
        };
    }

    // runs the main loop for the app
    pub fn run(&mut self, terminal: &mut tui::Tui) -> io::Result<()> {
        self.tasks = self.client.get_tasks();
        self.position.select(Some(0));
        while !self.exit {
            terminal.draw(|frame| self.render_frame(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    // renders the task list widget
    fn render_frame(&mut self, frame: &mut Frame) {
        frame.render_stateful_widget(
            ui::make_list_widget(&self.tasks, frame.size().width),
            frame.size(),
            &mut self.position,
        );
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
        match key_event.code {
            KeyCode::Char('q') => self.exit = true,

            KeyCode::Char('j') => self.increment_selection(),
            KeyCode::Down => self.increment_selection(),

            KeyCode::Char('k') => self.decrement_selection(),
            KeyCode::Up => self.decrement_selection(),

            KeyCode::Char('u') => self.tasks = self.client.get_tasks(),

            KeyCode::Char('c') => self.complete_current_task(),
            KeyCode::Delete => self.complete_current_task(),

            // KeyCode::Char('n') => self.create_task(),
            _ => {}
        }
    }

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

    fn complete_current_task(&mut self) {
        let current = self.position.selected().unwrap_or(0);
        self.client.complete_task(&self.tasks[current]);
        self.tasks = self.client.get_tasks();
    }
}

fn main() -> io::Result<()> {
    // initialise terminal ready for render
    let mut terminal = tui::init()?;
    // initialise app and api client
    let token = env::var("TODOIST_TOKEN").unwrap(); // TODO error handle this pls
    let mut app = App::new(token);
    let app_result = app.run(&mut terminal);
    // return terminal to default state
    tui::restore().unwrap();
    return app_result;
}

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    prelude::*,
    widgets::{block::*, *},
};

use std::{env, io};

pub mod api;
pub mod tui;
pub mod ui;

pub struct App {
    client: api::Api,
    position: ListState,
    tasks: Vec<api::Task>,
    mode: Mode,
    input: String,
    exit: bool,
}

enum Mode {
    Normal,
    Create,
}

impl App {
    /// initialise app struct with api client
    pub fn new(todoist_token: String) -> App {
        return App {
            client: api::Api::new(todoist_token),
            position: ListState::default(),
            tasks: Vec::new(),
            mode: Mode::Normal,
            input: String::new(),
            exit: false,
        };
    }

    /// runs the main loop for the app
    pub fn run(&mut self, terminal: &mut tui::Tui) -> io::Result<()> {
        self.tasks = self.client.get_tasks();
        self.position.select(Some(0));
        while !self.exit {
            terminal.draw(|frame| self.render_frame(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    /// renders the task list widget
    fn render_frame(&mut self, frame: &mut Frame) {
        match self.mode {
            Mode::Normal => frame.render_stateful_widget(
                ui::make_list_widget(&self.tasks, frame.size().width),
                frame.size(),
                &mut self.position,
            ),
            Mode::Create => {
                let layout = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints(vec![Constraint::Max(4), Constraint::Fill(1)])
                    .split(frame.size());
                let footer = Title::from(Line::from(vec![
                    " delete ".blue().into(),
                    "to exit create mode - ".into(),
                    "enter ".blue().into(),
                    "to create - ".into(),
                ]));
                frame.render_widget(
                    Paragraph::new(self.input.as_str())
                        .style(Style::default().fg(Color::LightMagenta))
                        .block(
                            Block::bordered().title("Create Task").title(
                                footer
                                    .alignment(Alignment::Center)
                                    .position(Position::Bottom),
                            ),
                        ),
                    layout[0],
                );
                frame.render_stateful_widget(
                    ui::make_list_widget(&self.tasks, frame.size().width),
                    layout[1],
                    &mut self.position,
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

                KeyCode::Char('j') => self.increment_selection(),
                KeyCode::Down => self.increment_selection(),

                KeyCode::Char('k') => self.decrement_selection(),
                KeyCode::Up => self.decrement_selection(),

                KeyCode::Char('u') => self.tasks = self.client.get_tasks(),

                KeyCode::Char('c') => self.complete_current_task(),
                KeyCode::Delete => self.complete_current_task(),

                KeyCode::Char('n') => self.create_task(),
                _ => {}
            },
            Mode::Create => match key_event.code {
                KeyCode::Enter => {
                    self.add_task();
                    self.mode = Mode::Normal
                }
                KeyCode::Char(to_insert) => self.enter_character(to_insert),
                KeyCode::Backspace => self.delete_character(),
                KeyCode::Delete => self.mode = Mode::Normal,
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
        if self.tasks.len() == 0 {
            return;
        };
        self.client.complete_task(&self.tasks[current]);
        self.tasks = self.client.get_tasks();
    }

    fn create_task(&mut self) {
        self.mode = Mode::Create
    }

    fn add_task(&mut self) {
        self.client.quick_add(self.input.to_owned());
        self.input = String::new();
        self.tasks = self.client.get_tasks();
    }

    /// user input interaction

    fn enter_character(&mut self, input: char) {
        self.input.push(input);
    }

    fn delete_character(&mut self) {
        self.input.pop();
    }
}

// long term i need to split this up a bit more, and have some kind of queue for api requests to be handled non-blockingly. rn im just happy to have something useable

fn main() -> io::Result<()> {
    // initialise terminal ready for render
    let mut terminal = tui::init()?;
    // initialise app and api client
    let token = match env::var("TODOIST_TOKEN") {
        Err(_) => panic!("Check that TODOIST_TOKEN environment variable is set"),
        Ok(token) => token,
    };
    let mut app = App::new(token);
    let app_result = app.run(&mut terminal);
    // return terminal to default state
    tui::restore().unwrap();
    return app_result;
}

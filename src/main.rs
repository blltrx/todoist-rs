use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    prelude::*,
    symbols::border,
    widgets::{block::*, *},
};
use std::{env, io};

pub mod api;
pub mod tui;

// mostly based on the basic tutorial on the ratatui docs
pub struct App {
    client: api::Api,
    position: u8,
    tasks: Vec<api::Task>,
    exit: bool,
}

impl App {
    // initialise app struct with api client
    pub fn new(todoist_token: String) -> App {
        return App {
            client: api::Api::new(todoist_token),
            position: 0,
            tasks: Vec::new(),
            exit: false,
        };
    }

    // runs the main loop for the app
    pub fn run(&mut self, terminal: &mut tui::Tui) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.render_frame(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn render_frame(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.size());
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
            _ => {}
        }
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let header = Title::from(" todo ".italic());
        let footer = Title::from(Line::from(vec![
            " c ".blue().into(),
            "to complete - ".into(),
            "n ".blue().into(),
            "to create - ".into(),
            "u ".blue().into(),
            "to update ".into(),
        ]));
        let body = Text::from("body text");

        let block = Block::default()
            .title(header.alignment(Alignment::Center))
            .title(
                footer
                    .alignment(Alignment::Center)
                    .position(Position::Bottom),
            )
            .borders(Borders::ALL)
            .border_set(border::PLAIN);

        Paragraph::new(body)
            .centered()
            .block(block)
            .render(area, buf);
    }
}

fn main() -> io::Result<()> {
    // initialise terminal ready for render
    let mut terminal = tui::init()?;
    // initialise app and api client
    let token = env::var("TODOIST_TOKEN").unwrap();
    let mut app = App::new(token);
    let app_result = app.run(&mut terminal);
    // return terminal to default state
    tui::restore().unwrap();
    return app_result;
}

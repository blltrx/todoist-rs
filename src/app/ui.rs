use ratatui::{
    prelude::*,
    symbols::border,
    widgets::{block::*, *},
};

use crate::app::api;

pub fn render_normal_ui(frame: &mut Frame, tasks: &Vec<api::Task>, position: &mut ListState) {
    frame.render_stateful_widget(
        make_list_widget(tasks, frame.size().width),
        frame.size(),
        position,
    )
}

pub fn render_create_ui(
    frame: &mut Frame,
    tasks: &Vec<api::Task>,
    position: &mut ListState,
    create_input: &String,
) {
    // create vertical layout
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Max(4), Constraint::Fill(1)])
        .split(frame.size());

    frame.render_widget(make_input_widget(create_input), layout[0]);
    frame.render_stateful_widget(
        make_list_widget(tasks, frame.size().width),
        layout[1],
        position,
    )
}

fn make_list_widget(list_elements: &Vec<api::Task>, width: u16) -> List {
    // calculate sizing based on frame width
    let content_length: usize = (width as f32 * 0.6).round() as usize;
    let (mut spacer_length, overflow) = usize::overflowing_sub(width as usize, content_length + 17);
    if overflow {
        spacer_length = 2;
    };

    // setup formatting
    let header = Title::from(" todo ".bold().magenta());
    let footer = Title::from(Line::from(vec![
        " c ".magenta().into(),
        "to complete - ".into(),
        "n ".magenta().into(),
        "to create - ".into(),
        "u ".magenta().into(),
        "to update ".into(),
    ]));

    // create list of tasks
    let items = list_elements.iter().map(|task| {
        format!(
            "{:content_length$}{:spacer_length$}{:10}  {:1}",
            task.content
                .chars()
                .take(content_length)
                .collect::<String>(),
            " ",
            match &task.due {
                None => String::from("not due"),
                Some(x) => x.date.to_owned(),
            },
            task.priority,
        )
    });

    // create widget containter
    let block = Block::default()
        .title(header.alignment(Alignment::Center))
        .title(
            footer
                .alignment(Alignment::Center)
                .position(Position::Bottom),
        )
        .borders(Borders::ALL)
        .border_set(border::PLAIN);

    return List::new(items)
        .block(block)
        .highlight_symbol("> ")
        .highlight_style(Style::new().magenta())
        .style(Style::new().blue())
        .highlight_spacing(HighlightSpacing::Always);
}

fn make_input_widget(current_input: &String) -> Paragraph {
    let footer = Title::from(Line::from(vec![
        " delete ".blue().into(),
        "to exit create mode - ".into(),
        "enter ".blue().into(),
        "to create - ".into(),
    ]));

    return Paragraph::new(current_input.as_str())
        .style(Style::default().fg(Color::LightMagenta))
        .block(
            Block::bordered().title("Create Task").title(
                footer
                    .alignment(Alignment::Center)
                    .position(Position::Bottom),
            ),
        );
}

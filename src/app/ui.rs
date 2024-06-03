use ratatui::{
    prelude::*,
    symbols::border,
    widgets::{block::*, *},
};

pub fn render_normal_ui(frame: &mut Frame, tasks: &Vec<String>, position: &mut ListState) {
    frame.render_stateful_widget(make_list_widget(tasks), frame.size(), position)
}

pub fn render_create_ui(
    frame: &mut Frame,
    tasks: &Vec<String>,
    position: &mut ListState,
    create_input: &str,
) {
    // create vertical layout
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Max(4), Constraint::Fill(1)])
        .split(frame.size());

    frame.render_widget(make_input_widget(create_input), layout[0]);
    frame.render_stateful_widget(make_list_widget(tasks), layout[1], position)
}

pub fn render_info_ui(
    frame: &mut Frame,
    tasks: &Vec<String>,
    position: &mut ListState,
    taskinfo: String,
) {
    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(frame.size());
    frame.render_stateful_widget(make_list_widget(tasks), layout[0], position);
    frame.render_widget(make_info_widget(&taskinfo), layout[1]);
}

fn make_list_widget(items: &Vec<String>) -> List {
    // setup formatting
    let header = Title::from(" todo ".bold().magenta());
    let footer = Title::from(Line::from(vec![
        " c ".magenta(),
        "to complete - ".into(),
        "n ".magenta(),
        "to create - ".into(),
        "u ".magenta(),
        "to update ".into(),
    ]));

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

    return List::new(items.to_owned())
        .block(block)
        .highlight_symbol("> ")
        .highlight_style(Style::new().magenta())
        .style(Style::new().blue())
        .highlight_spacing(HighlightSpacing::Always);
}

fn make_input_widget(current_input: &str) -> Paragraph {
    let footer = Title::from(Line::from(vec![
        " delete ".blue(),
        "to exit create mode - ".into(),
        "enter ".blue(),
        "to create - ".into(),
    ]));

    return Paragraph::new(current_input)
        .style(Style::default().fg(Color::LightMagenta))
        .block(
            Block::bordered().title("Create Task").title(
                footer
                    .alignment(Alignment::Center)
                    .position(Position::Bottom),
            ),
        );
}

fn make_info_widget(taskinfo: &str) -> Paragraph {
    let footer = Title::from(Line::from(vec![
        " <backspace> ".magenta(),
        "to close ".into(),
    ]));
    return Paragraph::new(taskinfo)
        .block(
            Block::bordered()
                .title(Title::from(" task infomation ".italic().magenta()))
                .title(
                    footer
                        .alignment(Alignment::Center)
                        .position(Position::Bottom),
                )
                .style(Style::new().fg(Color::Blue)),
        )
        .style(Style::new().fg(Color::White))
        .wrap(Wrap { trim: true });
}

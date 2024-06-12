use ratatui::{
    prelude::*,
    symbols::border,
    widgets::{block::*, *},
};

pub fn render_normal_ui(frame: &mut Frame, tasks: &Vec<String>, position: &mut ListState) {
    //! Using &mut Frame renders the main list as a stateful widget
    frame.render_stateful_widget(list(tasks), frame.size(), position)
}

pub fn render_create_ui(
    frame: &mut Frame,
    tasks: &Vec<String>,
    position: &mut ListState,
    create_input: &str,
) {
    //! Using &mut Frame renders the main list as a stateful widget, and the input box widget.
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Max(4), Constraint::Fill(1)])
        .split(frame.size());

    frame.render_widget(
        input_box(create_input, String::from("Create Task")),
        layout[0],
    );
    frame.render_stateful_widget(list(tasks), layout[1], position)
}

pub fn render_info_ui(
    frame: &mut Frame,
    tasks: &Vec<String>,
    position: &mut ListState,
    taskinfo: String,
) {
    //! Using &mut Frame renders the main list as a stateful widget and the info panel widget.
    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(frame.size());

    frame.render_stateful_widget(list(tasks), layout[0], position);
    frame.render_widget(infomation_panel(&taskinfo), layout[1]);
}

pub fn render_edit_ui(
    frame: &mut Frame,
    title: &str,
    description: &str,
    labels: &str,
    date: &str,
    priority: &str,
) {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Max(4),
            Constraint::Fill(1),
            Constraint::Max(4),
            Constraint::Max(4),
            Constraint::Max(4),
        ])
        .split(frame.size());

    frame.render_widget(
        multiple_input_box(title, String::from(" title ")),
        layout[0],
    );
    frame.render_widget(
        multiple_input_box(description, String::from(" description ")),
        layout[1],
    );
    frame.render_widget(
        multiple_input_box(labels, String::from(" labels (comma seperated) ")),
        layout[2],
    );
    frame.render_widget(
        multiple_input_box(date, String::from(" date (ISO formatted) ")),
        layout[3],
    );
    frame.render_widget(
        multiple_input_box(priority, String::from(" priority (1-4 inclusive) ")),
        layout[4],
    );
}

fn list(items: &Vec<String>) -> List {
    // setup formatting
    let header = Title::from(" todo ".bold().magenta());
    let footer = Title::from(Line::from(vec![
        " c ".magenta(),
        "to complete - ".into(),
        "n ".magenta(),
        "to create - ".into(),
        "u ".magenta(),
        "to update ".into(),
        "e ".magenta(),
        "to edit ".into(),
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

fn input_box(current_input: &str, title: String) -> Paragraph {
    let footer = Title::from(Line::from(vec![
        " delete ".light_blue(),
        "to exit mode - ".into(),
        "enter ".light_blue(),
        "to confirm - ".into(),
    ]));

    return Paragraph::new(current_input)
        .style(Style::default().fg(Color::LightMagenta))
        .block(
            Block::bordered().title(title).title(
                footer
                    .alignment(Alignment::Center)
                    .position(Position::Bottom),
            ),
        )
        .wrap(Wrap { trim: true });
}

fn multiple_input_box(current_input: &str, title: String) -> Paragraph {
    return Paragraph::new(current_input)
        .style(Style::default().fg(Color::White))
        .block(Block::bordered().title(title.magenta()).light_blue())
        .wrap(Wrap { trim: true });
}

fn infomation_panel(taskinfo: &str) -> Paragraph {
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
                .style(Style::new().fg(Color::LightBlue))
                .style(Style::new().fg(Color::White)),
        )
        .wrap(Wrap { trim: true });
}

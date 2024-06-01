use ratatui::{
    prelude::*,
    symbols::border,
    widgets::{block::*, *},
};

use crate::api;

pub fn make_list_widget(list_elements: &Vec<api::Task>) -> List {
    let header = Title::from(" todo ".italic());
    let footer = Title::from(Line::from(vec![
        " c ".blue().into(),
        "to complete - ".into(),
        "n ".blue().into(),
        "to create - ".into(),
        "u ".blue().into(),
        "to update ".into(),
    ]));
    let items = list_elements.iter().map(|task| {
        format!(
            "{}  {}  {}  {}",
            task.content,
            task.description,
            task.priority,
            match &task.due {
                None => String::from("not due"),
                Some(x) => x.date.to_owned(),
            }
        )
    });

    let block = Block::default()
        .title(header.alignment(Alignment::Center))
        .title(
            footer
                .alignment(Alignment::Center)
                .position(Position::Bottom),
        )
        .borders(Borders::ALL)
        .border_set(border::PLAIN);

    return List::new(items).block(block).highlight_symbol(">");
}

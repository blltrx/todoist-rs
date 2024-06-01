use ratatui::{
    prelude::*,
    symbols::border,
    widgets::{block::*, *},
};

use crate::api;

// this along with the App needs to be refactored, like why is this here?

pub fn make_list_widget(list_elements: &Vec<api::Task>, width: u16) -> List {
    let content_length: usize = (width as f32 * 0.6).round() as usize;
    let (mut spacer_length, overflow) = usize::overflowing_sub(width as usize, content_length + 17);
    if overflow {
        spacer_length = 2;
    };

    let header = Title::from(" todo ".bold().magenta());
    let footer = Title::from(Line::from(vec![
        " c ".magenta().into(),
        "to complete - ".into(),
        "n ".magenta().into(),
        "to create - ".into(),
        "u ".magenta().into(),
        "to update ".into(),
    ]));
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

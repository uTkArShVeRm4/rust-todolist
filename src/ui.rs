use std::fmt::write;

use chrono::Datelike;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    prelude::{Alignment, Frame},
    style::{Color, Modifier, Style, Stylize},
    text::Line,
    widgets::{
        calendar::{CalendarEventStore, DateStyler, Monthly},
        Block, BorderType, Borders, List, Paragraph,
    },
};

use time;

use crate::app::{App, CurrentScreen};

fn centered_rect(r: Rect, percent_x: u16, percent_y: u16) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

pub fn render(app: &mut App, f: &mut Frame) {
    let screen_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(20),
            Constraint::Percentage(20),
            Constraint::Percentage(60),
        ])
        .split(f.size());

    let list_layout = screen_layout[0];
    let inputs_layout = screen_layout[1];

    let inputs_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(inputs_layout);

    let list = List::new(app.todo_items.clone())
        .block(
            Block::default()
                .title("Todolist")
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
        .style(Style::default().fg(Color::Yellow));

    let input_text = Line::from(app.current_input.clone());

    let now = chrono::Utc::now();

    let date = time::Date::from_ordinal_date(now.year(), app.current_date as u16).unwrap();
    // let today_style = CalendarEventStore::today(Style::new().yellow());

    let hint_text = Paragraph::new(
        "Esc to exit, Tab to move between screen.\nUp and Down to move inside the screen.",
    )
    .alignment(Alignment::Center)
    .yellow();

    match app.current_screen {
        CurrentScreen::Todolist => {
            let mut today_style = CalendarEventStore::default();
            today_style.add(
                date,
                Style::default().add_modifier(Modifier::UNDERLINED), // .add_modifier(Modifier::SLOW_BLINK),
            );
            let calendar = Monthly::new(date, today_style)
                .block(
                    Block::default()
                        .title("Deadline")
                        .title_alignment(Alignment::Center)
                        .borders(Borders::ALL)
                        .border_type(BorderType::Rounded)
                        .yellow(),
                )
                .show_month_header(Style::default().yellow());

            f.render_stateful_widget(
                list.highlight_style(
                    Style::default()
                        .add_modifier(Modifier::UNDERLINED)
                        .add_modifier(Modifier::SLOW_BLINK),
                ),
                list_layout,
                &mut app.todo_select_state,
            );
            f.render_widget(
                Paragraph::new(input_text)
                    .block(
                        Block::default()
                            .title("Input")
                            .title_alignment(Alignment::Center)
                            .borders(Borders::ALL)
                            .border_type(BorderType::Rounded),
                    )
                    .style(Style::default().fg(Color::Yellow)),
                inputs_layout[0],
            );
            f.render_widget(calendar, inputs_layout[1]);
            f.render_widget(hint_text, screen_layout[2]);
        }
        CurrentScreen::Input => {
            let mut today_style = CalendarEventStore::default();
            today_style.add(
                date,
                Style::default().add_modifier(Modifier::UNDERLINED), // .add_modifier(Modifier::SLOW_BLINK),
            );
            let calendar = Monthly::new(date, today_style)
                .block(
                    Block::default()
                        .title("Deadline")
                        .title_alignment(Alignment::Center)
                        .borders(Borders::ALL)
                        .border_type(BorderType::Rounded)
                        .yellow(),
                )
                .show_month_header(Style::default().yellow());

            f.render_stateful_widget(list, list_layout, &mut app.todo_select_state);
            f.render_widget(
                Paragraph::new(input_text)
                    .block(
                        Block::default()
                            .title("Input")
                            .title_alignment(Alignment::Center)
                            .title_style(Style::default().add_modifier(Modifier::UNDERLINED))
                            .borders(Borders::ALL)
                            .border_type(BorderType::Rounded),
                    )
                    .style(Style::default().fg(Color::Yellow)),
                inputs_layout[0],
            );
            f.render_widget(calendar, inputs_layout[1]);

            f.render_widget(hint_text, screen_layout[2]);
        }
        CurrentScreen::Deadline => {
            let mut today_style = CalendarEventStore::default();
            today_style.add(
                date,
                Style::default()
                    .add_modifier(Modifier::UNDERLINED)
                    .add_modifier((Modifier::SLOW_BLINK)), // .add_modifier(Modifier::SLOW_BLINK),
            );
            let calendar = Monthly::new(date, today_style)
                .block(
                    Block::default()
                        .title("Deadline")
                        .title_alignment(Alignment::Center)
                        .borders(Borders::ALL)
                        .border_type(BorderType::Rounded)
                        .yellow(),
                )
                .show_month_header(Style::default().yellow());

            f.render_stateful_widget(list, list_layout, &mut app.todo_select_state);
            f.render_widget(
                Paragraph::new(input_text)
                    .block(
                        Block::default()
                            .title("Input")
                            .title_alignment(Alignment::Center)
                            .borders(Borders::ALL)
                            .border_type(BorderType::Rounded),
                    )
                    .style(Style::default().fg(Color::Yellow)),
                inputs_layout[0],
            );
            f.render_widget(calendar, inputs_layout[1]);
            f.render_widget(hint_text, screen_layout[2]);
        }
    };
}

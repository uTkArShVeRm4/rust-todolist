use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    prelude::{Alignment, Frame},
    style::{Color, Modifier, Style, Stylize},
    text::Line,
    widgets::{Block, BorderType, Borders, List, Paragraph},
};

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
            Constraint::Percentage(10),
            Constraint::Percentage(70),
        ])
        .split(f.size());

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

    match app.current_screen {
        CurrentScreen::Todolist => {
            f.render_stateful_widget(
                list.highlight_style(Style::default().add_modifier(Modifier::UNDERLINED)),
                screen_layout[0],
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
                screen_layout[1],
            );
        }
        CurrentScreen::Input => {
            f.render_stateful_widget(list, screen_layout[0], &mut app.todo_select_state);
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
                screen_layout[1],
            )
        }
    };
}

use ratatui::{
    prelude::{Alignment, Frame},
    style::{Color, Modifier, Style, Stylize},
    widgets::{Block, BorderType, Borders, List, Paragraph},
};

use crate::app::App;

pub fn render(app: &mut App, f: &mut Frame) {
    f.render_stateful_widget(
        List::new(app.todo_items.clone())
            .block(
                Block::default()
                    .title("Todolist")
                    .title_alignment(Alignment::Center)
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded),
            )
            .style(Style::default().fg(Color::Yellow))
            .highlight_symbol(">>")
            .highlight_style(Style::default().add_modifier(Modifier::UNDERLINED)),
        f.size(),
        &mut app.todo_select_state,
    );
}

use ratatui::{
    prelude::{Alignment, Frame},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, List},
};

use crate::app::App;

pub fn render(app: &mut App, f: &mut Frame) {
    f.render_stateful_widget(
        List::new(app.todo_items.clone())
            .block(
                Block::default()
                    .title("Counter App")
                    .title_alignment(Alignment::Center)
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded),
            )
            .style(Style::default().fg(Color::Yellow))
            .highlight_symbol(">>"),
        f.size(),
        &mut app.todo_select_state,
    )
}

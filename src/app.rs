use ratatui::widgets::ListState;
use std::cmp::{max, min};
#[derive(Debug, Default)]
pub struct App {
    pub should_quit: bool,
    pub todo_items: Vec<String>,
    pub todo_select_state: ListState,
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set should_quit to true to quit the application.
    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    pub fn liststate_increment(&mut self) {
        if let Some(mut current) = self.todo_select_state.selected() {
            current = min(current + 1, self.todo_items.len() - 1);
            self.todo_select_state.select(Some(current));
        } else {
            self.todo_select_state.select(Some(0));
        };
    }

    pub fn liststate_decrement(&mut self) {
        if let Some(current) = self.todo_select_state.selected() {
            if current > 0 {
                if let Some(new_value) = current.checked_sub(1) {
                    self.todo_select_state.select(Some(new_value));
                } else {
                    // underflow
                    self.todo_select_state.select(Some(0));
                }
            }
        } else {
            self.todo_select_state.select(Some(0));
        };
    }
}
#[cfg(test)]
mod tests {
    use super::*;
}

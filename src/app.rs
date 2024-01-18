use ratatui::widgets::ListState;
use rusqlite::{Connection, Result};
use std::cmp::min;
#[derive(Debug)]
pub struct Task {
    id: u32,
    title: String,
    deadline: String,
}

pub enum CurrentScreen {
    Todolist,
    Input,
}

impl Default for CurrentScreen {
    fn default() -> Self {
        CurrentScreen::Todolist
    }
}

impl Task {
    pub fn get_format_string(&self) -> String {
        format!(
            "{}{}{}",
            self.title,
            " ".repeat(20 - self.title.len()),
            self.deadline
        )
    }
}

#[derive(Default)]
pub struct App {
    pub should_quit: bool,
    pub todo_items: Vec<String>,
    pub todo_select_state: ListState,
    pub current_input: String,
    pub current_screen: CurrentScreen,
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

    pub fn toggle_screen(&mut self) {
        match self.current_screen {
            CurrentScreen::Input => self.current_screen = CurrentScreen::Todolist,
            CurrentScreen::Todolist => self.current_screen = CurrentScreen::Input,
        }
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
            if current > 1 {
                if let Some(new_value) = current.checked_sub(1) {
                    self.todo_select_state.select(Some(new_value));
                } else {
                    // underflow
                    self.todo_select_state.select(Some(1));
                }
            }
        } else {
            self.todo_select_state.select(Some(1));
        };
    }

    pub fn load_task_database(&mut self) -> Result<()> {
        let conn = Connection::open("tasks.db")?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS task (
            id INTEGER PRIMARY KEY,
            title TEXT NOT NULL,
            deadline TEXT)",
            [],
        )?;

        // let task1 = Task {
        //     id: 1,
        //     title: "Urgent Work".to_string(),
        //     deadline: "18-01-2023".to_string(),
        // };
        //
        // conn.execute)?;

        let mut stmt = conn.prepare("SELECT * FROM task")?;
        let task_iter = stmt.query_map([], |row| {
            Ok(Task {
                id: row.get(0)?,
                title: row.get(1)?,
                deadline: row.get(2)?,
            })
        })?;

        for task in task_iter {
            let task = task.unwrap();
            self.todo_items.push(task.get_format_string());
        }

        Ok(())
    }
}
#[cfg(test)]
mod tests {
    use super::*;
}

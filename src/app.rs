use ratatui::widgets::ListState;
use rusqlite::{Connection, Result};
use std::cmp::min;

#[derive(Debug)]
pub struct Task {
    title: String,
    deadline: String,
}

pub enum CurrentScreen {
    Todolist,
    Input,
    Deadline,
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
            CurrentScreen::Deadline => self.current_screen = CurrentScreen::Deadline,
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

    pub fn insert_database(&mut self, conn: &Connection, values: (String, String)) -> Result<()> {
        conn.execute(
            "INSERT INTO TABLE task(title, deadline) values (?1, ?2)",
            values,
        )?;
        Ok(())
    }

    pub fn load_database(&mut self, conn: &Connection) -> Result<()> {
        conn.execute(
            "CREATE TABLE IF NOT EXISTS task (
            title TEXT NOT NULL,
            deadline TEXT)",
            [],
        )?;

        let mut stmt = conn.prepare("SELECT * FROM task")?;
        let task_iter = stmt.query_map([], |row| {
            Ok(Task {
                title: row.get(0)?,
                deadline: row.get(1)?,
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
    use std::fs;
    #[test]
    fn test_load_database() {
        let mut app = App::new();
        let conn = Connection::open("test.db").expect("Database creation failed");
        conn.execute(
            "CREATE TABLE task (
            title TEXT NOT NULL,
            deadline TEXT)",
            [],
        )
        .expect("Initial table creation failed");

        let items: Vec<Task> = vec![
            Task {
                title: "Urgent Work".to_string(),
                deadline: "18-01-2024".to_string(),
            },
            Task {
                title: "Less urgent work".to_string(),
                deadline: "20-01-2024".to_string(),
            },
        ];

        let mut app_tasks: Vec<String> = vec![];

        for item in items {
            app_tasks.push(item.get_format_string());
            conn.execute(
                "INSERT INTO task (title, deadline) VALUES (?1, ?2)",
                (item.title, item.deadline),
            )
            .expect("Insertion failed");
        }

        let _ = app.load_database(&conn);

        assert_eq!(app.todo_items, app_tasks);
        conn.close();
        let _ = fs::remove_file("test.db");
    }

    // #[test]
    // fn test_insert_database() {
    //     let mut app = App::new();
    //     let conn = Connection::open("test.db").expect("Database creation failed");
    //     conn.execute(
    //         "CREATE TABLE task (
    //         title TEXT NOT NULL,
    //         deadline TEXT)",
    //         [],
    //     )
    //     .expect("Initial table creation failed");
    //
    //     let values = ("Big task".to_string(), "04-02-2024".to_string());
    //
    //     app.insert_database(&conn, values);

    // let mut stmt = conn
    //     .prepare("SELECT * FROM task")
    //     .expect("connection failed to prepare");
    // let mut values_from_database = stmt
    //     .query_map([], |row| Ok(row.get(0)?, row.get(1)?))
    //     .expect("Failed to get rows");
    // }
}

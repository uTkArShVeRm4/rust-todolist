use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::app::{App, CurrentScreen};

pub fn update(app: &mut App, key_event: KeyEvent) {
    match app.current_screen {
        CurrentScreen::Todolist => match key_event.code {
            KeyCode::Down => app.liststate_increment(),
            KeyCode::Up => app.liststate_decrement(),
            KeyCode::Tab => app.toggle_screen(),
            KeyCode::Esc | KeyCode::Char('q') => app.quit(),
            KeyCode::Char('c') | KeyCode::Char('C') => {
                if key_event.modifiers == KeyModifiers::CONTROL {
                    app.quit()
                }
            }
            _ => (),
        },
        CurrentScreen::Input => match key_event.code {
            KeyCode::Char(value) => app.current_input.push(value),
            KeyCode::Backspace => {
                app.current_input.pop();
            }
            KeyCode::Tab => app.toggle_screen(),
            KeyCode::Esc => app.quit(),
            _ => (),
        },
        _ => {}
    }
}

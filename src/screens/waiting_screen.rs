use std::time::Duration;

use crossterm::{terminal, event::{poll, read, Event}};

use crate::utils::tui::{ScreenParams, exit};

#[allow(unused_variables)]
pub fn waiting_screen(params: ScreenParams) {

    terminal::enable_raw_mode().unwrap();

    loop {
        if poll(Duration::from_millis(50)).unwrap() {
            if let Ok(event) = read() {
                match event {
                    Event::Key(event) => match event.code {
                        crossterm::event::KeyCode::Char('q') => {
                            break;
                        }
                        _ => {}
                    },
                    _ => {}
                }
            }
        }
    }

    exit();
}

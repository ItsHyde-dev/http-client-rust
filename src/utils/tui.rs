use std::io::stdout;

use crossterm::{
    cursor,
    terminal::{self, ClearType},
    ExecutableCommand,
};

use super::parser::Request;

use crate::screens::{
    request_details_screen::request_details_screen, request_listing_screen::request_listing_screen, waiting_screen::waiting_screen,
};

#[derive(Clone)]
pub struct ScreenParams {
    pub screen: Screens,
    pub requests: Option<Vec<Request>>,
    pub selected_request: Option<Request>,
}

#[derive(Clone)]
pub enum Screens {
    RequestListing,
    RequestDetails,
    Waiting
}

pub fn switch_screens(params: ScreenParams) {

    stdout().execute(cursor::Hide).unwrap();

    clear_screen();
    match params.screen {
        Screens::RequestListing => {
            request_listing_screen(params);
        }
        Screens::RequestDetails => {
            request_details_screen(params);
        }
        Screens::Waiting => {
            waiting_screen(params);
        }
    }
}

pub fn clear_screen() {
    let mut stdout = std::io::stdout();
    stdout.execute(terminal::Clear(ClearType::All)).unwrap();
    stdout.execute(cursor::MoveTo(0, 0)).unwrap();
}

pub fn exit() {
    terminal::disable_raw_mode().unwrap();
    stdout().execute(cursor::Show).unwrap();
    std::process::exit(0);
}

use std::time::Duration;

use crossterm::{
    cursor,
    event::{poll, read, Event},
    terminal, ExecutableCommand,
};

use crate::utils::{
    parser::{CurlRequest, Request},
    tui::{clear_screen, exit, switch_screens, ScreenParams, Screens},
};

use std::io::Write;

pub fn request_listing_screen(params: ScreenParams) {
    terminal::enable_raw_mode().unwrap();
    let mut stdout = std::io::stdout();
    let requests = params.requests.as_ref().unwrap();

    let mut selected_index = 0;

    loop {
        display_request_list(&mut stdout, &requests, selected_index);

        if poll(Duration::from_millis(50)).unwrap() {
            if let Ok(event) = read() {
                match event {
                    Event::Key(event) => match event.code {
                        crossterm::event::KeyCode::Char('j') | crossterm::event::KeyCode::Down => {
                            if selected_index < requests.len() - 1 {
                                selected_index += 1;
                            }
                        }
                        crossterm::event::KeyCode::Char('k') | crossterm::event::KeyCode::Up => {
                            if selected_index > 0 {
                                selected_index -= 1;
                            }
                        }
                        crossterm::event::KeyCode::Char('q') => {
                            break;
                        }
                        crossterm::event::KeyCode::Enter => {
                            let request = requests[selected_index].clone();
                            terminal::disable_raw_mode().unwrap();

                            let mut params = params;

                            params.screen = Screens::RequestDetails;
                            params.selected_request = Some(request);
                            switch_screens(params);

                            break;
                        }
                        _ => {}
                    },
                    _ => {}
                }
            }
        }

        stdout.execute(cursor::MoveTo(0, 0)).unwrap();
    }

    exit();
}

pub fn display_request_list(
    stdout: &mut std::io::Stdout,
    items: &Vec<Request>,
    selected_index: usize,
) {
    clear_screen();

    for (index, request) in items.iter().enumerate() {
        let counter = index + 1;

        stdout.execute(cursor::MoveToColumn(0)).unwrap();
        if index == selected_index {
            println!(" > {counter}. {}", request.name.trim());
        } else {
            println!("   {counter}. {}", request.name.trim());
        }
    }

    println!();
    stdout.execute(cursor::MoveToColumn(0)).unwrap();
    write!(stdout, ">> Press q to exit \r\n").unwrap();
    write!(stdout, ">> Press Enter to select request \r\n").unwrap();
}

pub fn curl_request_listing_screen(params: ScreenParams) {
    terminal::enable_raw_mode().unwrap();
    let mut stdout = std::io::stdout();
    let requests = params.curl_requests.as_ref().unwrap();

    let mut selected_index = 0;

    loop {
        display_curl_request_list(&mut stdout, &requests, selected_index);

        if poll(Duration::from_millis(50)).unwrap() {
            if let Ok(event) = read() {
                match event {
                    Event::Key(event) => match event.code {
                        crossterm::event::KeyCode::Char('j') | crossterm::event::KeyCode::Down => {
                            if selected_index < requests.len() - 1 {
                                selected_index += 1;
                            }
                        }
                        crossterm::event::KeyCode::Char('k') | crossterm::event::KeyCode::Up => {
                            if selected_index > 0 {
                                selected_index -= 1;
                            }
                        }
                        crossterm::event::KeyCode::Char('q') => {
                            break;
                        }
                        crossterm::event::KeyCode::Enter => {
                            let request = requests[selected_index].clone();
                            terminal::disable_raw_mode().unwrap();

                            let mut params = params;

                            params.screen = Screens::CurlRequestDetails;
                            params.selected_curl_request = Some(request);
                            switch_screens(params);

                            break;
                        }
                        _ => {}
                    },
                    _ => {}
                }
            }
        }

        stdout.execute(cursor::MoveTo(0, 0)).unwrap();
    }

    exit();
}

pub fn display_curl_request_list(
    stdout: &mut std::io::Stdout,
    items: &Vec<CurlRequest>,
    selected_index: usize,
) {
    clear_screen();

    for (index, request) in items.iter().enumerate() {
        let counter = index + 1;

        stdout.execute(cursor::MoveToColumn(0)).unwrap();
        if index == selected_index {
            println!(" > {counter}. {}", request.name.trim());
        } else {
            println!("   {counter}. {}", request.name.trim());
        }
    }

    println!();
    stdout.execute(cursor::MoveToColumn(0)).unwrap();
    write!(stdout, ">> Press q to exit \r\n").unwrap();
    write!(stdout, ">> Press Enter to select request \r\n").unwrap();
}

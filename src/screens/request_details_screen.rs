use crossterm::{
    event::{poll, read, Event},
    terminal::{self},
};
use serde_json::Value;
use std::{
    io::{stdout, Write},
    time::Duration,
};

use crate::{utils::{
    parser::{Request, CurlRequest},
    request_sender::{send_request, send_curl_request},
    tui::{clear_screen, exit, switch_screens, ScreenParams, Screens},
}, TOKIO_RUNTIME};

pub fn request_details_screen(params: ScreenParams) {
    terminal::enable_raw_mode().unwrap();
    let request = params.selected_request.as_ref().unwrap();
    loop {
        display_details(&request);

        if poll(Duration::from_millis(50)).unwrap() {
            if let Ok(event) = read() {
                match event {
                    Event::Key(event) => match event.code {
                        crossterm::event::KeyCode::Char('q') => {
                            break;
                        }
                        crossterm::event::KeyCode::Char('b') => {
                            let mut params = params;
                            params.screen = Screens::RequestListing;

                            switch_screens(params);
                            break;
                        }
                        crossterm::event::KeyCode::Char('y') => {
                            let mut params = params.clone();
                            params.screen = Screens::Waiting;
                            TOKIO_RUNTIME.block_on(send_request(&request));
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

fn display_details(request: &Request) {
    clear_screen();

    write!(stdout(), "Name: {} \r\n", request.name).unwrap();
    write!(stdout(), "Method: {} \r\n", request.method).unwrap();
    write!(stdout(), "URL: {} \r\n", request.url).unwrap();

    if request.headers.is_some() {
        let json_value: Value = serde_json::from_str(request.body.as_ref().unwrap()).unwrap();
        let printable_headers = serde_json::to_string_pretty(&json_value)
            .unwrap()
            .replace("\n", "\r\n");
        write!(stdout(), "Headers: \r\n{} \r\n", printable_headers).unwrap();
    }
    if request.body.is_some() {
        let json_value: Value = serde_json::from_str(request.body.as_ref().unwrap()).unwrap();
        let printable_body = serde_json::to_string_pretty(&json_value)
            .unwrap()
            .replace("\n", "\r\n");
        write!(stdout(), "Body: \r\n{} \r\n", printable_body).unwrap();
    }

    println!();

    write!(stdout(), ">> Press q to exit \r\n").unwrap();
    write!(stdout(), ">> Press b to go back to listing \r\n").unwrap();
    write!(stdout(), ">> Press y to execute \r\n").unwrap();
}

pub fn curl_request_details_screen(params: ScreenParams) {
    terminal::enable_raw_mode().unwrap();
    let request = params.selected_curl_request.as_ref().unwrap();
    loop {
        display_details_curl(&request);

        if poll(Duration::from_millis(50)).unwrap() {
            if let Ok(event) = read() {
                match event {
                    Event::Key(event) => match event.code {
                        crossterm::event::KeyCode::Char('q') => {
                            break;
                        }
                        crossterm::event::KeyCode::Char('b') => {
                            let mut params = params;
                            params.screen = Screens::RequestListing;

                            switch_screens(params);
                            break;
                        }
                        crossterm::event::KeyCode::Char('y') => {
                            let mut params = params.clone();
                            params.screen = Screens::Waiting;

                            TOKIO_RUNTIME.block_on(send_curl_request(request.req.clone()));
// let output = async_process::Command::new("bash").arg("-c").arg(request.req.clone()).output().await;
// println!("{}", String::from_utf8_lossy(&output.stdout));
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

fn display_details_curl(request: &CurlRequest) {
    clear_screen();

    write!(stdout(), "Name: {} \r\n", request.name).unwrap();
    write!(stdout(), "Request: {} \r\n", request.req).unwrap();

    println!();

    write!(stdout(), ">> Press q to exit \r\n").unwrap();
    write!(stdout(), ">> Press b to go back to listing \r\n").unwrap();
    write!(stdout(), ">> Press y to execute \r\n").unwrap();
}

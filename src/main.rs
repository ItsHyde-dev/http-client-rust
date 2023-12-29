use std::io;

use clap::Parser;
use clap::ValueEnum;
use crossterm::execute;
use crossterm::terminal::EnterAlternateScreen;
use crossterm::terminal::LeaveAlternateScreen;
use lazy_static::lazy_static;

pub mod utils {
    pub mod brackets;
    pub mod filereader;
    pub mod parser;
    pub mod request_sender;
    pub mod tui;
}

pub mod screens {
    pub mod request_details_screen;
    pub mod request_listing_screen;
    pub mod waiting_screen;
}

use utils::filereader;
use utils::parser;
use utils::tui::*;

#[derive(Clone, Debug, ValueEnum)]
enum Mediums {
    Curl,
    Custom,
}

#[derive(Parser, Debug)]
struct Args {
    // path to file containing the http requests
    #[arg(short, long)]
    path: String,
    #[arg(short, long, value_enum, default_value_t = Mediums::Curl)]
    medium: Mediums,
}

lazy_static! {
    static ref TOKIO_RUNTIME: tokio::runtime::Runtime = tokio::runtime::Runtime::new().unwrap();
}

fn main() {
    let args = Args::parse();
    let requests = filereader::read_file(args.path);
    execute!(io::stdout(), EnterAlternateScreen).unwrap();
    match args.medium {
        Mediums::Curl => {
            let parsed_requests = parser::parse_curl_requests(requests);

            dbg!(&parsed_requests);

            let params = ScreenParams {
                screen: Screens::CurlRequestListing,
                requests: None,
                selected_request: None,
                curl_requests: Some(parsed_requests),
                selected_curl_request: None,
            };

            switch_screens(params);
        }
        Mediums::Custom => {
            let parsed_requests = parser::parse_requests(requests);
            let params = ScreenParams {
                screen: Screens::RequestListing,
                requests: Some(parsed_requests),
                selected_request: None,
                curl_requests: None,
                selected_curl_request: None,
            };

            switch_screens(params);
        }
    };
    execute!(io::stdout(), LeaveAlternateScreen).unwrap();
}

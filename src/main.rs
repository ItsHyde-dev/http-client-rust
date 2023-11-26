use clap::Parser;
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

#[derive(Parser, Debug)]
struct Args {
    // path to file containing the http requests
    #[arg(short, long)]
    path: String,
}

lazy_static! {
    static ref TOKIO_RUNTIME: tokio::runtime::Runtime = tokio::runtime::Runtime::new().unwrap();
}

fn main() {
    let args = Args::parse();
    let requests = filereader::read_file(args.path);
    let parsed_requests = parser::parse_requests(requests);

    let params = ScreenParams {
        screen: Screens::RequestListing,
        requests: Some(parsed_requests),
        selected_request: None,
    };

    switch_screens(params);
}

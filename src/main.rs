use clap::Parser;

pub mod utils {
    pub mod brackets;
    pub mod filereader;
    pub mod parser;
}

use utils::filereader;
use utils::parser;

#[derive(Parser, Debug)]
struct Args {
    // path to file containing the http requests
    #[arg(short, long)]
    path: String,
}

fn main() {
    let args = Args::parse();
    let requests = filereader::read_file(args.path);
    parser::parse_requests(requests);
}

use clap::Parser;
use serde::{Deserialize, Serialize};

#[derive(Parser, Debug)]
struct Args {
    // path to file containing the http requests
    #[arg(short, long)]
    path: String,
}

fn main() {
    let args = Args::parse();
    let requests = read_file(args.path);
    parse_requests(requests);
}

fn read_file(path: String) -> Vec<String> {
    let contents: String = std::fs::read_to_string(path).unwrap();

    let mut requests: Vec<String> = Vec::new();
    let mut block: String = "".to_string();
    let mut flag = false;

    for line in contents.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        if line.starts_with("####") {
            match flag {
                true => {
                    requests.push(block.to_string());
                    block = "".to_string();
                    flag = false;
                    continue;
                }
                false => {
                    flag = true;
                }
            }
        } else {
            block = format!("{}\n{}", block, line);
        }
    }

    if block.len() > 0 {
        requests.push(block);
    }

    return requests;
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Request {
    pub name: String,
    pub url: String,
    pub method: String,
    pub headers: Option<String>,
    pub body: Option<String>,
}

fn parse_requests(requests: Vec<String>) {
    for request in requests {
        let (keys, values) = get_key_value_pairs(request);

        for i in 0..keys.len() {
            let key = keys[i].to_string();
            let value = values[i].to_string();

            if is_key_for_json_data(key.clone()) {
                let json: serde_json::Value = match serde_json::from_str(&value) {
                    Ok(json) => json,
                    Err(_) => {
                        println!("Please format json values properly for key: {}", key);
                        panic!();
                    }
                };

                println!("{}", json);
            } else {
                println!("{}: {}", key, value);
            }
        }
    }
}

fn get_key_value_pairs(data: String) -> (Vec<String>, Vec<String>) {
    let iter = data.lines();
    let mut reading_json_data = false;

    let mut value = "".to_string();
    let mut brackets: Brackets = Brackets::new();

    let mut keys: Vec<String> = Vec::new();
    let mut values: Vec<String> = Vec::new();

    for line in iter {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        if reading_json_data {
            value = format!("{}\n{}", value, line);
            match line {
                "{" => {
                    brackets.open();
                }
                "}" => {
                    brackets.close();

                    if brackets.is_valid() {
                        reading_json_data = false;
                    }

                    values.push(value.trim().to_string());
                    value = "".to_string();
                }
                _ => {}
            }
            continue;
        }

        if starts_with_valid_key(line.to_string()) {
            let key = get_key(line.to_string());
            value = format!("{}\n{}", value, get_value(line.to_string()));

            keys.push(key.to_string());

            if is_key_for_json_data(key.to_string()) {
                reading_json_data = true;
            } else {
                values.push(value.trim().to_string());
                value = "".to_string();
            }

            continue;
        }
    }

    return (keys, values);
}

fn get_value(line: String) -> String {
    return line.split(":").skip(1).collect::<String>();
}

fn get_key(line: String) -> String {
    return line.split(":").next().unwrap().to_string();
}

fn is_key_for_json_data(key: String) -> bool {
    let json_keys = ["headers", "body"];

    for i in json_keys {
        if key == i {
            return true;
        }
    }

    return false;
}

pub struct Brackets {
    open: i32,
    close: i32,
}

impl Brackets {
    fn new() -> Self {
        Brackets { open: 0, close: 0 }
    }

    fn open(&mut self) {
        self.open += 1;
    }

    fn close(&mut self) {
        self.close += 1;
    }

    fn is_valid(&self) -> bool {
        return self.open == self.close;
    }
}

fn starts_with_valid_key(data: String) -> bool {
    let valid_keys = ["name", "url", "method", "headers", "body"];
    for i in valid_keys {
        if data.starts_with(i) {
            return true;
        }
    }

    return false;
}

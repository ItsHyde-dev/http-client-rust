use std::io::stdout;

use super::parser;
use crossterm::{
    terminal::{self, Clear, ClearType},
    ExecutableCommand,
};
use serde_json::Value;

use reqwest::Client;

pub async fn send_request(request_data: &parser::Request) {
    terminal::disable_raw_mode().unwrap();
    stdout().execute(Clear(ClearType::All)).unwrap();

    let client = Client::new();

    let url = reqwest::Url::parse(request_data.url.as_ref()).unwrap();
    let mut request_builder = client.request(get_method(request_data.method.as_str()), url);

    if request_data.headers.is_some() {
        let headers_json: Value =
            serde_json::from_str(request_data.headers.as_ref().unwrap()).unwrap();
        for (key, value) in headers_json.as_object().unwrap() {
            request_builder = request_builder.header(key, value.as_str().unwrap());
        }
    }

    request_builder = request_builder.body(request_data.body.as_ref().unwrap().clone());
    let request = request_builder.build();

    println!("Request URL: {}", request_data.url);

    match request {
        Ok(request) => {
            let response = client.execute(request).await;
            let response = match response {
                Ok(response) => response,
                Err(e) => {
                    println!("{}", e);
                    return;
                }
            };

            if response.status().is_success() {
                println!("Request succeeded with status: {:?}", response.status());
            } else {
                println!("Request failed with status: {:?}", response.status());
            }
            let response_bytes = response.bytes().await;
            let json: Result<Value, serde_json::Error> =
                serde_json::from_slice(&response_bytes.as_ref().unwrap());

            match json {
                Ok(json) => {
                    println!(
                        "Response Body: {}",
                        serde_json::to_string_pretty(&json).unwrap()
                    );
                }
                Err(_) => {
                    println!(
                        "Response Body: {}",
                        String::from_utf8_lossy(&response_bytes.unwrap())
                    );
                }
            }
        }
        Err(e) => {
            println!("{}", e);
        }
    }
}

pub fn get_method(method_string: &str) -> reqwest::Method {
    return match method_string {
        "GET" => reqwest::Method::GET,
        "POST" => reqwest::Method::POST,
        "PUT" => reqwest::Method::PUT,
        "DELETE" => reqwest::Method::DELETE,
        _ => reqwest::Method::GET,
    };
}

pub async fn send_curl_request(request_string: String) {
    terminal::disable_raw_mode().unwrap();
    stdout().execute(Clear(ClearType::All)).unwrap();

    let output = async_process::Command::new("bash")
        .arg("-c")
        .arg(format!("{request_string} --no-progress-meter"))
        .output()
        .await;

    let output = output.unwrap();

    let stdout_string = String::from_utf8_lossy(&output.stdout);
    let stderr_string = String::from_utf8_lossy(&output.stderr);

    if stderr_string.len() > 0 {
        println!("{}", stderr_string);
    } else {
        println!("Response: {}", stdout_string);
    }
}

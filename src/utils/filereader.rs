use regex::Regex;

pub fn read_file(path: String) -> Vec<String> {
    let contents: String = std::fs::read_to_string(path).unwrap();
    let pattern = Regex::new(r"#{3,}").unwrap();
    let requests: Vec<String> = pattern
        .split(contents.as_str())
        .filter_map(|s| {
            if s.trim().is_empty() {
                None
            } else {
                Some(s.to_string())
            }
        })
        .collect();

    println!("{:?}", requests);

    // let mut requests: Vec<String> = Vec::new();
    // let mut block: String = "".to_string();
    // let mut flag = false;

    // for line in contents.lines() {
    // let line = line.trim();
    // if line.is_empty() {
    // continue;
    // }

    // if line.starts_with("####") {
    // match flag {
    // true => {
    // requests.push(block.to_string());
    // block = "".to_string();
    // flag = false;
    // continue;
    // }
    // false => {
    // flag = true;
    // }
    // }
    // } else {
    // block = format!("{}\n{}", block, line);
    // }
    // }

    // if block.len() > 0 {
    // requests.push(block);
    // }

    return requests;
}

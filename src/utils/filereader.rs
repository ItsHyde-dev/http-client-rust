pub fn read_file(path: String) -> Vec<String> {
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

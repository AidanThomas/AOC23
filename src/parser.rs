use std::fs;

pub fn read_by_line(path: &str) -> Result<Vec<String>, String> {
    let mut result = Vec::new();

    let contents = match fs::read_to_string(path) {
        Ok(x) => x,
        Err(e) => return Err(e.to_string()),
    };

    for line in contents.lines() {
        result.push(line.to_string());
    }

    Ok(result)
}

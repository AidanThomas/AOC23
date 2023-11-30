use std::fs;

pub fn read_by_line(path: &str) -> Result<Vec<String>, String> {
    let contents = match fs::read_to_string(path) {
        Ok(x) => x,
        Err(e) => return Err(e.to_string()),
    };

    Ok(contents.lines().map(|x| x.to_string()).collect())
}

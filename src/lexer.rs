use std::str::FromStr;

pub fn lexer(input: &str) -> Result<Vec<Vec<String>>, String> {
    let mut tokens = Vec::new();

    for (line_num, line) in input.lines().enumerate() {
        let trimmed = line.trim();
        
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue; // Skip empty lines and comments
        }

        let parts: Vec<String> = trimmed.split_whitespace().map(String::from).collect();
        if parts.is_empty() {
            return Err(format!("Error on line {}: Empty or invalid line", line_num + 1));
        }

        tokens.push(parts);
    }

    Ok(tokens)
}

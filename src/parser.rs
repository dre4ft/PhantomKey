use crate::token::Token;
use std::str::FromStr; 

/*impl FromStr for Token {
    type Err = String;

    fn from_str(command: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = command.split_whitespace().collect();
        match parts[0].to_uppercase().as_str() {
            "STRING" => {
                if parts.len() < 2 {
                    return Err("STRING requires a text argument.".to_string());
                }
                let text = parts[1..].join(" ");
                Ok(Token::String(text))
            }
            "DELAY" => {
                if parts.len() != 2 || !parts[1].parse::<u32>().is_ok() {
                    return Err("DELAY requires a single numeric argument.".to_string());
                }
                Ok(Token::Delay(parts[1].parse::<u32>().unwrap()))
            }
            "MOVE_MOUSE" => {
                if parts.len() != 3 || !parts[1].parse::<i32>().is_ok() || !parts[2].parse::<i32>().is_ok() {
                    return Err("MOVE_MOUSE requires two numeric arguments.".to_string());
                }
                Ok(Token::MoveMouse(
                    parts[1].parse::<i32>().unwrap(),
                    parts[2].parse::<i32>().unwrap(),
                ))
            }
            "CLICK" => {
                if parts.len() != 2 || !["LEFT", "RIGHT", "MIDDLE"].contains(&parts[1].to_uppercase().as_str()) {
                    return Err("CLICK requires one of LEFT, RIGHT, or MIDDLE.".to_string());
                }
                Ok(Token::Click(parts[1].to_uppercase()))
            }
            "ENTER" => Ok(Token::Enter),
            "BACKSPACE" => Ok(Token::Backspace),
            "TAB" => Ok(Token::Tab),
            "ESCAPE" => Ok(Token::Escape),
            "ARROW" => {
                if parts.len() != 2 || !["UP", "DOWN", "LEFT", "RIGHT"].contains(&parts[1].to_uppercase().as_str()) {
                    return Err("ARROW requires one of UP, DOWN, LEFT, or RIGHT.".to_string());
                }
                Ok(Token::Arrow(parts[1].to_uppercase()))
            }
            "HOLD" => {
                if parts.len() != 2 {
                    return Err("HOLD requires a key argument.".to_string());
                }
                Ok(Token::Hold(parts[1].to_uppercase()))
            }
            "RELEASE" => {
                if parts.len() != 2 {
                    return Err("RELEASE requires a key argument.".to_string());
                }
                Ok(Token::Release(parts[1].to_uppercase()))
            }
            "TYPE" => {
                if parts.len() < 2 {
                    return Err("TYPE requires a key sequence.".to_string());
                }
                Ok(Token::Type(parts[1..].join(" ")))
            }
            "SCROLL" => {
                if parts.len() != 2 || !parts[1].parse::<i32>().is_ok() {
                    return Err("SCROLL requires a numeric argument.".to_string());
                }
                Ok(Token::Scroll(parts[1].parse::<i32>().unwrap()))
            }
            _ => Err(format!("Unknown command: {}", parts[0])),
        }
    }
}
*/
pub fn parse(tokens: Vec<Vec<String>>) -> Result<Vec<Token>, String> {
    let mut parsed_tokens = Vec::new();

    for token_line in tokens {
        if token_line.is_empty() {
            continue; // Ignore empty lines
        }

        let command = &token_line[0].to_uppercase();
        let args = &token_line[1..];

        let token = match command.as_str() {
            // Handle STRING command
            "STRING" => {
                if args.is_empty() {
                    return Err("STRING command requires a text argument.".into());
                }
                Token::String(args.join(" ")) // Combine all args into one string
            }

            // Handle DELAY command
            "DELAY" => {
                if args.len() != 1 {
                    return Err("DELAY command requires exactly one numerical argument.".into());
                }
                let delay_ms = args[0].parse::<u64>().map_err(|_| "Invalid number for DELAY")?;
                Token::Delay(delay_ms)
            }

            // Handle MOVE_MOUSE command
            "MOVE_MOUSE" => {
                if args.len() != 2 {
                    return Err("MOVE_MOUSE requires exactly two arguments (x, y).".into());
                }
                let x = args[0].parse::<i32>().map_err(|_| "Invalid x coordinate")?;
                let y = args[1].parse::<i32>().map_err(|_| "Invalid y coordinate")?;
                Token::MoveMouse(x, y)
            }

            // Handle CLICK command
            "CLICK" => {
                if args.len() != 1 {
                    return Err("CLICK command requires exactly one argument (LEFT, RIGHT, or MIDDLE).".into());
                }
                let button = args[0].to_uppercase();
                if !["LEFT", "RIGHT", "MIDDLE"].contains(&button.as_str()) {
                    return Err("Invalid button for CLICK. Use LEFT, RIGHT, or MIDDLE.".into());
                }
                Token::Click(button)
            }

            // Handle BACKSPACE, TAB, ESCAPE, ENTER (No Arguments)
            "BACKSPACE" => Token::Backspace,
            "TAB" => Token::Tab,
            "ESCAPE" => Token::Escape,
            "ENTER" => Token::Enter,

            // Handle ARROW command
            "ARROW" => {
                if args.len() != 1 {
                    return Err("ARROW command requires exactly one argument (UP, DOWN, LEFT, or RIGHT).".into());
                }
                let direction = args[0].to_uppercase();
                if !["UP", "DOWN", "LEFT", "RIGHT"].contains(&direction.as_str()) {
                    return Err("Invalid direction for ARROW. Use UP, DOWN, LEFT, or RIGHT.".into());
                }
                Token::Arrow(direction)
            }

            // Handle SCROLL command
            "SCROLL" => {
                if args.len() != 1 {
                    return Err("SCROLL command requires exactly one numerical argument.".into());
                }
                let amount = args[0].parse::<i32>().map_err(|_| "Invalid number for SCROLL")?;
                Token::Scroll(amount)
            }

            // Handle TYPE command (alias for STRING)
            "TYPE" => {
                if args.is_empty() {
                    return Err("TYPE command requires a text argument.".into());
                }
                Token::Type(args.join(" "))
            }

            // Handle HOLD and RELEASE
            "HOLD" | "RELEASE" => {
                if args.len() != 1 {
                    return Err(format!("{} command requires exactly one argument (e.g., SHIFT, CTRL).", command));
                }
                let key = args[0].to_uppercase();
                if !["SHIFT", "CTRL", "ALT"].contains(&key.as_str()) {
                    return Err("Invalid key for HOLD/RELEASE. Use SHIFT, CTRL, or ALT.".into());
                }
                if command == "HOLD" {
                    Token::Hold(key)
                } else {
                    Token::Release(key)
                }
            }

            _ => return Err(format!("Unknown command: {}", command)),
        };

        parsed_tokens.push(token);
    }

    Ok(parsed_tokens)
}

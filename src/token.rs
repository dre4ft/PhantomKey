use std::str::FromStr;

#[derive(Debug)]
pub enum Token {
    Delay(u64),
    String(String),
    MoveMouse(i32, i32),
    Click(String),
    Enter,
    Hold(String),
    Release(String),
    Type(String),
    Scroll(i32),
    Backspace,
    Tab,
    Escape,
    Arrow(String),
}

impl FromStr for Token {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = input.split_whitespace().collect();
        match parts.as_slice() {
            ["DELAY", time] => Ok(Token::Delay(time.parse().map_err(|_| "Invalid delay")?)),
            ["STRING", text] => Ok(Token::String(text.to_string())),
            ["MOVE_MOUSE", x, y] => Ok(Token::MoveMouse(
                x.parse().map_err(|_| "Invalid x")?,
                y.parse().map_err(|_| "Invalid y")?,
            )),
            ["CLICK", button] => Ok(Token::Click(button.to_string())),
            ["ENTER"] => Ok(Token::Enter),
            ["HOLD", key] => Ok(Token::Hold(key.to_string())),
            ["RELEASE", key] => Ok(Token::Release(key.to_string())),
            ["TYPE", text] => Ok(Token::Type(text.to_string())),
            ["SCROLL", amount] => Ok(Token::Scroll(amount.parse().map_err(|_| "Invalid scroll")?)),
            ["BACKSPACE"] => Ok(Token::Backspace),
            ["TAB"] => Ok(Token::Tab),
            ["ESCAPE"] => Ok(Token::Escape),
            ["ARROW", direction] => Ok(Token::Arrow(direction.to_string())),
            _ => Err(format!("Unknown command: {}", input)),
        }
    }
}

impl Token {
    pub fn to_hid_report(&self) -> Vec<u8> {
        match self {
            // Delay: Represented as a NOP with a delay
            Token::Delay(ms) => vec![0x00, (*ms & 0xFF) as u8], // Example delay implementation
            
            // Single Key Presses
            Token::Enter => vec![0x00, 0x28], // HID scan code for ENTER
            Token::Backspace => vec![0x00, 0x2A], // HID scan code for BACKSPACE
            Token::Tab => vec![0x00, 0x2B], // HID scan code for TAB
            Token::Escape => vec![0x00, 0x29], // HID scan code for ESCAPE

            // Arrow Keys
            Token::Arrow(direction) => match direction.as_str() {
                "UP" => vec![0x00, 0x52],
                "DOWN" => vec![0x00, 0x51],
                "LEFT" => vec![0x00, 0x50],
                "RIGHT" => vec![0x00, 0x4F],
                _ => vec![0x00, 0x00], // Default to NOP for unrecognized
            },

            // String Input
            Token::String(text) | Token::Type(text) => {
                text.chars()
                    .flat_map(|c| match c {
                        // Lowercase a-z
                        'a'..='z' => vec![0x00, 0x04 + (c as u8 - b'a')],
            
                        // Uppercase A-Z (with SHIFT)
                        'A'..='Z' => vec![0x02, 0x04 + (c as u8 - b'A')],
            
                        // Numbers 0-9
                        '0'..='9' => match c {
                            '1' => vec![0x02, 0x1E], // Shift + 1 = !
                            '2' => vec![0x02, 0x1F], // Shift + 2 = @
                            '3' => vec![0x02, 0x20], // Shift + 3 = #
                            '4' => vec![0x02, 0x21], // Shift + 4 = $
                            '5' => vec![0x02, 0x22], // Shift + 5 = %
                            '6' => vec![0x02, 0x23], // Shift + 6 = ^
                            '7' => vec![0x02, 0x24], // Shift + 7 = &
                            '8' => vec![0x02, 0x25], // Shift + 8 = *
                            '9' => vec![0x02, 0x26], // Shift + 9 = (
                            '0' => vec![0x02, 0x27], // Shift + 0 = )
                            _ => vec![0x00, 0x27 + (c as u8 - b'0')], // Numbers without modifiers
                        },
            
                        // Space
                        ' ' => vec![0x00, 0x2C],
            
                        // Special characters
                        '!' => vec![0x02, 0x1E], // Shift + 1
                        '@' => vec![0x02, 0x1F], // Shift + 2
                        '#' => vec![0x02, 0x20], // Shift + 3
                        '$' => vec![0x02, 0x21], // Shift + 4
                        '%' => vec![0x02, 0x22], // Shift + 5
                        '^' => vec![0x02, 0x23], // Shift + 6
                        '&' => vec![0x02, 0x24], // Shift + 7
                        '*' => vec![0x02, 0x25], // Shift + 8
                        '(' => vec![0x02, 0x26], // Shift + 9
                        ')' => vec![0x02, 0x27], // Shift + 0
                        '-' => vec![0x00, 0x2D],
                        '_' => vec![0x02, 0x2D], // Shift + -
                        '=' => vec![0x00, 0x2E],
                        '+' => vec![0x02, 0x2E], // Shift + =
                        '[' => vec![0x00, 0x2F],
                        '{' => vec![0x02, 0x2F], // Shift + [
                        ']' => vec![0x00, 0x30],
                        '}' => vec![0x02, 0x30], // Shift + ]
                        '\\' => vec![0x00, 0x31],
                        '|' => vec![0x02, 0x31], // Shift + \
                        ';' => vec![0x00, 0x33],
                        ':' => vec![0x02, 0x33], // Shift + ;
                        '\'' => vec![0x00, 0x34],
                        '"' => vec![0x02, 0x34], // Shift + '
                        ',' => vec![0x00, 0x36],
                        '<' => vec![0x02, 0x36], // Shift + ,
                        '.' => vec![0x00, 0x37],
                        '>' => vec![0x02, 0x37], // Shift + .
                        '/' => vec![0x00, 0x38],
                        '?' => vec![0x02, 0x38], // Shift + /
            
                        // Unsupported character fallback (logging for debugging)
                        _ => {
                            eprintln!("Warning: Unsupported character '{}'", c);
                            vec![0x00, 0x00]
                        }
                    })
                    .collect()
            }
            

            // Mouse Movements
            Token::MoveMouse(x, y) => vec![0x00, *x as u8, *y as u8], // Mouse report format
            Token::Scroll(amount) => vec![0x00, 0x00, *amount as u8], // Vertical scroll

            // Mouse Clicks
            Token::Click(button) => match button.as_str() {
                "LEFT" => vec![0x01, 0x00], // Mouse button 1 (Left click)
                "RIGHT" => vec![0x02, 0x00], // Mouse button 2 (Right click)
                "MIDDLE" => vec![0x04, 0x00], // Mouse button 3 (Middle click)
                _ => vec![0x00, 0x00], // Default to NOP
            },

            // Key Holding
            Token::Hold(key) | Token::Release(key) => match key.as_str() {
                 "SHIFT" => vec![0x02, 0x00], // SHIFT modifier
                "CTRL" => vec![0x01, 0x00],  // CTRL modifier
                 "ALT" => vec![0x04, 0x00],   // ALT modifier
                 _ => {
                     eprintln!("Warning: Unsupported key '{}'", key);
                     vec![0x00, 0x00]         // Default to NOP
                    },
            },
        }
    }
}

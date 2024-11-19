use std::env;
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};
use std::path::Path;

mod lexer;
mod token;
mod parser;
mod compiler;
use lexer::*;
use parser::*;
use compiler::*;



fn main() -> io::Result<()> {
    // 1. Parse command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: phantomkey-compiler <input_script.pk> <output_script.bin>");
        return Ok(());
    }
    let input_path = &args[1];
    let output_path = &args[2];

    // 2. Read the .pk script file
    let mut script_content = String::new();
    let mut file = File::open(input_path)?;
    file.read_to_string(&mut script_content)?;

    // 3. Tokenize the script
    let tokens = match lexer(&script_content) {
        Ok(tokens) => tokens,
        Err(e) => {
            eprintln!("Lexer error: {}", e);
            return Ok(());
        }
    };

    // 4. Parse the tokens
    let parsed_tokens = match parse(tokens) {
        Ok(parsed) => parsed,
        Err(e) => {
            eprintln!("Parser error: {}", e);
            return Ok(());
        }
    };

    // 5. Compile the parsed tokens into binary
    let binary_output = compile(parsed_tokens);

    // 6. Write the compiled binary to the .bin file
    let mut output_file = OpenOptions::new()
        .create(true)
        .write(true)
        .open(output_path)?;

    output_file.write_all(&binary_output)?;

    println!("Successfully compiled to: {}", output_path);
    Ok(())
}

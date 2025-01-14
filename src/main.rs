use std::fs::File;
use std::io::{self, Read};
use std::env;

mod lexer;
mod parser;
mod interpreter;

use lexer::{Lexer, Token};
use parser::{Parser, ASTNode};
use interpreter::Interpreter;

fn main() {
    // Get the file name from command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <filename.mofo>", args[0]);
        std::process::exit(1);
    }

    let filename = &args[1];

    // Check if the file has the .mofo extension
    if !filename.ends_with(".mofo") {
        eprintln!("Error: File must have a .mofo extension.");
        std::process::exit(1);
    }

    // Read the file content
    match read_file_to_string(filename) {
        Ok(content) => {
            let mut lexer = Lexer::new(content);
            let mut tokens = Vec::new();

            while let Some(token) = lexer.next_token() {
                if token == Token::EOF {
                    break;
                }
                tokens.push(token);
            }

            let mut parser = Parser::new(tokens);
            let mut interpreter = Interpreter::new();

            loop {
                match parser.parse() {
                    Ok(ast) => {
                        if let Err(err) = interpreter.execute(ast) {
                            eprintln!("Execution Error: {}", err);
                            break;
                        }
                    }
                    Err(err) => {
                        eprintln!("Parsing Error: {}", err);
                        break;
                    }
                }

                if parser.current_token().is_none() {
                    break;
                }
            }

            println!("Execution completed.");
        }
        Err(e) => eprintln!("Error reading file: {}", e),
    }
}

fn read_file_to_string(filename: &str) -> io::Result<String> {
    let mut file = File::open(filename)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

use std::process::Command;

#[test]
fn test_file_reading() {
    let output = Command::new("./target/debug/mofo-language")
        .arg("examples/hello.mofo")
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("Invalid UTF-8");
    assert!(stdout.contains("Hello, World!"));
}

use mofo_language::lexer::{Lexer, Token};

#[test]
fn test_lexer() {
    let input = String::from("printf(\"Hello, Lexer!\")");
    let mut lexer = Lexer::new(input);

    assert_eq!(lexer.next_token(), Some(Token::KeywordPrintf));
    assert_eq!(lexer.next_token(), Some(Token::ParenOpen));
    assert_eq!(lexer.next_token(), Some(Token::StringLiteral("Hello, Lexer!".to_string())));
    assert_eq!(lexer.next_token(), Some(Token::ParenClose));
    assert_eq!(lexer.next_token(), Some(Token::EOF));
}


use mofo_language::lexer::{Lexer, Token};
use mofo_language::parser::{Parser, ASTNode};

#[test]
fn test_parser() {
    let input = String::from("printf(\"Hello, Parser!\")");
    let mut lexer = Lexer::new(input);

    let tokens: Vec<Token> = lexer
        .into_iter()
        .filter(|token| *token != Token::EOF)
        .collect();

    let mut parser = Parser::new(tokens);

    assert_eq!(
        parser.parse(),
        Ok(ASTNode::PrintStatement("Hello, Parser!".to_string()))
    );
}

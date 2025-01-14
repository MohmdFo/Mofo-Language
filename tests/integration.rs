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

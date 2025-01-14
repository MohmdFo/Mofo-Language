use mofo_language::lexer::{Lexer, Token};
use mofo_language::parser::{Parser, ASTNode, Expression};
use mofo_language::interpreter::Interpreter;


#[test]
fn test_lexer_with_underscored_variables() {
    let input = "_my_var = \"Hello, Variable!\"";
    let mut lexer = Lexer::new(input.to_string());

    assert_eq!(lexer.next_token(), Some(Token::Identifier("_my_var".to_string())));
    assert_eq!(lexer.next_token(), Some(Token::Assign));
    assert_eq!(lexer.next_token(), Some(Token::StringLiteral("Hello, Variable!".to_string())));
    assert_eq!(lexer.next_token(), Some(Token::EOF));
}

#[test]
fn test_interpreter_with_underscored_variables() {
    let tokens = vec![
        Token::Identifier("_my_var".to_string()),
        Token::Assign,
        Token::StringLiteral("Hello, Variable!".to_string()),
        Token::KeywordPrintf,
        Token::ParenOpen,
        Token::Identifier("_my_var".to_string()),
        Token::ParenClose,
        Token::EOF,
    ];
    let mut parser = Parser::new(tokens);
    let mut interpreter = Interpreter::new();

    while let Ok(ast) = parser.parse() {
        if let Err(err) = interpreter.execute(ast) {
            panic!("Interpreter error: {}", err);
        }

        if parser.current_token().is_none() {
            break;
        }
    }

    // The output is printed to the console. Use a testing framework or mock if needed.
}

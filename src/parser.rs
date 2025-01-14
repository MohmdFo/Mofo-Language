use crate::lexer::Token;

#[derive(Debug)]
pub enum ASTNode {
    PrintStatement(String), // A printf statement with a string argument
}

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, position: 0 }
    }

    pub fn parse(&mut self) -> Result<ASTNode, String> {
        match self.current_token() {
            Some(Token::KeywordPrintf) => self.parse_print_statement(),
            _ => Err("Unexpected token at the start of the statement.".to_string()),
        }
    }

    fn parse_print_statement(&mut self) -> Result<ASTNode, String> {
        self.consume_token(); // Consume 'printf'

        if self.current_token() != Some(&Token::ParenOpen) {
            return Err("Expected '(' after 'printf'.".to_string());
        }
        self.consume_token(); // Consume '('

        if let Some(Token::StringLiteral(value)) = self.current_token() {
            let string_value = value.clone();
            self.consume_token(); // Consume the string literal

            if self.current_token() != Some(&Token::ParenClose) {
                return Err("Expected ')' after string literal.".to_string());
            }
            self.consume_token(); // Consume ')'

            Ok(ASTNode::PrintStatement(string_value))
        } else {
            Err("Expected a string literal as the argument to 'printf'.".to_string())
        }
    }

    fn current_token(&self) -> Option<&Token> {
        self.tokens.get(self.position)
    }

    fn consume_token(&mut self) {
        self.position += 1;
    }
}

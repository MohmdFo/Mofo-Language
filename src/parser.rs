use crate::lexer::Token;

#[derive(Debug)]
pub enum ASTNode {
    PrintStatement(Box<Expression>), // A printf statement with an expression
    Assignment(String, String, String), // Variable name, type, and value
}

#[derive(Debug)]
pub enum Expression {
    Variable(String),       // Reference to a variable
    StringLiteral(String),  // Literal string
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
            Some(Token::Identifier(_)) => self.parse_assignment(),
            _ => Err("Unexpected token at the start of the statement.".to_string()),
        }
    }

    pub fn current_token(&self) -> Option<&Token> {
        self.tokens.get(self.position)
    }

    fn parse_print_statement(&mut self) -> Result<ASTNode, String> {
        self.consume_token(); // Consume 'printf'

        if self.current_token() != Some(&Token::ParenOpen) {
            return Err("Expected '(' after 'printf'.".to_string());
        }
        self.consume_token(); // Consume '('

        let expression = match self.current_token() {
            Some(Token::StringLiteral(value)) => {
                let string_value = value.clone();
                self.consume_token(); // Consume the string literal
                Expression::StringLiteral(string_value)
            }
            Some(Token::Identifier(name)) => {
                let var_name = name.clone();
                self.consume_token(); // Consume the identifier
                Expression::Variable(var_name)
            }
            _ => return Err("Expected a string literal or variable name.".to_string()),
        };

        if self.current_token() != Some(&Token::ParenClose) {
            return Err("Expected ')' after printf argument.".to_string());
        }
        self.consume_token(); // Consume ')'

        Ok(ASTNode::PrintStatement(Box::new(expression)))
    }

    fn parse_assignment(&mut self) -> Result<ASTNode, String> {
        // Ensure the current token is an identifier
        if let Some(Token::Identifier(name)) = self.current_token() {
            let variable_name = name.clone();
            self.consume_token(); // Consume the identifier

            // Ensure the next token is a colon ':'
            if self.current_token() != Some(&Token::Colon) {
                return Err(format!("Expected ':' after variable name '{}'.", variable_name));
            }
            self.consume_token(); // Consume ':'

            // Ensure the next token is a type
            if let Some(Token::Type(var_type)) = self.current_token() {
                let variable_type = var_type.clone();
                self.consume_token(); // Consume the type

                // Ensure the next token is an assignment operator '='
                if self.current_token() != Some(&Token::Assign) {
                    return Err(format!(
                        "Expected '=' after variable type '{}' for '{}'.",
                        variable_type, variable_name
                    ));
                }
                self.consume_token(); // Consume '='

                // Ensure the next token is a string literal
                if let Some(Token::StringLiteral(value)) = self.current_token() {
                    let variable_value = value.clone();
                    self.consume_token(); // Consume the string literal
                    return Ok(ASTNode::Assignment(
                        variable_name,
                        variable_type,
                        variable_value,
                    ));
                } else {
                    return Err(format!(
                        "Expected a string literal as the value for variable '{}'.",
                        variable_name
                    ));
                }
            } else {
                return Err("Expected a type after ':'.".to_string());
            }
        } else {
            Err("Expected a variable name.".to_string())
        }
    }

    fn consume_token(&mut self) {
        self.position += 1;
    }
}

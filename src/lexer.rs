#[derive(Debug, PartialEq)]
pub enum Token {
    KeywordPrintf,
    Identifier(String),  // Variable names
    StringLiteral(String),
    ParenOpen,
    ParenClose,
    Assign,              // '='
    EOF,                 // End of file
}

pub struct Lexer {
    input: String,
    position: usize,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Self { input, position: 0 }
    }

    pub fn next_token(&mut self) -> Option<Token> {
        self.skip_whitespace();

        if self.position >= self.input.len() {
            return Some(Token::EOF);
        }

        let current_char = self.current_char();

        match current_char {
            '(' => {
                self.position += 1;
                Some(Token::ParenOpen)
            }
            ')' => {
                self.position += 1;
                Some(Token::ParenClose)
            }
            '=' => {
                self.position += 1;
                Some(Token::Assign)
            }
            '"' => self.read_string(),
            'a'..='z' | 'A'..='Z' | '_' => self.read_identifier(),
            _ => None,
        }
    }

    fn skip_whitespace(&mut self) {
        while self.position < self.input.len()
            && self.current_char().is_whitespace()
        {
            self.position += 1;
        }
    }

    fn current_char(&self) -> char {
        self.input.chars().nth(self.position).unwrap()
    }

    fn read_string(&mut self) -> Option<Token> {
        self.position += 1; // Skip the opening quote
        let start = self.position;

        while self.position < self.input.len()
            && self.current_char() != '"'
        {
            self.position += 1;
        }

        if self.position >= self.input.len() {
            return None; // Error: Unterminated string
        }

        let string_literal = self.input[start..self.position].to_string();
        self.position += 1; // Skip the closing quote
        Some(Token::StringLiteral(string_literal))
    }

    fn read_identifier(&mut self) -> Option<Token> {
        let start = self.position;

        while self.position < self.input.len()
            && (self.current_char().is_alphanumeric() || self.current_char() == '_')
        {
            self.position += 1;
        }

        let identifier = &self.input[start..self.position];

        match identifier {
            "printf" => Some(Token::KeywordPrintf),
            _ => Some(Token::Identifier(identifier.to_string())),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Token {
    KeywordPrintf,
    StringLiteral(String),
    ParenOpen,
    ParenClose,
    EOF, // End of file
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
            '"' => self.read_string(),
            'a'..='z' | 'A'..='Z' => self.read_keyword(),
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

    fn read_keyword(&mut self) -> Option<Token> {
        let start = self.position;

        while self.position < self.input.len()
            && self.current_char().is_alphanumeric()
        {
            self.position += 1;
        }

        let keyword = &self.input[start..self.position];

        match keyword {
            "printf" => Some(Token::KeywordPrintf),
            _ => None, // Unknown keyword
        }
    }
}

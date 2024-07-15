use crate::token::token::Token;
use crate::token::token_type::TokenType;

pub struct Scanner {
    pub source: String,
    pub line: usize,
    pub current: usize,
    pub start: usize,
    pub tokens: Vec<Token>,
    pub exit_code: u8,
}

impl Default for Scanner {
    fn default() -> Scanner {
        Scanner {
            source: String::from(""),
            line: 1,
            current: 0,
            start: 0,
            tokens: Vec::new(),
            exit_code: 0,
        }
    }
}

impl Scanner {
    fn is_at_end(&self) -> bool {
        return self.current >= self.source.chars().count();
    }

    pub fn scan_tokens(&mut self) -> u8 {
        while !self.is_at_end() {
            // We are at the beginning of the next lexeme.
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token {
            token_type: TokenType::EOF,
            lexeme: String::from(""),
            literal: None,
            line: self.line,
        });

        return self.exit_code;
    }

    fn match_expected(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        };
        if self.source.chars().nth(self.current) != Some(expected) {
            return false;
        };

        self.current += 1;
        return true;
    }

    fn advance(&mut self) -> Option<char> {
        let char = self.source.chars().nth(self.current);
        self.current += 1;
        return char;
    }

    fn add_token(&mut self, token_type: TokenType, literal: Option<String>) {
        let lexeme = self.source.get(self.start..self.current).unwrap();
        self.tokens.push(Token {
            token_type,
            lexeme: lexeme.to_string(),
            literal,
            line: self.line,
        });
    }

    fn peek(&self) -> Option<char> {
        if self.is_at_end() {
            return None;
        };
        return self.source.chars().nth(self.current);
    }

    fn error(&mut self, message: String) {
        eprintln!("[line {}] Error: {}", self.line, message);
        self.exit_code = 65;
    }

    fn string(&mut self) {
        while self.peek() != Some('"') && !self.is_at_end() {
            if self.peek() == Some('\n') {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            self.error(String::from("Unterminated string."));
            return;
        }

        // The closing ".
        self.advance();

        // Trim the surrounding quotes.
        let value = self.source.get(self.start + 1..self.current - 1).unwrap();
        self.add_token(TokenType::STRING, Some(String::from(value)));
    }

    fn is_digit(&self, c: Option<char>) -> bool {
        return c >= Some('0') && c <= Some('9');
    }

    fn peek_next(&self) -> Option<char> {
        if (self.current + 1 >= self.source.chars().count()) {
            return None;
        };
        return self.source.chars().nth(self.current + 1);
    }

    fn number(&mut self) {
        while self.is_digit(self.peek()) {
            self.advance();
        }

        // Look for a fractional part.
        if self.peek() == Some('.') && self.is_digit(self.peek_next()) {
            // Consume the "."
            self.advance();

            while self.is_digit(self.peek()) {
                self.advance();
            }
        }

        self.add_token(
            TokenType::NUMBER,
            Some(String::from(
                self.source
                    .get(self.start..self.current)
                    .unwrap()
                    .parse::<f32>()
                    .unwrap()
                    .to_string(),
            )),
        );
    }

    fn scan_token(&mut self) {
        let token = self.advance();
        match token {
            Some('(') => self.add_token(TokenType::LEFT_PAREN, None),
            Some(')') => self.add_token(TokenType::RIGHT_PAREN, None),
            Some('{') => self.add_token(TokenType::LEFT_BRACE, None),
            Some('}') => self.add_token(TokenType::RIGHT_BRACE, None),
            Some(',') => self.add_token(TokenType::COMMA, None),
            Some('.') => self.add_token(TokenType::DOT, None),
            Some('-') => self.add_token(TokenType::MINUS, None),
            Some('+') => self.add_token(TokenType::PLUS, None),
            Some(';') => self.add_token(TokenType::SEMICOLON, None),
            Some('*') => self.add_token(TokenType::STAR, None),
            Some('!') => {
                if self.match_expected('=') {
                    self.add_token(TokenType::BANG_EQUAL, None);
                } else {
                    self.add_token(TokenType::BANG, None);
                }
            }
            Some('=') => {
                if self.match_expected('=') {
                    self.add_token(TokenType::EQUAL_EQUAL, None);
                } else {
                    self.add_token(TokenType::EQUAL, None);
                }
            }
            Some('<') => {
                if self.match_expected('=') {
                    self.add_token(TokenType::LESS_EQUAL, None);
                } else {
                    self.add_token(TokenType::LESS, None);
                }
            }
            Some('>') => {
                if self.match_expected('=') {
                    self.add_token(TokenType::GREATER_EQUAL, None);
                } else {
                    self.add_token(TokenType::GREATER, None);
                }
            }
            Some('/') => {
                if self.match_expected('/') {
                    // A comment goes until the end of the line.
                    while self.peek() != Some('\n') && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::SLASH, None);
                }
            }
            Some(' ') => {}
            Some('\r') => {}
            Some('\t') => {}
            Some('\n') => {
                self.line += 1;
            }
            Some('"') => self.string(),
            Some(other) => {
                if self.is_digit(Some(other)) {
                    self.number();
                } else {
                    self.error(format!("Unexpected character: {}", other));
                }
            }
            None => todo!(
                "Unexpected token {:?} at position {:?}",
                token,
                self.current
            ),
        }
    }
}

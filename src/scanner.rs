use std::collections::HashMap;

use crate::token::token::Literal;
use crate::token::token::Token;
use crate::token::token_type::TokenType;

pub struct Scanner<'a> {
    pub source: &'a str,
    pub line: usize,
    pub current: usize,
    pub start: usize,
    pub tokens: Vec<Token<'a>>,
    pub exit_code: u8,
    pub keywords: HashMap<String, TokenType>,
}

impl<'a> Default for Scanner<'a> {
    fn default() -> Scanner<'a> {
        Scanner {
            source: "",
            line: 1,
            current: 0,
            start: 0,
            tokens: Vec::new(),
            exit_code: 0,
            keywords: HashMap::from([
                (String::from("and"), TokenType::And),
                (String::from("class"), TokenType::Class),
                (String::from("else"), TokenType::Else),
                (String::from("false"), TokenType::False),
                (String::from("for"), TokenType::For),
                (String::from("fun"), TokenType::Fun),
                (String::from("if"), TokenType::If),
                (String::from("nil"), TokenType::Nil),
                (String::from("or"), TokenType::Or),
                (String::from("print"), TokenType::Print),
                (String::from("return"), TokenType::Return),
                (String::from("super"), TokenType::Super),
                (String::from("this"), TokenType::This),
                (String::from("true"), TokenType::True),
                (String::from("var"), TokenType::Var),
                (String::from("while"), TokenType::While),
            ]),
        }
    }
}

impl<'a> Scanner<'a> {
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
            lexeme: "",
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

    fn add_token(&mut self, token_type: TokenType, literal: Option<Literal>) {
        let lexeme = self.source.get(self.start..self.current).unwrap();
        self.tokens.push(Token {
            token_type,
            lexeme,
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
        self.add_token(TokenType::String, Some(Literal::Str(String::from(value))));
    }

    fn is_digit(&self, c: Option<char>) -> bool {
        return c >= Some('0') && c <= Some('9');
    }

    fn peek_next(&self) -> Option<char> {
        if self.current + 1 >= self.source.chars().count() {
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
            TokenType::Number,
            Some(Literal::Number(
                self.source
                    .get(self.start..self.current)
                    .unwrap()
                    .parse::<f64>()
                    .unwrap(),
            )),
        );
    }

    fn is_alpha(&self, c: Option<char>) -> bool {
        return (c >= Some('a') && c <= Some('z'))
            || (c >= Some('A') && c <= Some('Z'))
            || c == Some('_');
    }

    fn is_alpha_numeric(&self, c: Option<char>) -> bool {
        return self.is_alpha(c) || self.is_digit(c);
    }

    fn identifier(&mut self) {
        while self.is_alpha_numeric(self.peek()) {
            self.advance();
        }

        let text = self.source.get(self.start..self.current).unwrap();
        let identifier = match self.keywords.get(text) {
            Some(keyword) => keyword,
            None => &TokenType::Identifier,
        };
        self.add_token(*identifier, None);
    }

    fn scan_token(&mut self) {
        let token = self.advance();
        match token {
            Some('(') => self.add_token(TokenType::LeftParen, None),
            Some(')') => self.add_token(TokenType::RightParen, None),
            Some('{') => self.add_token(TokenType::LeftBrace, None),
            Some('}') => self.add_token(TokenType::RightBrace, None),
            Some(',') => self.add_token(TokenType::Comma, None),
            Some('.') => self.add_token(TokenType::Dot, None),
            Some('-') => self.add_token(TokenType::Minus, None),
            Some('+') => self.add_token(TokenType::Plus, None),
            Some(';') => self.add_token(TokenType::Semicolon, None),
            Some('*') => self.add_token(TokenType::Star, None),
            Some('!') => {
                if self.match_expected('=') {
                    self.add_token(TokenType::BangEqual, None);
                } else {
                    self.add_token(TokenType::Bang, None);
                }
            }
            Some('=') => {
                if self.match_expected('=') {
                    self.add_token(TokenType::EqualEqual, None);
                } else {
                    self.add_token(TokenType::Equal, None);
                }
            }
            Some('<') => {
                if self.match_expected('=') {
                    self.add_token(TokenType::LessEqual, None);
                } else {
                    self.add_token(TokenType::Less, None);
                }
            }
            Some('>') => {
                if self.match_expected('=') {
                    self.add_token(TokenType::GreaterEqual, None);
                } else {
                    self.add_token(TokenType::Greater, None);
                }
            }
            Some('/') => {
                if self.match_expected('/') {
                    // A comment goes until the end of the line.
                    while self.peek() != Some('\n') && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash, None);
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
                } else if self.is_alpha(Some(other)) {
                    self.identifier();
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

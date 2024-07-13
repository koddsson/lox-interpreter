use std::env;
use std::fmt;
use std::fs;
use std::io::{self, Write};

#[derive(Debug)]
enum TokenType {
    // Single-character tokens.
    LEFT_PAREN,
    RIGHT_PAREN,
    LEFT_BRACE,
    RIGHT_BRACE,
    COMMA,
    DOT,
    MINUS,
    PLUS,
    SEMICOLON,
    SLASH,
    STAR,

    // One or two character tokens.
    BANG,
    BANG_EQUAL,
    EQUAL,
    EQUAL_EQUAL,
    GREATER,
    GREATER_EQUAL,
    LESS,
    LESS_EQUAL,

    // Literals.
    IDENTIFIER,
    STRING,
    NUMBER,

    // Keywords.
    AND,
    CLASS,
    ELSE,
    FALSE,
    FUN,
    FOR,
    IF,
    NIL,
    OR,
    PRINT,
    RETURN,
    SUPER,
    THIS,
    TRUE,
    VAR,
    WHILE,

    EOF,
}

#[derive(Debug)]
struct Token {
    token_type: Option<TokenType>,
    lexeme: char,
    literal: Option<String>,
    line: usize,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.lexeme != ' ' {
            write!(
                f,
                "{:?} {} {:?}",
                self.token_type, self.lexeme, self.literal,
            )
        } else {
            write!(f, "{:?}  {:?}", self.token_type, self.literal)
        }
    }
}

fn scan_token(c: char) -> Option<TokenType> {
    match c {
        '(' => Some(TokenType::LEFT_PAREN),
        ')' => Some(TokenType::RIGHT_PAREN),
        '{' => Some(TokenType::LEFT_BRACE),
        '}' => Some(TokenType::RIGHT_BRACE),
        ',' => Some(TokenType::COMMA),
        '.' => Some(TokenType::DOT),
        '-' => Some(TokenType::MINUS),
        '+' => Some(TokenType::PLUS),
        ';' => Some(TokenType::SEMICOLON),
        '*' => Some(TokenType::STAR),
        _ => {
            println!("[line 1] Error: Unexpected character: {}", c);
            None
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        writeln!(io::stderr(), "Usage: {} tokenize <filename>", args[0]).unwrap();
        return;
    }

    let command = &args[1];
    let filename = &args[2];

    match command.as_str() {
        "tokenize" => {
            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
                String::new()
            });

            let tokens = file_contents.chars().map(|c| {
                let token_type = scan_token(c);

                Token {
                    token_type,
                    lexeme: c,
                    literal: Some(c.to_string()),
                    line: 1,
                }
            });

            for token in tokens {
                if token.token_type.is_none() {
                    continue;
                }
                println!("{:?}", token);
            }
        }
        _ => {
            writeln!(io::stderr(), "Unknown command: {}", command).unwrap();
            return;
        }
    }
}

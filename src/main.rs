use std::env;
use std::fmt;
use std::fs;
use std::io::{self, Write};
use std::process::ExitCode;

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

struct Token {
    token_type: TokenType,
    lexeme: char,
    literal: String,
    line: usize,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} {} {}", self.token_type, self.lexeme, self.literal)
    }
}

struct UnexpectedTokenError {
    line: usize,
    token: char,
}

impl fmt::Display for UnexpectedTokenError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "[line {}] Error: Unexpected character: {}",
            self.line, self.token
        )
    }
}

/*
[line 1] Error: Unexpected character: $
[line 1] Error: Unexpected character: #
COMMA , null
DOT . null
LEFT_PAREN ( null
EOF  null
*/

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        writeln!(io::stderr(), "Usage: {} tokenize <filename>", args[0]).unwrap();
        return ExitCode::FAILURE;
    }

    let command = &args[1];
    let filename = &args[2];

    let mut result = 0;

    match command.as_str() {
        "tokenize" => {
            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
                String::new()
            });

            fn scan_token(line: usize, token: char) -> Result<TokenType, UnexpectedTokenError> {
                match token {
                    '(' => Ok(TokenType::LEFT_PAREN),
                    ')' => Ok(TokenType::RIGHT_PAREN),
                    '{' => Ok(TokenType::LEFT_BRACE),
                    '}' => Ok(TokenType::RIGHT_BRACE),
                    ',' => Ok(TokenType::COMMA),
                    '.' => Ok(TokenType::DOT),
                    '-' => Ok(TokenType::MINUS),
                    '+' => Ok(TokenType::PLUS),
                    ';' => Ok(TokenType::SEMICOLON),
                    '*' => Ok(TokenType::STAR),
                    _ => Err(UnexpectedTokenError { line, token }),
                }
            }

            let mut tokens: Vec<Token> = Vec::new();
            let mut line = 1;

            for c in file_contents.chars() {
                if c == '\n' {
                    line += 1;
                    continue;
                }
                match scan_token(line, c) {
                    Ok(token_type) => {
                        tokens.push(Token {
                            token_type,
                            lexeme: c,
                            literal: String::from("null"),
                            line,
                        });
                    }
                    Err(err) => {
                        eprintln!("{}", err);
                        result = 65;
                    }
                }
            }

            for token in tokens {
                println!("{}", token);
            }

            println!("EOF  null");

            return ExitCode::from(result);
        }
        _ => {
            writeln!(io::stderr(), "Unknown command: {}", command).unwrap();
            return ExitCode::FAILURE;
        }
    }
}

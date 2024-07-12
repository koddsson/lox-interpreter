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

struct Token {
    token_type: TokenType,
    lexeme: char,
    literal: String,
    line: u32,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.lexeme != ' ' {
            write!(f, "{:?} {} {}", self.token_type, self.lexeme, self.literal)
        } else {
            write!(f, "{:?}  {}", self.token_type, self.literal)
        }
    }
}

fn scan_token(c: char) -> TokenType {
    match c {
        '(' => TokenType::LEFT_PAREN,
        ')' => TokenType::RIGHT_PAREN,
        '{' => TokenType::LEFT_BRACE,
        '}' => TokenType::RIGHT_BRACE,
        ',' => TokenType::COMMA,
        '.' => TokenType::DOT,
        '-' => TokenType::MINUS,
        '+' => TokenType::PLUS,
        ';' => TokenType::SEMICOLON,
        '*' => TokenType::STAR,
        _ => todo!("{}", c),
    }
}

fn scan_tokens<'a>(program: String) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();

    for char in program.chars() {
        if char == '\n' {
            continue;
        };
        let token = Token {
            token_type: scan_token(char),
            lexeme: char,
            literal: String::from("null"),
            line: 1,
        };
        tokens.push(token);
    }

    tokens.push(Token {
        token_type: TokenType::EOF,
        lexeme: ' ',
        literal: String::from("null"),
        line: 1,
    });

    return tokens;
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

            for token in scan_tokens(file_contents) {
                println!("{}", token);
            }
        }
        _ => {
            writeln!(io::stderr(), "Unknown command: {}", command).unwrap();
            return;
        }
    }
}

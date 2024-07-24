#![allow(clippy::needless_return)]
#![warn(clippy::integer_arithmetic)]

use std::collections::HashMap;
use std::env;
use std::fs;
use std::process::ExitCode;

use interpreter::Environment;

use crate::interpreter::Interpreter;
use crate::parser::Parser;
use crate::tokenizer::Tokenizer;

mod expr;
pub mod interpreter;
mod parse_error;
mod parser;
mod statement;
pub mod token;
pub mod tokenizer;

#[derive(Debug, PartialEq)]
pub enum Error {
    TokenizingError(String),
    ParseError(String),
    InterpreterError(String),
}

pub fn execute(source: &str) -> Result<Vec<String>, Error> {
    let mut tokenizer = Tokenizer {
        source,
        ..Default::default()
    };

    let results = tokenizer.scan_tokens();
    if results != 0 {
        return Err(Error::TokenizingError("Failed parsing tokens".to_string()));
    }

    let mut parser = Parser {
        tokens: tokenizer.tokens,
        ..Default::default()
    };

    let statements = match parser.parse() {
        Ok(statements) => statements,
        Err(err) => {
            return Err(Error::ParseError(err.to_string()));
        }
    };

    let mut interpreter = Interpreter {
        environment: Environment {
            map: HashMap::new(),
            enclosing: HashMap::new(),
        },
    };

    match interpreter.interpret(statements) {
        Ok(values) => Ok(values.into_iter().map(|value| value.to_string()).collect()),
        Err(err) => return Err(Error::InterpreterError(err.to_string())),
    }
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} tokenize <filename>", args[0]);
        return ExitCode::FAILURE;
    }

    let command = &args[1];
    let filename = &args[2];

    let source = fs::read_to_string(filename).unwrap_or_else(|_| {
        eprintln!("Failed to read file {}", filename);
        String::new()
    });

    match command.as_str() {
        "tokenize" => {
            let mut tokenizer = Tokenizer {
                source: source.as_str(),
                ..Default::default()
            };

            let results = tokenizer.scan_tokens();
            for token in tokenizer.tokens {
                println!("{}", token);
            }

            return ExitCode::from(results);
        }
        "parse" => {
            let mut tokenizer = Tokenizer {
                source: source.as_str(),
                ..Default::default()
            };

            let results = tokenizer.scan_tokens();
            if results != 0 {
                return ExitCode::from(results);
            }

            let mut parser = Parser {
                tokens: tokenizer.tokens,
                ..Default::default()
            };

            let statements = match parser.parse() {
                Ok(statements) => statements,
                Err(err) => {
                    eprintln!("{}", err);
                    return ExitCode::from(65);
                }
            };

            for statement in statements {
                println!("{}", statement);
            }

            return ExitCode::from(results);
        }
        "interpret" => {
            let mut tokenizer = Tokenizer {
                source: source.as_str(),
                ..Default::default()
            };

            let results = tokenizer.scan_tokens();
            if results != 0 {
                return ExitCode::from(results);
            }

            let mut parser = Parser {
                tokens: tokenizer.tokens,
                ..Default::default()
            };

            let statements = match parser.parse() {
                Ok(statements) => statements,
                Err(err) => {
                    eprintln!("{}", err);
                    return ExitCode::from(65);
                }
            };

            let mut interpreter = Interpreter {
                environment: Environment {
                    map: HashMap::new(),
                    enclosing: HashMap::new(),
                },
            };

            match interpreter.interpret(statements) {
                Ok(value) => value,
                Err(err) => {
                    eprintln!("{}", err);
                    return ExitCode::from(70);
                }
            };

            return ExitCode::from(results);
        }
        _ => {
            eprintln!("Unknown command: {}", command);
            return ExitCode::FAILURE;
        }
    }
}

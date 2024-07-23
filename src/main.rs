#![allow(clippy::needless_return)]
use std::env;
use std::fs;
use std::process::ExitCode;

use crate::interpreter::interpret;
use crate::parser::Parser;
use crate::tokenizer::Tokenizer;

mod environment;
mod expr;
mod interpreter;
mod parse_error;
mod parser;
mod statement;
mod symbol;
mod token;
mod tokenizer;
mod value;

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

            let parser = Parser {
                tokens: tokenizer.tokens,
                ..Default::default()
            };

            let expressions = match parser.parse() {
                Ok(expressions) => expressions,
                Err(err) => {
                    eprintln!("{}", err);
                    return ExitCode::from(65);
                }
            };

            for expression in expressions {
                println!("{}", expression);
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

            interpret(statements);

            return ExitCode::from(results);
        }
        _ => {
            eprintln!("Unknown command: {}", command);
            return ExitCode::FAILURE;
        }
    }
}

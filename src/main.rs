#![allow(clippy::needless_return)]
use std::env;
use std::fs;
use std::process::ExitCode;

use crate::parser::Parser;
use crate::tokenizer::Tokenizer;

mod expr;
mod parse_error;
mod parser;
mod token;
mod tokenizer;

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

            let expression = match parser.parse() {
                Ok(expression) => expression,
                Err(err) => {
                    eprintln!("{}", err);
                    return ExitCode::from(65);
                }
            };

            println!("{}", expression);

            return ExitCode::from(results);
        }
        _ => {
            eprintln!("Unknown command: {}", command);
            return ExitCode::FAILURE;
        }
    }
}

#![allow(clippy::needless_return)]
use std::env;
use std::fs;
use std::process::ExitCode;

use crate::parser::Parser;
use crate::scanner::Scanner;

mod expr;
mod parse_error;
mod parser;
mod scanner;
mod token;

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
            let mut scanner = Scanner {
                source: source.as_str(),
                ..Default::default()
            };

            let results = scanner.scan_tokens();
            for token in scanner.tokens {
                println!("{}", token);
            }

            return ExitCode::from(results);
        }
        "parse" => {
            let mut scanner = Scanner {
                source: source.as_str(),
                ..Default::default()
            };

            let results = scanner.scan_tokens();

            let mut parser = Parser {
                tokens: scanner.tokens,
                ..Default::default()
            };

            if results != 0 {
                return ExitCode::from(results);
            }

            let expression = match parser.parse() {
                Ok(expression) => expression,
                Err(err) => {
                    eprintln!("{}", err);
                    return ExitCode::FAILURE;
                }
            };

            println!("{:?}", expression);

            return ExitCode::from(results);
        }
        _ => {
            eprintln!("Unknown command: {}", command);
            return ExitCode::FAILURE;
        }
    }
}

use std::env;
use std::fs;
use std::io::{self, Write};
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
        writeln!(io::stderr(), "Usage: {} tokenize <filename>", args[0]).unwrap();
        return ExitCode::FAILURE;
    }

    let command = &args[1];
    let filename = &args[2];

    let source = fs::read_to_string(filename).unwrap_or_else(|_| {
        writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
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

            let parser = Parser {
                tokens: scanner.tokens,
                ..Default::default()
            };

            if results != 0 {
                return ExitCode::from(results);
            }

            let results = parser.parse();
            //for token in scanner.tokens {
            //    println!("{}", token);
            //}

            return ExitCode::from(results);
        }
        _ => {
            writeln!(io::stderr(), "Unknown command: {}", command).unwrap();
            return ExitCode::FAILURE;
        }
    }
}

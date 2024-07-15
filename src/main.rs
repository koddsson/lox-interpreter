use std::env;
use std::fs;
use std::io::{self, Write};
use std::process::ExitCode;

use crate::scanner::Scanner;

mod scanner;
mod token;

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

    match command.as_str() {
        "tokenize" => {
            let source = fs::read_to_string(filename).unwrap_or_else(|_| {
                writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
                String::new()
            });

            let mut scanner = Scanner {
                source,
                ..Default::default()
            };

            scanner.scan_tokens();
            for token in scanner.tokens {
                println!("{}", token);
            }

            return ExitCode::from(0);
        }
        _ => {
            writeln!(io::stderr(), "Unknown command: {}", command).unwrap();
            return ExitCode::FAILURE;
        }
    }
}

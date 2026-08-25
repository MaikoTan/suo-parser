//! Command-line interface for suo-parser.
//!
//! Usage:
//!   suo [FILE]            Read FILE (or stdin if omitted / `-`) and round-trip it.
//!   suo --ast [FILE]      Print the parsed AST as debug output instead of generating.

use std::io::{self, Read, Write};
use std::process::ExitCode;

use suo_parser_core::generator::Generator;
use suo_parser_core::parser::Parser;
use suo_parser_core::tokenizer::Tokenizer;
use suo_parser_core::types::semantic_ast::Program;

fn main() -> ExitCode {
    let mut args = std::env::args().skip(1);
    let mut print_ast = false;
    let mut file: Option<String> = None;

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "-h" | "--help" => {
                eprintln!("Usage: suo [--ast] [FILE]");
                eprintln!("  FILE   timeline file to read, or `-`/omit for stdin");
                eprintln!("  --ast  print the parsed AST (debug) instead of generating");
                return ExitCode::SUCCESS;
            }
            "--ast" => print_ast = true,
            _ => file = Some(arg),
        }
    }

    let mut input = String::new();
    let read_result = match file.as_deref() {
        Some("-") | None => io::stdin().read_to_string(&mut input),
        Some(path) => std::fs::read_to_string(path).map(|s| {
            input = s;
            input.len()
        }),
    };

    if let Err(err) = read_result {
        eprintln!("error: failed to read input: {}", err);
        return ExitCode::FAILURE;
    }

    let tokenizer: Tokenizer<_> = input.as_str().into();
    let mut parser = Parser::new(tokenizer);
    let program: Program = parser.parse();

    let stdout = io::stdout();
    let mut out = stdout.lock();
    if print_ast {
        let _ = writeln!(out, "{:#?}", program);
    } else {
        let generated = Generator::new(program).generate();
        let _ = write!(out, "{}", generated);
    }

    ExitCode::SUCCESS
}

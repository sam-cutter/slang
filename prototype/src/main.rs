use lexer::Lexer;
use parser::Parser;
use source::Source;
use token_stream::TokenStream;

mod expression;
mod lexer;
mod parser;
mod source;
mod token;
mod token_stream;

use std::{
    env,
    io::{self, BufRead, Write},
};

fn main() {
    // The first argument is always the executable path (e.g. /bin/slang)
    let args = env::args().skip(1);

    if args.len() == 0 {
        run_prompt();
    } else {
        println!("No arguments expected.");
    }
}

fn run_prompt() {
    let mut line = String::new();

    let mut stdin = io::stdin().lock();
    let mut stdout = io::stdout().lock();

    loop {
        line.clear();

        print!("\n> ");
        let _ = stdout.flush();
        let _ = stdin.read_line(&mut line);

        run(line.trim());
    }
}

fn run(source: &str) {
    let source = Source::new(source);

    let lexer = Lexer::new(source);

    let (tokens, errors) = lexer.lex();

    for error in &errors {
        eprintln!("{:?}", error);
    }

    if errors.len() != 0 {
        return;
    }

    let tokens = TokenStream::new(tokens);

    let parser = Parser::new(tokens);

    match parser.parse() {
        Ok(expression) => match expression.evaluate() {
            Ok(literal) => println!("{:?}", literal),
            Err(error) => println!("{}", error),
        },
        Err(error) => {
            println!("{}", error)
        }
    }
}

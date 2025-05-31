use expression::Literal;
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
    env, fs,
    io::{self, BufRead, Write},
};

fn main() {
    let args = &env::args().collect::<Vec<String>>()[..];

    match args {
        [_executable] => run_prompt(),
        [_executable, filename] => run_file(filename),
        _ => println!("Usage: slang [filename]"),
    }
}

fn run_prompt() {
    let mut line = String::new();

    let mut stdin = io::stdin().lock();
    let mut stdout = io::stdout().lock();

    loop {
        line.clear();

        print!("> ");
        let _ = stdout.flush();
        let _ = stdin.read_line(&mut line);

        run(line.trim());
    }
}

fn run_file(filename: &str) {
    let contents = fs::read_to_string(filename);

    match contents {
        Ok(source) => run(&source),
        Err(_) => return,
    }
}

fn run(source: &str) {
    let source = Source::new(source);

    let lexer = Lexer::new(source);

    let (tokens, errors) = lexer.lex();

    for token in &tokens {
        println!("{:?}", token);
    }

    for error in &errors {
        eprintln!("{:?}", error);
    }

    println!();

    let tokens = TokenStream::new(tokens);

    let parser = Parser::new(tokens);

    match parser.parse() {
        Ok(expressions) => {
            for expression in expressions {
                println!("{:?}", &expression);
                println!();

                if let Ok(literal) = expression.evaluate() {
                    println!("{:?}", literal)
                }
            }
        }
        Err(errors) => {
            for error in errors {
                eprintln!("{:?}", error);
            }
        }
    }
}

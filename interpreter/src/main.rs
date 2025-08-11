use std::{
    env, fs,
    io::{self, BufRead, Write},
};

use lexer::Lexer;
use parser::Parser;
use source::Source;
use stack::Stack;
use token_stream::TokenStream;

mod environment;
mod expression;
mod lexer;
mod parser;
mod source;
mod stack;
mod statement;
mod token;
mod token_stream;

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

    let mut stack = Stack::new();

    loop {
        line.clear();

        print!("> ");
        let _ = stdout.flush();
        let _ = stdin.read_line(&mut line);

        run(line.trim(), &mut stack);
    }
}

fn run_file(filename: &str) {
    let contents = fs::read_to_string(filename);

    let mut stack = Stack::new();

    match contents {
        Ok(source) => run(&source, &mut stack),
        Err(error) => eprintln!("{}", error),
    }
}

fn run(source: &str, stack: &mut Stack) {
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
        Ok(statements) => {
            for statement in statements {
                if let Err(error) = statement.execute(stack) {
                    eprintln!("{}", error);
                    // TODO: return with an exit code
                    return;
                }
            }
        }
        Err(errors) => {
            for error in errors {
                eprintln!("{}", error);
            }
        }
    }
}

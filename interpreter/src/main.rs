use std::{
    env, fs,
    io::{self, BufRead, Write},
};

use heap::ManagedHeap;
use lexer::Lexer;
use parser::Parser;
use source::Source;
use stack::Stack;
use statement::{ControlFlow, Statement};
use token_stream::TokenStream;

mod environment;
mod expression;
mod heap;
mod lexer;
mod parser;
mod source;
mod stack;
mod statement;
mod token;
mod token_stream;
mod value;

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
    let mut heap = ManagedHeap::new();

    loop {
        line.clear();

        print!("> ");
        let _ = stdout.flush();
        let _ = stdin.read_line(&mut line);

        run(line.trim(), &mut stack, &mut heap);
    }
}

fn run_file(filename: &str) {
    let contents = fs::read_to_string(filename);

    let mut stack = Stack::new();
    let mut heap = ManagedHeap::new();

    match contents {
        Ok(source) => run(&source, &mut stack, &mut heap),
        Err(error) => eprintln!("{}", error),
    }
}

fn run(source: &str, stack: &mut Stack, heap: &mut ManagedHeap) {
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
            let mut non_definitions = Vec::new();

            for statement in statements {
                match statement {
                    Statement::FunctionDefinition {
                        identifier: _,
                        parameters: _,
                        block: _,
                    } => {
                        if let Err(error) = statement.execute(stack, heap) {
                            eprintln!("{}", error);
                            return;
                        }
                    }
                    _ => non_definitions.push(statement),
                }
            }

            for statement in non_definitions {
                match statement.execute(stack, heap) {
                    Ok(control) => match control {
                        ControlFlow::Continue => continue,
                        ControlFlow::Break(_) => return,
                    },
                    Err(error) => {
                        eprintln!("{}", error);
                        return;
                    }
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

use std::{
    env, fs,
    io::{self, BufRead, Write},
};

use heap::{
    ManagedHeap, garbage_collected::GarbageCollectedHeap, naive::NaiveHeap,
    reference_counted::ReferenceCountedHeap,
};
use lexer::Lexer;
use parser::Parser;
use source::Source;
use stack::Stack;
use statement::{ControlFlow, Statement};
use token_stream::TokenStream;

use crate::stats::Logger;

mod environment;
mod expression;
mod heap;
mod lexer;
mod parser;
mod source;
mod stack;
mod statement;
mod stats;
mod token;
mod token_stream;
mod value;

fn main() {
    let args = &env::args().collect::<Vec<String>>()[..];

    match args {
        [_executable, heap] if heap == "gc" => run_prompt(gc()),
        [_executable, heap] if heap == "rc" => run_prompt(rc()),
        [_executable, heap] if heap == "na" => run_prompt(na()),

        [_executable, heap, filename] if heap == "gc" => run_file(filename, gc()),
        [_executable, heap, filename] if heap == "rc" => run_file(filename, rc()),
        [_executable, heap, filename] if heap == "na" => run_file(filename, na()),

        _ => println!("Usage: slang <gc|rc|na> [filename]"),
    }
}

fn run_prompt(heap: ManagedHeap) {
    let mut line = String::new();

    let mut stdin = io::stdin().lock();
    let mut stdout = io::stdout().lock();

    let mut stack = Stack::new();
    let mut heap = heap;
    let mut logger = Logger::new();

    loop {
        line.clear();

        print!("> ");
        let _ = stdout.flush();
        let _ = stdin.read_line(&mut line);

        run(line.trim(), &mut stack, &mut heap, &mut logger);
    }
}

fn run_file(filename: &str, heap: ManagedHeap) {
    let contents = fs::read_to_string(filename);

    let mut stack = Stack::new();
    let mut heap = heap;
    let mut logger = Logger::new();

    match contents {
        Ok(source) => {
            run(&source, &mut stack, &mut heap, &mut logger);

            logger.new_entry(
                heap.objects_count(),
                stack.frames_count(),
                heap.size(),
                stack.size(),
            );

            logger.write_to_csv(filename);
        }
        Err(error) => eprintln!("{}", error),
    }
}

fn run(source: &str, stack: &mut Stack, heap: &mut ManagedHeap, logger: &mut Logger) {
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
                        if let Err(error) = statement.execute(stack, heap, logger) {
                            eprintln!("{}", error);
                            return;
                        }
                    }
                    _ => non_definitions.push(statement),
                }
            }

            for statement in non_definitions {
                match statement.execute(stack, heap, logger) {
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

fn gc() -> ManagedHeap {
    ManagedHeap::GarbageCollected(GarbageCollectedHeap::new())
}

fn rc() -> ManagedHeap {
    ManagedHeap::ReferenceCounted(ReferenceCountedHeap::new())
}

fn na() -> ManagedHeap {
    ManagedHeap::Naive(NaiveHeap::new())
}

use std::{
    cell::RefCell,
    env, fs,
    io::{self, BufRead, Write},
    rc::Rc,
};

use environment::Environment;
use lexer::Lexer;
use parser::Parser;
use source::Source;
use statement::Statement;
use token_stream::TokenStream;

use crate::statement::ControlFlow;

mod environment;
mod expression;
mod lexer;
mod parser;
mod source;
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

    let environment = Rc::new(RefCell::new(Environment::new(None)));

    loop {
        line.clear();

        print!("> ");
        let _ = stdout.flush();
        let _ = stdin.read_line(&mut line);

        run(line.trim(), Rc::clone(&environment));
    }
}

fn run_file(filename: &str) {
    let contents = fs::read_to_string(filename);

    let environment = Rc::new(RefCell::new(Environment::new(None)));

    match contents {
        Ok(source) => run(&source, environment),
        Err(error) => eprintln!("{}", error),
    }
}

fn run(source: &str, environment: Rc<RefCell<Environment>>) {
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
                        if let Err(error) = statement.execute(Rc::clone(&environment)) {
                            eprintln!("{}", error);
                            return;
                        }
                    }
                    _ => non_definitions.push(statement),
                }
            }

            for statement in non_definitions {
                match statement.execute(Rc::clone(&environment)) {
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

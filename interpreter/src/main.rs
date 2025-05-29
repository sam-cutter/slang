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

fn main() {
    let source = Source::new(
        r#"
    "#,
    );

    let lexer = Lexer::new(source);

    let (tokens, errors) = lexer.lex();

    for token in &tokens {
        println!("{:?}", token);
    }

    for error in &errors {
        eprintln!("{:?}", error);
    }

    let tokens = TokenStream::new(tokens);

    let parser = Parser::new(tokens);

    match parser.parse() {
        Ok(expressions) => {
            for expression in expressions {
                println!("{:#?}", expression);
            }
        }
        Err(errors) => {
            for error in errors {
                eprintln!("{:?}", error);
            }
        }
    }
}

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
    (1 / (1 + 1)))


    "#,
    );

    let lexer = Lexer::new(source);

    let (tokens, errors) = lexer.lex();

    for token in &tokens {
        println!("{:?}", token);
    }

    for error in &errors {
        println!("{:?}", error);
    }

    let tokens = TokenStream::new(tokens);

    let mut parser = Parser::new(tokens);

    match parser.expression() {
        Ok(expression) => println!("{:#?}", expression),
        Err(error) => println!("{:?}", error),
    }
}

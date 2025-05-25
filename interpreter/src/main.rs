use lexer::Lexer;
use parser::Parser;
use source::Source;

mod expression;
mod lexer;
mod parser;
mod source;
mod token;

fn main() {
    let mut source = Source::new("1 + 2;");

    let lexer = Lexer::new(source);

    let (tokens, errors): (Vec<token::Token>, Vec<lexer::LexerError>) = lexer.lex();

    for token in &tokens {
        println!("{:?}", token);
    }

    let parser = Parser::new(tokens);
}

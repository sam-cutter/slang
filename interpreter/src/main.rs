use lexer::Lexer;

mod lexer;
mod source;
mod token;

fn main() {
    let mut lexer = Lexer::new("// Testing\n\"hey\" + 1.25");

    let tokens = lexer.lex();

    for token in tokens {
        println!("{:#?}", token);
    }
}

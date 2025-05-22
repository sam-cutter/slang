use lexer::Lexer;

mod lexer;
mod source;
mod token;

fn main() {
    let mut lexer = Lexer::new(
        r#"
        let name = "Sam";

        print(name);

        print(1 == 2);
    "#,
    );

    let (tokens, errors): (&Vec<token::Token>, Vec<lexer::LexerError>) = lexer.lex();

    for token in tokens {
        println!("{:?}", token);
    }

    for error in errors {
        println!("{}", error);
    }
}

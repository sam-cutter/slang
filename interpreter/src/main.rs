use lexer::Lexer;

mod lexer;
mod source;
mod token;

fn main() {
    let mut lexer = Lexer::new(
        r#"let name = "Sam";
        greet(name);

        fun greet(name) {
            print("Hello " + name);
        }

        if (1 < 2.0) || (true != false) return;
    "#,
    );

    let tokens = lexer.lex().unwrap();

    for token in tokens {
        println!("{:#?}", token);
    }
}

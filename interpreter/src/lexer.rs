use crate::{
    source::{Location, Source},
    token::{Token, TokenCategory},
};

pub struct Lexer {
    source: Source,
    tokens: Vec<Token>,
    current_token_start: Location,
}

impl Lexer {
    pub fn new(source: &str) -> Self {
        Self {
            source: Source::new(source),
            tokens: Vec::new(),
            current_token_start: Location::start(),
        }
    }

    pub fn lex(&mut self) -> &Vec<Token> {
        self.current_token_start = self.source.location();

        while let Some(character) = self.source.advance() {
            match character {
                // Single character tokens (not including Slash)
                '(' => self.add_token(TokenCategory::LeftParenthesis),
                ')' => self.add_token(TokenCategory::RightParenthesis),
                '{' => self.add_token(TokenCategory::LeftBrace),
                '}' => self.add_token(TokenCategory::RightBrace),
                ',' => self.add_token(TokenCategory::Comma),
                '.' => self.add_token(TokenCategory::Dot),
                '+' => self.add_token(TokenCategory::Plus),
                '-' => self.add_token(TokenCategory::Minus),
                '*' => self.add_token(TokenCategory::Star),
                '/' => self.handle_slash(),
                ';' => self.add_token(TokenCategory::Semicolon),

                '!' => self.handle_bang(),
                '=' => self.handle_equal(),
                '<' => self.handle_less(),
                '>' => self.handle_greater(),
                '&' => self.handle_ampersand(),
                '|' => self.handle_pipe(),

                ' ' | '\r' | '\t' | '\n' => (),

                '"' => self.handle_string(),

                character if character.is_ascii_digit() => self.handle_number(character),

                character if character.is_ascii_alphabetic() || character == '_' => {
                    self.handle_word(character)
                }

                _ => unimplemented!(),
            }

            self.current_token_start = self.source.location();
        }

        return &self.tokens;
    }

    fn add_token(&mut self, category: TokenCategory) {
        self.tokens.push(Token::new(
            category,
            self.current_token_start,
            self.source.location().index - self.current_token_start.index,
        ));
    }

    fn handle_bang(&mut self) {
        if self.source.matches('=') {
            self.add_token(TokenCategory::BangEqual);
        } else {
            self.add_token(TokenCategory::Bang);
        }
    }

    fn handle_equal(&mut self) {
        if self.source.matches('=') {
            self.add_token(TokenCategory::DoubleEqual);
        } else {
            self.add_token(TokenCategory::Equal);
        }
    }

    fn handle_less(&mut self) {
        if self.source.matches('=') {
            self.add_token(TokenCategory::LessEqual);
        } else {
            self.add_token(TokenCategory::Less);
        }
    }

    fn handle_greater(&mut self) {
        if self.source.matches('=') {
            self.add_token(TokenCategory::GreaterEqual);
        } else {
            self.add_token(TokenCategory::Greater);
        }
    }

    fn handle_ampersand(&mut self) {
        if self.source.matches('&') {
            self.add_token(TokenCategory::DoubleAmpersand);
        } else {
            unimplemented!();
        }
    }

    fn handle_pipe(&mut self) {
        if self.source.matches('|') {
            self.add_token(TokenCategory::DoublePipe);
        } else {
            unimplemented!();
        }
    }

    fn handle_slash(&mut self) {
        if self.source.matches('/') {
            while self
                .source
                .peek()
                .is_some_and(|character| character != '\n')
            {
                self.source.advance();
            }
        } else {
            self.add_token(TokenCategory::Slash);
        }
    }

    fn handle_string(&mut self) {
        let mut string = String::new();

        while let Some(character) = self.source.peek() {
            if character == '"' {
                break;
            }

            string.push(character);
            self.source.advance();
        }

        if self.source.at_end() {
            unimplemented!()
        }

        // Consume the enclosing "
        self.source.advance();

        self.add_token(TokenCategory::String(string));
    }

    fn handle_number(&mut self, first_digit: char) {
        let mut number = String::new();

        number.push(first_digit);

        while let Some(character) = self.source.peek() {
            if character.is_ascii_digit() {
                number.push(character);
                self.source.advance();
            } else {
                break;
            }
        }

        if self.source.peek().is_some_and(|character| character == '.')
            && self
                .source
                .peek_after()
                .is_some_and(|character| character.is_ascii_digit())
        {
            number.push('.');
            self.source.advance();

            while let Some(character) = self.source.peek() {
                if character.is_ascii_digit() {
                    number.push(character);
                    self.source.advance();
                } else {
                    break;
                }
            }
        }

        let number: f64 = number.parse().unwrap();

        self.add_token(TokenCategory::Number(number))
    }

    fn handle_word(&mut self, first_character: char) {
        let mut word = String::new();

        word.push(first_character);

        while let Some(character) = self.source.peek() {
            if character.is_ascii_alphanumeric() || character == '_' {
                word.push(character);
                self.source.advance();
            } else {
                break;
            }
        }

        match word.as_str() {
            "true" => self.add_token(TokenCategory::Boolean(true)),
            "false" => self.add_token(TokenCategory::Boolean(false)),

            "if" => self.add_token(TokenCategory::If),
            "else" => self.add_token(TokenCategory::Else),
            "while" => self.add_token(TokenCategory::While),

            "fun" => self.add_token(TokenCategory::Fun),
            "return" => self.add_token(TokenCategory::Return),
            "let" => self.add_token(TokenCategory::Let),

            _ => self.add_token(TokenCategory::Identifier(word)),
        }
    }
}

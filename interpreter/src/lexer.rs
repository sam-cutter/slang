use std::{
    error::Error,
    fmt::{Debug, Display},
};

use crate::{
    source::{Location, Source},
    token::{Token, TokenCategory},
};

pub struct Lexer {
    source: Source,
    tokens: Vec<Token>,
    current_token_start: Location,
}

pub enum LexerError {
    UnterminatedString(Location),
    UnexpectedCharacter {
        location: Location,
        character: char,
        expected: Option<char>,
    },
}

impl Display for LexerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnterminatedString(location) => {
                write!(f, "Unterminated string, beginning at {}", location)
            }
            Self::UnexpectedCharacter {
                location,
                character,
                expected,
            } => write!(
                f,
                "Unexpected character at position {}: `{}`{}",
                location,
                character,
                match expected {
                    Some(expected) => format!(" (expected `{}`)", expected),
                    None => String::new(),
                }
            ),
        }
    }
}

impl Debug for LexerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl Error for LexerError {}

impl Lexer {
    pub fn new(source: &str) -> Self {
        Self {
            source: Source::new(source),
            tokens: Vec::new(),
            current_token_start: Location::start(),
        }
    }

    pub fn lex(&mut self) -> (&Vec<Token>, Vec<LexerError>) {
        let mut errors = Vec::new();

        self.current_token_start = self.source.location();

        while let Some(character) = self.source.advance() {
            let result = match character {
                '(' => Ok(self.add_token(TokenCategory::LeftParenthesis)),
                ')' => Ok(self.add_token(TokenCategory::RightParenthesis)),
                '{' => Ok(self.add_token(TokenCategory::LeftBrace)),
                '}' => Ok(self.add_token(TokenCategory::RightBrace)),
                ',' => Ok(self.add_token(TokenCategory::Comma)),
                '.' => Ok(self.add_token(TokenCategory::Dot)),
                ';' => Ok(self.add_token(TokenCategory::Semicolon)),

                // Arithmetic operators
                '+' => Ok(self.add_token(TokenCategory::Plus)),
                '-' => Ok(self.add_token(TokenCategory::Minus)),
                '*' => Ok(self.add_token(TokenCategory::Star)),
                '/' => Ok(self.handle_slash()),

                // Logical and bitwise operators
                '!' => Ok(self.handle_bang()),
                '=' => Ok(self.handle_equal()),
                '>' => Ok(self.handle_greater()),
                '<' => Ok(self.handle_less()),
                '&' => Ok(self.handle_ampersand()),
                '|' => Ok(self.handle_pipe()),

                // Literals (not including booleans or null)
                '"' => self.handle_string(),
                character if character.is_ascii_digit() => Ok(self.handle_number(character)),

                // Identifiers and keywords
                character if character.is_ascii_alphabetic() || character == '_' => {
                    Ok(self.handle_word(character))
                }

                // Whitespace
                ' ' | '\r' | '\t' | '\n' => Ok(()),

                // Unexpected characters
                _ => Err(LexerError::UnexpectedCharacter {
                    location: self.current_token_start,
                    character: character,
                    expected: None,
                }),
            };

            if let Err(error) = result {
                errors.push(error);
            }

            self.current_token_start = self.source.location();
        }

        (&self.tokens, errors)
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
            self.add_token(TokenCategory::Ampersand);
        }
    }

    fn handle_pipe(&mut self) {
        if self.source.matches('|') {
            self.add_token(TokenCategory::DoublePipe);
        } else {
            self.add_token(TokenCategory::Pipe);
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

    fn handle_string(&mut self) -> Result<(), LexerError> {
        let mut string = String::new();

        while let Some(character) = self.source.peek() {
            if character == '"' {
                break;
            }

            string.push(character);
            self.source.advance();
        }

        if self.source.at_end() {
            return Err(LexerError::UnterminatedString(self.current_token_start));
        }

        // Consume the enclosing "
        self.source.advance();

        self.add_token(TokenCategory::String(string));

        Ok(())
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

        self.add_token(TokenCategory::Number(number));
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
            // Literals
            "true" => self.add_token(TokenCategory::Boolean(true)),
            "false" => self.add_token(TokenCategory::Boolean(false)),
            "null" => self.add_token(TokenCategory::Null),

            // Control flow
            "if" => self.add_token(TokenCategory::If),
            "else" => self.add_token(TokenCategory::Else),
            "while" => self.add_token(TokenCategory::While),
            "return" => self.add_token(TokenCategory::Return),

            // Identifier related
            "let" => self.add_token(TokenCategory::Let),
            "fun" => self.add_token(TokenCategory::Fun),
            _ => self.add_token(TokenCategory::Identifier(word)),
        };
    }
}

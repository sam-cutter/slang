use std::{
    error::Error,
    fmt::{Debug, Display},
};

use crate::{
    source::{GeneralLocation, Location, Source},
    token::{Token, TokenData},
};

pub enum LexerError {
    UnexpectedCharacter {
        location: Location,
        character: char,
        expected: Option<char>,
    },
    UnexpectedEndOfFile {
        expected: char,
    },
    UnknownKeyword {
        location: Location,
        keyword: String,
    },
}

impl Display for LexerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnexpectedCharacter {
                location,
                character,
                expected,
            } => write!(
                f,
                "{} Unexpected character: `{}`{}",
                location,
                character,
                match expected {
                    Some(expected) => format!(" (expected `{}`)", expected),
                    None => String::new(),
                }
            ),
            Self::UnexpectedEndOfFile { expected } => {
                write!(
                    f,
                    "{} Reached end of file, but expected `{}`",
                    GeneralLocation::EndOfFile,
                    expected
                )
            }
            Self::UnknownKeyword { location, keyword } => {
                write!(f, "{} Unexpected keyword: `{}`", location, keyword)
            }
        }
    }
}

impl Debug for LexerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl Error for LexerError {}

pub struct Lexer {
    source: Source,
    tokens: Vec<Token>,
    current_token_start: Location,
}

impl Lexer {
    pub fn new(source: Source) -> Self {
        Self {
            source: source,
            tokens: Vec::new(),
            current_token_start: Location::start(),
        }
    }

    pub fn lex(mut self) -> (Vec<Token>, Vec<LexerError>) {
        let mut errors = Vec::new();

        while let Some(character) = self.source.advance() {
            let result = match character {
                '(' => Ok(self.add_token(TokenData::LeftParenthesis)),
                ')' => Ok(self.add_token(TokenData::RightParenthesis)),
                '?' => Ok(self.add_token(TokenData::QuestionMark)),
                ':' => Ok(self.add_token(TokenData::Colon)),

                // Arithmetic operators
                '+' => Ok(self.add_token(TokenData::Plus)),
                '-' => Ok(self.add_token(TokenData::Minus)),
                '*' => Ok(self.add_token(TokenData::Star)),
                '/' => Ok(self.add_token(TokenData::Slash)),

                // Logical and bitwise operators
                '!' => Ok(self.handle_bang()),
                '=' => self.handle_equal(),
                '>' => Ok(self.handle_greater()),
                '<' => Ok(self.handle_less()),
                '&' => Ok(self.handle_ampersand()),
                '|' => Ok(self.handle_pipe()),

                character if character.is_ascii_digit() => Ok(self.handle_number(character)),

                // Identifiers and keywords
                character if character.is_ascii_alphabetic() || character == '_' => {
                    self.handle_word(character)
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

        (self.tokens, errors)
    }

    fn add_token(&mut self, data: TokenData) {
        self.tokens.push(Token::new(data, self.current_token_start));
    }

    fn handle_bang(&mut self) {
        if self.source.matches('=') {
            self.add_token(TokenData::BangEqual);
        } else {
            self.add_token(TokenData::Bang);
        }
    }

    fn handle_equal(&mut self) -> Result<(), LexerError> {
        if let Some(character) = self.source.peek() {
            if character == '=' {
                self.source.advance();
                Ok(self.add_token(TokenData::DoubleEqual))
            } else {
                Err(LexerError::UnexpectedCharacter {
                    location: self.current_token_start,
                    character,
                    expected: Some('='),
                })
            }
        } else {
            Err(LexerError::UnexpectedEndOfFile { expected: '=' })
        }
    }

    fn handle_less(&mut self) {
        if self.source.matches('=') {
            self.add_token(TokenData::LessEqual);
        } else {
            self.add_token(TokenData::Less);
        }
    }

    fn handle_greater(&mut self) {
        if self.source.matches('=') {
            self.add_token(TokenData::GreaterEqual);
        } else {
            self.add_token(TokenData::Greater);
        }
    }

    fn handle_ampersand(&mut self) {
        if self.source.matches('&') {
            self.add_token(TokenData::DoubleAmpersand);
        } else {
            self.add_token(TokenData::Ampersand);
        }
    }

    fn handle_pipe(&mut self) {
        if self.source.matches('|') {
            self.add_token(TokenData::DoublePipe);
        } else {
            self.add_token(TokenData::Pipe);
        }
    }

    fn handle_number(&mut self, first_digit: char) {
        let mut number = String::new();

        number.push(first_digit);

        while let Some(character) = self.source.peek() {
            if !character.is_ascii_digit() {
                break;
            }

            number.push(character);
            self.source.advance();
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
                if !character.is_ascii_digit() {
                    break;
                }

                number.push(character);
                self.source.advance();
            }

            let number: f64 = number.parse().unwrap();

            self.add_token(TokenData::Float(number))
        } else {
            let number: i32 = number.parse().unwrap();

            self.add_token(TokenData::Integer(number));
        }
    }

    fn handle_word(&mut self, first_character: char) -> Result<(), LexerError> {
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
            "true" => self.add_token(TokenData::Boolean(true)),
            "false" => self.add_token(TokenData::Boolean(false)),

            _ => Err(LexerError::UnknownKeyword {
                location: self.current_token_start,
                keyword: word,
            })?,
        };

        Ok(())
    }
}

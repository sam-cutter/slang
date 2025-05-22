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
    EndOfFileReached(Option<char>),
}

impl Display for LexerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnterminatedString(location) => {
                write!(f, "Unterminated string at position {}", location)
            }
            Self::UnexpectedCharacter {
                location,
                character,
                expected,
            } => write!(
                f,
                "Unexpected character: `{}` at position {}{}",
                character,
                location,
                if let Some(expected) = expected {
                    format!(" (expected `{}`)", expected)
                } else {
                    String::new()
                }
            ),
            Self::EndOfFileReached(expected) => {
                if let Some(expected) = expected {
                    write!(f, "End of file reached, but expected `{}`", expected)
                } else {
                    write!(f, "End of file reached.")
                }
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

impl Lexer {
    pub fn new(source: &str) -> Self {
        Self {
            source: Source::new(source),
            tokens: Vec::new(),
            current_token_start: Location::start(),
        }
    }

    pub fn lex(&mut self) -> Result<&Vec<Token>, LexerError> {
        self.current_token_start = self.source.location();

        while let Some(character) = self.source.advance() {
            let result: Result<(), LexerError> = match character {
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

                ' ' | '\r' | '\t' | '\n' => Ok(()),

                '"' => self.handle_string(),

                character if character.is_ascii_digit() => self.handle_number(character),

                character if character.is_ascii_alphabetic() || character == '_' => {
                    self.handle_word(character)
                }

                _ => {
                    return Err(LexerError::UnexpectedCharacter {
                        location: self.current_token_start,
                        character: character,
                        expected: None
                    });
                }
            };

            self.current_token_start = self.source.location();
        }

        return Ok(&self.tokens);
    }

    fn add_token(&mut self, category: TokenCategory) -> Result<(), LexerError> {
        self.tokens.push(Token::new(
            category,
            self.current_token_start,
            self.source.location().index - self.current_token_start.index,
        ));

        Ok(())
    }

    fn handle_bang(&mut self) -> Result<(), LexerError> {
        if self.source.matches('=') {
            return self.add_token(TokenCategory::BangEqual);
        } else {
            return self.add_token(TokenCategory::Bang);
        }
    }

    fn handle_equal(&mut self) -> Result<(), LexerError> {
        if self.source.matches('=') {
            return self.add_token(TokenCategory::DoubleEqual);
        } else {
            return self.add_token(TokenCategory::Equal);
        }
    }

    fn handle_less(&mut self) -> Result<(), LexerError> {
        if self.source.matches('=') {
            return self.add_token(TokenCategory::LessEqual);
        } else {
            return self.add_token(TokenCategory::Less);
        }
    }

    fn handle_greater(&mut self) -> Result<(), LexerError> {
        if self.source.matches('=') {
            return self.add_token(TokenCategory::GreaterEqual);
        } else {
            return self.add_token(TokenCategory::Greater);
        }
    }

    fn handle_ampersand(&mut self) -> Result<(), LexerError> {
        if let Some(character) = self.source.peek() {
            if character == '&' {
                return self.add_token(TokenCategory::DoubleAmpersand);
            } else {
                return Err(LexerError::UnexpectedCharacter {
                    location: self.source.location(),
                    character: character,
                    expected: Some('|'),
                });
            }
        } else {
            return Err(LexerError::EndOfFileReached(Some('|')));
        }
    }

    fn handle_pipe(&mut self) -> Result<(), LexerError> {
        if let Some(character) = self.source.peek() {
            if character == '|' {
                return self.add_token(TokenCategory::DoublePipe);
            } else {
                return Err(LexerError::UnexpectedCharacter {
                    location: self.current_token_start,
                    character: character,
                    expected: Some('|'),
                });
            }
        } else {
            return Err(LexerError::EndOfFileReached(Some('|')));
        }
    }

    fn handle_slash(&mut self) -> Result<(), LexerError> {
        if self.source.matches('/') {
            while self
                .source
                .peek()
                .is_some_and(|character| character != '\n')
            {
                self.source.advance();
            }

            return Ok(());
        } else {
            return self.add_token(TokenCategory::Slash);
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

    fn handle_number(&mut self, first_digit: char) -> Result<(), LexerError> {
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
            "true" => self.add_token(TokenCategory::Boolean(true)),
            "false" => self.add_token(TokenCategory::Boolean(false)),

            "if" => self.add_token(TokenCategory::If),
            "else" => self.add_token(TokenCategory::Else),
            "while" => self.add_token(TokenCategory::While),

            "fun" => self.add_token(TokenCategory::Fun),
            "return" => self.add_token(TokenCategory::Return),
            "let" => self.add_token(TokenCategory::Let),

            "null" => self.add_token(TokenCategory::Null),

            _ => self.add_token(TokenCategory::Identifier(word)),
        }
    }
}

use crate::{
    source::{Location, Source},
    token::{Token, TokenKind},
};

use std::{
    error::Error,
    fmt::{Debug, Display},
};

pub struct Lexer {
    source: Source,
    tokens: Vec<Token>,
}

pub enum LexerError {
    UnterminatedString(Location),
    UnterminatedBlockComment(Location),
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
            Self::UnterminatedBlockComment(location) => {
                write!(f, "Unterminated block comment, beginning at {}", location)
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
    pub fn new(source: Source) -> Self {
        Self {
            source: source,
            tokens: Vec::new(),
        }
    }

    pub fn lex(mut self) -> (Vec<Token>, Vec<LexerError>) {
        let mut errors = Vec::new();

        while let Some(character) = self.source.advance() {
            let result = match character {
                '(' => Ok(self.add_token(TokenKind::LeftParenthesis)),
                ')' => Ok(self.add_token(TokenKind::RightParenthesis)),
                '{' => Ok(self.add_token(TokenKind::LeftBrace)),
                '}' => Ok(self.add_token(TokenKind::RightBrace)),
                ',' => Ok(self.add_token(TokenKind::Comma)),
                '.' => Ok(self.add_token(TokenKind::Dot)),
                ';' => Ok(self.add_token(TokenKind::Semicolon)),

                // Arithmetic operators
                '+' => Ok(self.add_token(TokenKind::Plus)),
                '-' => Ok(self.add_token(TokenKind::Minus)),
                '*' => Ok(self.add_token(TokenKind::Star)),
                '/' => self.handle_slash(),

                // Logical and bitwise operators
                '!' => Ok(self.handle_bang()),
                '=' => Ok(self.handle_equal()),
                '>' => Ok(self.handle_greater()),
                '<' => Ok(self.handle_less()),
                '&' => Ok(self.handle_ampersand()),
                '|' => Ok(self.handle_pipe()),

                // Literals (not including booleans or null)
                '"' => self.handle_string(),
                character if character.is_ascii_digit() => Ok(self.handle_number()),

                // Identifiers and keywords
                character if character.is_ascii_alphabetic() || character == '_' => {
                    Ok(self.handle_word(character))
                }

                // Whitespace
                ' ' | '\r' | '\t' | '\n' => Ok(()),

                // Unexpected characters
                _ => Err(LexerError::UnexpectedCharacter {
                    location: self.source.current_token_start(),
                    character: character,
                    expected: None,
                }),
            };

            if let Err(error) = result {
                errors.push(error);
            }

            self.source.new_token();
        }

        (self.tokens, errors)
    }

    fn add_token(&mut self, kind: TokenKind) {
        let (start, lexeme) = self.source.new_token();

        self.tokens.push(Token::new(kind, lexeme, start));
    }

    fn handle_bang(&mut self) {
        if self.source.matches('=') {
            self.add_token(TokenKind::BangEqual);
        } else {
            self.add_token(TokenKind::Bang);
        }
    }

    fn handle_equal(&mut self) {
        if self.source.matches('=') {
            self.add_token(TokenKind::DoubleEqual);
        } else {
            self.add_token(TokenKind::Equal);
        }
    }

    fn handle_less(&mut self) {
        if self.source.matches('=') {
            self.add_token(TokenKind::LessEqual);
        } else {
            self.add_token(TokenKind::Less);
        }
    }

    fn handle_greater(&mut self) {
        if self.source.matches('=') {
            self.add_token(TokenKind::GreaterEqual);
        } else {
            self.add_token(TokenKind::Greater);
        }
    }

    fn handle_ampersand(&mut self) {
        if self.source.matches('&') {
            self.add_token(TokenKind::DoubleAmpersand);
        } else {
            self.add_token(TokenKind::Ampersand);
        }
    }

    fn handle_pipe(&mut self) {
        if self.source.matches('|') {
            self.add_token(TokenKind::DoublePipe);
        } else {
            self.add_token(TokenKind::Pipe);
        }
    }

    fn handle_slash(&mut self) -> Result<(), LexerError> {
        // Block comments
        if self.source.matches('*') {
            while self.source.peek().is_some_and(|character| character != '*')
                || self
                    .source
                    .peek_after()
                    .is_some_and(|character| character != '/')
            {
                self.source.advance();
            }

            if self.source.peek().is_some() && self.source.peek_after().is_some() {
                self.source.advance();
                self.source.advance();
                return Ok(());
            } else {
                return Err(LexerError::UnterminatedBlockComment(
                    self.source.current_token_start(),
                ));
            }
        }
        // Single line comments
        else if self.source.matches('/') {
            while self
                .source
                .peek()
                .is_some_and(|character| character != '\n')
            {
                self.source.advance();
            }
        } else {
            self.add_token(TokenKind::Slash);
        }

        Ok(())
    }

    fn handle_string(&mut self) -> Result<(), LexerError> {
        while self.source.peek().is_some_and(|character| character != '"') {
            self.source.advance();
        }

        if self.source.at_end() {
            return Err(LexerError::UnterminatedString(
                self.source.current_token_start(),
            ));
        }

        // Consume the enclosing "
        self.source.advance();

        self.add_token(TokenKind::String);

        Ok(())
    }

    fn handle_number(&mut self) {
        while self
            .source
            .peek()
            .is_some_and(|character| character.is_ascii_digit())
        {
            self.source.advance();
        }

        if self.source.peek().is_some_and(|character| character == '.')
            && self
                .source
                .peek_after()
                .is_some_and(|character| character.is_ascii_digit())
        {
            self.source.advance();

            while self
                .source
                .peek()
                .is_some_and(|character| character.is_ascii_digit())
            {
                self.source.advance();
            }
        }

        self.add_token(TokenKind::Number);
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
            "true" | "false" => self.add_token(TokenKind::Boolean),
            "null" => self.add_token(TokenKind::Null),

            // Control flow
            "if" => self.add_token(TokenKind::If),
            "else" => self.add_token(TokenKind::Else),
            "while" => self.add_token(TokenKind::While),
            "return" => self.add_token(TokenKind::Return),

            // Identifier related
            "let" => self.add_token(TokenKind::Let),
            "fu" => self.add_token(TokenKind::Fu),
            _ => self.add_token(TokenKind::Identifier),
        };
    }
}

//! The lexer for the slang programming language.

use std::{
    error::Error,
    fmt::{Debug, Display},
};

use crate::{
    source::{Location, Source},
    token::{Token, TokenData},
};

/// All the errors which can occur while lexing.
pub enum LexerError {
    /// A string without the enclosing `"`.
    UnterminatedString(Location),
    /// A block comment without the enclosing `*/`.
    UnterminatedBlockComment(Location),
    /// An unexpected character, with an optional expected character.
    UnexpectedCharacter {
        //
        location: Location,
        character: char,
        expected: Option<char>,
    },
}

impl Display for LexerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnterminatedString(location) => {
                write!(f, "{} Unterminated string.", location)
            }
            Self::UnterminatedBlockComment(location) => {
                write!(f, "{} Unterminated block comment.", location)
            }
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
        }
    }
}

impl Debug for LexerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl Error for LexerError {}

/// An instance of a lexer, for a specific source code string.
pub struct Lexer {
    source: Source,
    tokens: Vec<Token>,
    current_token_start: Location,
}

impl Lexer {
    /// Creates a new [Lexer] for a specific [Source].
    pub fn new(source: Source) -> Self {
        Self {
            source: source,
            tokens: Vec::new(),
            current_token_start: Location::start(),
        }
    }

    /// Attempts to lexically analyse the source code to produce a sequence of tokens.
    ///
    /// Will consume the entire source code, returning all valid tokens, and any errors.
    pub fn lex(mut self) -> (Vec<Token>, Vec<LexerError>) {
        let mut errors = Vec::new();

        while let Some(character) = self.source.advance() {
            let result = match character {
                '(' => Ok(self.add_token(TokenData::LeftParenthesis)),
                ')' => Ok(self.add_token(TokenData::RightParenthesis)),
                '{' => Ok(self.add_token(TokenData::LeftBrace)),
                '}' => Ok(self.add_token(TokenData::RightBrace)),
                ',' => Ok(self.add_token(TokenData::Comma)),
                '.' => Ok(self.add_token(TokenData::Dot)),
                ';' => Ok(self.add_token(TokenData::Semicolon)),
                '?' => Ok(self.add_token(TokenData::QuestionMark)),
                ':' => Ok(self.add_token(TokenData::Colon)),

                // Arithmetic operators
                '+' => Ok(self.add_token(TokenData::Plus)),
                '-' => Ok(self.add_token(TokenData::Minus)),
                '*' => Ok(self.add_token(TokenData::Star)),
                '/' => self.handle_slash(),

                // Logical and bitwise operators
                '!' => Ok(self.handle_bang()),
                '=' => Ok(self.handle_equal()),
                '>' => Ok(self.handle_greater()),
                '<' => Ok(self.handle_less()),
                '&' => Ok(self.handle_ampersand()),
                '|' => Ok(self.handle_pipe()),

                // Literals (not including booleans)
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

        (self.tokens, errors)
    }

    /// Adds a token to the internal list of tokens.
    fn add_token(&mut self, data: TokenData) {
        self.tokens.push(Token::new(data, self.current_token_start));
    }

    /// Called when a `!` character is encountered.
    fn handle_bang(&mut self) {
        if self.source.matches('=') {
            self.add_token(TokenData::BangEqual);
        } else {
            self.add_token(TokenData::Bang);
        }
    }

    /// Called when a `=` character is encountered.
    fn handle_equal(&mut self) {
        if self.source.matches('=') {
            self.add_token(TokenData::DoubleEqual);
        } else {
            self.add_token(TokenData::Equal);
        }
    }

    /// Called when a `<` character is encountered.
    fn handle_less(&mut self) {
        if self.source.matches('=') {
            self.add_token(TokenData::LessEqual);
        } else {
            self.add_token(TokenData::Less);
        }
    }

    /// Called when a `>` character is encountered.
    fn handle_greater(&mut self) {
        if self.source.matches('=') {
            self.add_token(TokenData::GreaterEqual);
        } else {
            self.add_token(TokenData::Greater);
        }
    }

    /// Called when a `&` character is encountered.
    fn handle_ampersand(&mut self) {
        if self.source.matches('&') {
            self.add_token(TokenData::DoubleAmpersand);
        } else {
            self.add_token(TokenData::Ampersand);
        }
    }

    /// Called when a `|` character is encountered.
    fn handle_pipe(&mut self) {
        if self.source.matches('|') {
            self.add_token(TokenData::DoublePipe);
        } else {
            self.add_token(TokenData::Pipe);
        }
    }

    /// Called when a `/` character is encountered.
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
                    self.current_token_start,
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
            self.add_token(TokenData::Slash);
        }

        Ok(())
    }

    /// Called when a `"` character is encountered.
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

        self.add_token(TokenData::String(string));

        Ok(())
    }

    /// Called when a digit is encountered.
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

    /// Called when an alphabetic character is encountered.
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
            "true" => self.add_token(TokenData::Boolean(true)),
            "false" => self.add_token(TokenData::Boolean(false)),

            // Control flow
            "if" => self.add_token(TokenData::If),
            "else" => self.add_token(TokenData::Else),
            "while" => self.add_token(TokenData::While),
            "return" => self.add_token(TokenData::Return),

            // Identifier related
            "let" => self.add_token(TokenData::Let),
            "fu" => self.add_token(TokenData::Fu),

            _ => self.add_token(TokenData::Identifier(word)),
        };
    }
}

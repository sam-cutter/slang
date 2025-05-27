use std::{fmt::Display, mem};

#[derive(Clone, Copy, Debug)]
pub struct Location {
    pub index: usize,
    pub line: usize,
    pub column: usize,
}

impl Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.line, self.column)
    }
}

impl Location {
    pub fn start() -> Self {
        Self {
            index: 0,
            line: 1,
            column: 1,
        }
    }
}

pub struct Source {
    text: Vec<char>,
    location: Location,
    current_token_start: Location,
    current_token_lexeme: String,
}

impl Source {
    pub fn new(text: &str) -> Self {
        Self {
            text: text.chars().collect(),
            location: Location::start(),
            current_token_start: Location::start(),
            current_token_lexeme: String::new(),
        }
    }

    pub fn peek(&self) -> Option<char> {
        self.text.get(self.location.index).copied()
    }

    pub fn peek_after(&self) -> Option<char> {
        self.text.get(self.location.index + 1).copied()
    }

    pub fn advance(&mut self) -> Option<char> {
        let next = self.peek();

        if let Some(character) = next {
            self.current_token_lexeme.push(character);

            self.location.index += 1;
            self.location.column += 1;

            if character == '\n' {
                self.location.line += 1;
                self.location.column = 1;
            }
        }

        next
    }

    pub fn matches(&mut self, target: char) -> bool {
        if let Some(character) = self.peek() {
            if character == target {
                self.advance();
            }

            return character == target;
        }

        false
    }

    pub fn at_end(&self) -> bool {
        self.location.index >= self.text.len()
    }

    pub fn new_token(&mut self) -> (Location, String) {
        (
            mem::replace(&mut self.current_token_start, self.location),
            mem::replace(&mut self.current_token_lexeme, String::new()),
        )
    }

    pub fn current_token_start(&self) -> Location {
        self.current_token_start
    }
}

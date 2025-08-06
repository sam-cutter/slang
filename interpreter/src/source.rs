//! Code relating to the raw source code string.

use std::fmt::Display;

/// Represents the location of a character within a source code string.
#[derive(Clone, Copy, Debug)]
pub struct Location {
    /// The zero-indexed position of the character.
    pub index: usize,
    /// The line (`>= 1`) which the character appears on.
    pub line: usize,
    /// The column (`>= 1`) which the character appears in.
    pub column: usize,
}

impl Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[line {}, column {}]", self.line, self.column)
    }
}

impl Location {
    /// Returns a location representing the first character in a source code string, with index `0`, line `1`, column `1`.
    pub fn start() -> Self {
        Self {
            index: 0,
            line: 1,
            column: 1,
        }
    }
}

/// Can represent either a concrete location, or the end of a source code string.
pub enum GeneralLocation {
    /// A concrete location in the source code.
    Location(Location),
    /// The end of the source code string.
    EndOfFile,
}

impl Display for GeneralLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GeneralLocation::Location(location) => write!(f, "{}", location),
            GeneralLocation::EndOfFile => write!(f, "[end of file]"),
        }
    }
}

/// A wrapper around the source code string.
pub struct Source {
    /// The source code string.
    text: Vec<char>,
    /// The location of the next character.
    location: Location,
}

impl Source {
    /// Creates a new [Source] from a string input.
    pub fn new(text: &str) -> Self {
        Self {
            text: text.chars().collect(),
            location: Location::start(),
        }
    }

    /// Returns the next character in the string, without advancing the position.
    pub fn peek(&self) -> Option<char> {
        self.text.get(self.location.index).copied()
    }

    /// Returns the (next + 1)th character in the string, without advancing the position.
    pub fn peek_after(&self) -> Option<char> {
        self.text.get(self.location.index + 1).copied()
    }

    /// Returns the next character in the string, and advances the position.
    pub fn advance(&mut self) -> Option<char> {
        let next = self.peek();

        if let Some(character) = next {
            self.location.index += 1;
            self.location.column += 1;

            if character == '\n' {
                self.location.line += 1;
                self.location.column = 1;
            }
        }

        next
    }

    /// Conditionally advances the position if the next character matches a target.
    ///
    /// If the next character is equal to `target`, then the position is advanced, and `true` is returned. Otherwise, the position is not advanced, and `false` is returned.
    pub fn matches(&mut self, target: char) -> bool {
        if let Some(character) = self.peek() {
            if character == target {
                self.advance();
            }

            return character == target;
        }

        false
    }

    /// Returns whether the source code string has been consumed fully.
    pub fn at_end(&self) -> bool {
        self.location.index >= self.text.len()
    }

    /// Returns the location of the next character.
    pub fn location(&self) -> Location {
        self.location
    }
}

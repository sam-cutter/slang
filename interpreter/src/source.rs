pub struct Location {
    pub index: usize,
    pub line: usize,
    pub column: usize,
}

pub struct Source {
    text: Vec<char>,
    location: Location,
}

impl Source {
    pub fn new(text: &str) -> Self {
        Self {
            text: text.chars().collect(),
            location: Location {
                index: 0,
                line: 0,
                column: 0,
            },
        }
    }

    pub fn peek(&self) -> Option<char> {
        self.text.get(self.location.index).copied()
    }

    pub fn advance(&mut self) -> Option<char> {
        let next = self.peek();

        if let Some(character) = next {
            self.location.index += 1;
            self.location.column += 1;

            if character == '\n' {
                self.location.line += 1;
                self.location.column = 0;
            }
        }

        next
    }

    pub fn matches(&mut self, target: char) -> Option<bool> {
        if let Some(character) = self.peek() {
            if character == target {
                self.advance();
            }

            return Some(character == target);
        }

        None
    }
}

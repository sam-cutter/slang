#[derive(Clone, Copy, Debug)]
pub struct Location {
    pub index: usize,
    pub line: usize,
    pub column: usize,
}

impl Location {
    pub fn start() -> Self {
        Self {
            index: 0,
            line: 0,
            column: 0,
        }
    }
}

pub struct Source {
    text: Vec<char>,
    location: Location,
    // TODO: think about whether there's a better way to do this
    previous_location: Location,
}

impl Source {
    pub fn new(text: &str) -> Self {
        Self {
            text: text.chars().collect(),
            location: Location::start(),
            previous_location: Location::start(),
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
            self.previous_location = self.location;

            self.location.index += 1;
            self.location.column += 1;

            if character == '\n' {
                self.location.line += 1;
                self.location.column = 0;
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

    pub fn location(&self) -> Location {
        self.location
    }

    pub fn previous_location(&self) -> Location {
        self.previous_location
    }

    pub fn at_end(&self) -> bool {
        self.location.index >= self.text.len()
    }
}

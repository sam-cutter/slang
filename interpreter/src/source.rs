pub struct Source {
    text: Vec<char>,
    current: usize,
    line: usize,
    column: usize,
}

impl Source {
    pub fn new(text: &str) -> Self {
        Self {
            text: text.chars().collect(),
            current: 0,
            line: 0,
            column: 0,
        }
    }

    pub fn peek(&self) -> Option<char> {
        self.text.get(self.current).copied()
    }

    pub fn advance(&mut self) -> Option<char> {
        let next = self.peek();

        if let Some(character) = next {
            self.current += 1;
            self.column += 1;

            if character == '\n' {
                self.line += 1;
                self.column = 0;
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

use super::types::{Span, Range};

pub struct SpanManager {
    pub col: usize,
    pub row: usize,
    pub idx: usize
}

impl SpanManager {
    pub fn new() -> Self {
        SpanManager {
            col: 1,
            row: 1,
            idx: 0,
        }
    }

    pub fn state(&self) -> Span {
        Span {
            col: self.col,
            row: self.row,
            len: 0,
            range: Range {
                start: self.idx,
                end: self.idx
            }
        }
    }

    pub fn waste(&mut self, char: char) {
        self.idx += 1;
        
        match char {
            '\n' => {
                self.col = 0;
                self.row += 1;
            },
            _ => {
                self.col += 1;
            }
        }
    }

    pub fn waste_slice(&mut self, slice: Vec<char>) {
        for char in slice {
            self.waste(char)
        }
    }

    pub fn consume(&mut self, slice: Vec<char>) -> Span {
        let mut span = self.state();
        let mut len = 0;

        for char in slice {
            self.idx += 1;
            len += 1;

            match char {
                '\n' => {
                    self.col = 0;
                    self.row += 1;
                },
                _ => {
                    self.col += 1;
                }
            }
        }

        span.range = Range { start: span.range.start, end: span.range.start + len };
        span.len = len;
        span
    }
}

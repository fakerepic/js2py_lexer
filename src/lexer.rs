use crate::statefn::StateFn;
use crate::token::{Item, Type};

/// Analyze the input string and return a vector of tokens
/// * `input`: The input string to analyze
pub fn analyze(input: &str) -> Vec<Item> {
    let mut l = Lexer {
        input: input.to_string(),
        start: 0,
        pos: 0,
        items: Vec::new(),
        fn_start: StateFn::default(),
    };
    l.run();
    l.items.clone()
}

/// Lexer context
pub struct Lexer {
    input: String,
    start: usize,
    pos: usize,
    items: Vec<Item>,
    fn_start: StateFn,
}

impl Lexer {
    pub fn run(&mut self) {
        // Run the state machine
        let mut f = self.fn_start.clone();
        while let Some(next_f) = f.call(self) {
            f = next_f;
        }
    }
    pub fn current(&self) -> String {
        self.input[self.start..self.pos].to_string()
    }
    pub fn emit(&mut self, typ: Type) {
        let value = self.input[self.start..self.pos].to_string();
        self.items.push(Item { typ, val: value });
        self.start = self.pos;
    }
    pub fn set_input(&mut self, input: &str) {
        self.input = input.to_string();
        self.start = 0;
        self.pos = 0;
        self.items.clear();
    }

    pub fn ignore(&mut self) {
        self.start = self.pos;
    }
    pub fn peek(&self) -> Option<char> {
        self.input[self.pos..].chars().next()
    }
    pub fn step(&mut self) -> Option<char> {
        self.pos += self.peek()?.len_utf8();
        self.peek()
    }
    pub fn accept(&mut self, valid: &str) -> bool {
        if let Some(c) = self.peek() {
            if valid.contains(c) {
                self.step();
                return true;
            }
        }
        false
    }
    pub fn accept_run(&mut self, valid: &str) -> bool {
        let mut accepted = false;
        while self.accept(valid) {
            accepted = true;
        }
        accepted
    }
    pub fn eof(&self) -> bool {
        self.pos >= self.input.len()
    }
}
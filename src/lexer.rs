use crate::statefn::StateFn;
use crate::token::{Token, Type};
use std::sync::{mpsc, Arc};

/// Create a token stream from the input string
///
/// * `input`: the input string to tokenize
pub fn token_stream(input: &str) -> mpsc::Receiver<Token> {
    let (tx, rx) = mpsc::channel();
    let mut l = Lexer {
        input: Arc::from(input),
        start: 0,
        pos: 0,
        initial_state: StateFn::default(),
        sender: tx,
    };
    std::thread::spawn(move || l.run());
    rx
}

/// Lexer context
pub struct Lexer {
    input: Arc<str>,
    start: usize,
    pos: usize,
    initial_state: StateFn,
    sender: mpsc::Sender<Token>,
    // error: Option<String>,
}

impl Lexer {
    pub fn run(&mut self) {
        let mut f = self.initial_state.clone();

        // Run the state functions until there are no more state functions to run
        // This pattern can decouple the lexer from the state functions and make it easier to
        // extend the lexer with new state functions.
        while let Some(next_f) = f.call(self) {
            f = next_f;
        }
    }

    // The following methods are used by the state functions to interact with the lexer context:

    pub fn current(&self) -> String {
        self.input[self.start..self.pos].to_string()
    }
    /// Emit a token with the current value
    pub fn emit(&mut self, typ: Type) {
        self.send(typ);
        self.start = self.pos;
    }
    /// Send a token (without updating the start position)
    pub fn send(&mut self, typ: Type) {
        let _ = self.sender.send(Token {
            typ,
            start: self.start,
            end: self.pos,
        });
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
    pub fn back(&mut self) {
        if self.pos > 0 {
            self.pos -= 1;
        }
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
    pub fn accept_except(&mut self, invalid: &str) -> bool {
        if let Some(c) = self.peek() {
            if !invalid.contains(c) {
                self.step();
                return true;
            }
        }
        false
    }
    pub fn accept_run_except(&mut self, invalid: &str) -> bool {
        let mut accepted = false;
        while self.accept_except(invalid) {
            accepted = true;
        }
        accepted
    }
    pub fn eof(&self) -> bool {
        self.pos >= self.input.len()
    }
}

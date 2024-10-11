use crate::statefn::StateFn;
use crate::token::{Token, Type};
use std::sync::mpsc;

pub fn token_stream(input: &str) -> mpsc::Receiver<Token> {
    let (tx, rx) = mpsc::channel();
    let mut l = Lexer {
        input: input.to_string(),
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
    input: String,
    start: usize,
    pos: usize,
    initial_state: StateFn,
    sender: mpsc::Sender<Token>,
}

impl Lexer {
    pub fn run(&mut self) {
        // Run the state machine
        let mut f = self.initial_state.clone();
        while let Some(next_f) = f.call(self) {
            f = next_f;
        }
    }
    pub fn current(&self) -> String {
        self.input[self.start..self.pos].to_string()
    }
    pub fn emit(&mut self, typ: Type) {
        let val = self.input[self.start..self.pos].to_string();
        let _ = self.sender.send(Token { typ, val });
        self.start = self.pos;
    }
    pub fn set_input(&mut self, input: &str) {
        self.input = input.to_string();
        self.start = 0;
        self.pos = 0;
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

use crate::lexer::Lexer;

#[derive(Clone)]
pub struct StateFn {
    pub f: fn(&mut Lexer) -> Option<StateFn>,
}

impl StateFn {
    pub fn from(f: fn(&mut Lexer) -> Option<StateFn>) -> StateFn {
        StateFn { f }
    }
    pub fn call(&self, lexer: &mut Lexer) -> Option<StateFn> {
        (self.f)(lexer)
    }
}

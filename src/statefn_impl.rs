use crate::lexer::Lexer;
pub use crate::statefn::StateFn;
use crate::token::Type;

// TODO: error handling

impl Default for StateFn {
    fn default() -> StateFn {
        StateFn { f: lex_start }
    }
}

pub fn lex_start(lexer: &mut Lexer) -> Option<StateFn> {
    if lexer.eof() {
        return None;
    }

    let c = lexer.peek();

    // trivial cases:
    if let Some(next_f) = match c.unwrap() {
        ';' => Some(Type::SemiColon),
        '(' => Some(Type::LeftParen),
        ')' => Some(Type::RightParen),
        '{' => Some(Type::LeftBrace),
        '}' => Some(Type::RightBrace),
        '+' => Some(Type::Plus),
        '*' => Some(Type::Multiply),
        '/' => Some(Type::Divide),
        '>' => Some(Type::Greater),
        '<' => Some(Type::Less),
        _ => None,
    } {
        lexer.step();
        lexer.emit(next_f);
        return Some(StateFn::from(lex_start));
    }

    match c.unwrap() {
        'a'..='z' | 'A'..='Z' => return Some(StateFn::from(lex_alpha)),
        '0'..='9' => return Some(StateFn::from(lex_number)),
        '"' => return Some(StateFn::from(lex_string_literal)),
        '-' => return Some(StateFn::from(lex_minus)), // '-' or number
        '=' => return Some(StateFn::from(lex_eq_op)), // '=' or '=='

        // all blank characters
        ' ' | '\t' | '\n' | '\r' => {
            lexer.step();
            lexer.ignore();
        }

        _ => {
            // println!("Error: unexpected character '{}'", lexer.peek().unwrap());
            lexer.step();
        }
    }

    Some(StateFn::from(lex_start))
}

fn lex_eq_op(lexer: &mut Lexer) -> Option<StateFn> {
    lexer.accept_run("=");
    let s = lexer.current();
    match s.as_str() {
        "=" => lexer.emit(Type::Assign),
        "==" => lexer.emit(Type::Equal),
        _ => unreachable!(),
    }
    Some(StateFn::from(lex_start))
}

fn lex_minus(lexer: &mut Lexer) -> Option<StateFn> {
    lexer.step();
    match lexer.peek() {
        Some('0'..='9') => Some(StateFn::from(lex_number)),
        _ => {
            lexer.emit(Type::Minus);
            Some(StateFn::from(lex_start))
        }
    }
}

fn lex_number(lexer: &mut Lexer) -> Option<StateFn> {
    while let Some('0'..='9') = lexer.peek() {
        lexer.step();
    }
    lexer.emit(Type::Number);
    Some(StateFn::from(lex_start))
}

static SPECIAL_CHARS: &str = " !#$%&'()*+,-./:;<=>?@[]^_{}|~";
static DIGITS_AND_ALPHA: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";

fn lex_string_literal(lexer: &mut Lexer) -> Option<StateFn> {
    lexer.step();
    while let Some(c) = lexer.peek() {
        if c == '"' {
            lexer.step();
            lexer.emit(Type::StringLiteral);
            return Some(StateFn::from(lex_start));
        }
        if !SPECIAL_CHARS.contains(c) && !DIGITS_AND_ALPHA.contains(c) {
            // println!("Error: unexpected character '{}'", c);
            return None;
        }
        lexer.step();
    }
    None
}

fn lex_alpha(lexer: &mut Lexer) -> Option<StateFn> {
    lexer.accept_run(DIGITS_AND_ALPHA);
    let s = lexer.current();
    match s.as_str() {
        "var" => lexer.emit(Type::Var),
        "input" => lexer.emit(Type::Input),
        "if" => lexer.emit(Type::If),
        "print" => lexer.emit(Type::Print),
        "while" => lexer.emit(Type::While),
        _ => lexer.emit(Type::Identifier),
    }
    Some(StateFn::from(lex_start))
}

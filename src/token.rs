#[derive(Debug, Clone)]
pub struct Token {
    pub typ: Type,
    pub val: String,
}

#[derive(Debug, Clone)]
pub enum Type {
    LeftBrace,
    RightBrace,
    LeftParen,
    RightParen,
    SemiColon,
    Plus,
    Multiply,
    Divide,
    Greater,
    Less,

    Assign,
    Equal,

    Minus,

    Input,
    Print,
    Var,
    If,
    While,

    Identifier,
    Number,
    StringLiteral,

    LexerError
}

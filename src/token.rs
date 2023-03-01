#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    Def,
    If,
    Else,
    Return,
    Identifier,
    Int,
    Plus,
    Minus,
    Mul,
    Div,
    Colon,
    Comma,
    Assign,
    LessThan,
    // GreaterThan,
    Indent,
    Dedent,

    LParen,
    RParen,

    NewLine,
    EndMarker,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub value: String,
}

impl Token {
    pub fn new(token_type: TokenType, value: String) -> Self {
        Self { token_type, value }
    }
}

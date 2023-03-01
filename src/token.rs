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

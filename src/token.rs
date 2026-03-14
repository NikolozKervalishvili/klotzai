use std::borrow::Cow;

#[derive(Debug)]
pub struct Token<'a> {
    value: Cow<'a, str>,
    // token pos
    pos: usize,
    _type: TokenType,
}

impl<'a> Token<'a> {
    pub fn new(value: Cow<'a, str>, pos: usize, _type: TokenType) -> Self {
        Self { value, pos, _type }
    }
}

#[derive(Debug)]
pub enum TokenType {
    KeyWord,
    Identifier,
    Integer,
    Float,
    String,
    Operator,
    Delimiter,
}

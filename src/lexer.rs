use crate::token::{Token, TokenType};
use std::{
    borrow::Cow,
    collections::HashSet,
    iter::{self, Peekable},
    str::Chars,
    sync::LazyLock,
};

static KEYWORDS: LazyLock<HashSet<&str>> = LazyLock::new(|| {
    HashSet::from([
        "def", "if", "else", "while", "nil", "true", "false", "let", "return",
    ])
});

type Result<T> = std::result::Result<T, LexErr>;

#[derive(Debug)]
pub struct Lexer<'a> {
    source: &'a str,
    stream: Peekable<Chars<'a>>,
    // equals byte pos, utf-16 chars can contain multiple bytes e.g.
    raw_pos: usize,
    token_pos: usize,
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Result<Token<'a>>;
    fn next(&mut self) -> Option<Self::Item> {
        // just consume whitespaces
        self.consume_while(|c| c.is_ascii_whitespace())
            .for_each(drop);
        if let Some(&c) = self.stream.peek() {
            let token = Some(match c {
                // identifier: variable or keyword in this case
                c if c.is_ascii_alphabetic() => self.identifier(),
                c if c.is_ascii_digit() => self.number(),
                '.' | '+' | '-' | '*' | '/' | '!' | '=' | '&' | '|' | '%' | '<' | '>' => {
                    self.operator()
                }
                '"' => self.string(),
                // unrecogized character encapsulated
                _ => {
                    self.consume();
                    Err(LexErr::UnrecognizedChar {
                        raw_pos: self.raw_pos,
                    })
                }
            });
            self.token_pos += 1;
            return token;
        }
        None
    }
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            stream: source.chars().peekable(),
            raw_pos: 0,
            token_pos: 0,
        }
    }

    fn operator(&mut self) -> Result<Token<'a>> {
        let start = self.raw_pos;
        // consume peeked operator char and match second
        // WARNING: this code makes hard to allow multiple chars as second
        let second = match self.consume().unwrap() {
            '=' | '<' | '>' | '!' => Some(&'='),
            '&' => Some(&'&'),
            '|' => Some(&'|'),
            '?' => Some(&'.'),
            _ => None,
        };

        if self.stream.peek() == second {
            self.consume();
        }

        Ok(Token::new(
            Cow::Borrowed(&self.source[start..self.raw_pos]),
            self.token_pos,
            TokenType::Operator,
        ))
    }

    fn string(&mut self) -> Result<Token<'a>> {
        let start = self.raw_pos;
        // consume opening "
        self.consume();
        // inner content of string
        self.consume_while(|c| c != '"').for_each(drop);
        match self.stream.peek() {
            Some('"') => self.consume(),
            _ => {
                return Err(LexErr::UnclosedDelim {
                    raw_pos: self.raw_pos,
                });
            }
        };
        Ok(Token::new(
            Cow::Borrowed(&self.source[start..self.raw_pos]),
            self.token_pos,
            TokenType::String,
        ))
    }

    fn identifier(&mut self) -> Result<Token<'a>> {
        let start = self.raw_pos;
        self.consume_while(|c| c.is_ascii_alphanumeric() || c == '_')
            .for_each(drop);
        Ok(Token::new(
            Cow::Borrowed(&self.source[start..self.raw_pos]),
            self.token_pos,
            if KEYWORDS.contains(&self.source[start..self.raw_pos]) {
                TokenType::KeyWord
            } else {
                TokenType::Identifier
            },
        ))
    }

    // either integer or float
    fn number(&mut self) -> Result<Token<'a>> {
        let start = self.raw_pos;
        let token_type = match self
            .consume_while(|c| c.is_ascii_digit() || c == '_' || c == '.')
            .filter(|&c| c == '.')
            .count()
        {
            0 => TokenType::Integer,
            1 => TokenType::Float,
            _ => {
                return Err(LexErr::MoreThanOneDecimal {
                    token_pos: self.token_pos,
                });
            }
        };

        Ok(Token::new(
            Cow::Borrowed(&self.source[start..self.raw_pos]),
            self.token_pos,
            token_type,
        ))
    }

    fn consume(&mut self) -> Option<char> {
        self.stream
            .next()
            .inspect(|&c| self.raw_pos += c.len_utf8())
    }

    // this is lazy! because it returns an Iterator
    fn consume_while<F>(&mut self, mut f: F) -> impl Iterator<Item = char>
    where
        F: FnMut(char) -> bool,
    {
        iter::from_fn(move || {
            if let Some(&c) = self.stream.peek()
                && f(c)
            {
                return self.consume();
            }
            None
        })
    }
}

#[derive(Debug)]
pub enum LexErr {
    UnrecognizedChar { raw_pos: usize },
    UnclosedDelim { raw_pos: usize },
    MoreThanOneDecimal { token_pos: usize },
}

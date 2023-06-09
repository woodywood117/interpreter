#![allow(dead_code)]

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub ttype: TokenType,
    pub literal: String,
}

impl Token {
    pub fn new(ttype: TokenType, literal: String) -> Token {
        Token { ttype, literal }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    // Keywords
    Let,
    Fn,
    True,
    False,
    If,
    Else,
    Return,

    // Identifiers and literals
    Identifier,
    Integer,
    String,

    // Operators
    Plus,
    Increment,
    Minus,
    Decrement,
    Asterisk,
    Slash,
    Question,
    Percent,
    Assign,
    Bang,
    Equal,
    NotEqual,
    LessThan,
    GreaterThan,
    LessThanOrEqual,
    GreaterThanOrEqual,

    // Delimiters
    Comma,
    Semicolon,
    Colon,
    LeftParen,
    RightParen,
    LeftSquareBracket,
    RightSquareBracket,
    LeftCurlyBracket,
    RightCurlyBracket,

    // End of file
    Eof,
    Illegal,
}
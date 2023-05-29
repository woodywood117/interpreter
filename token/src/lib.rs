#![allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum Token {
    // Keywords
    Let,
    Fn,
    True,
    False,
    If,
    Else,
    Return,

    // Identifiers and literals
    Identifier(String),
    Integer(String),
    String(String),

    // Operators
    Plus,
    Minus,
    Asterisk,
    Slash,
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
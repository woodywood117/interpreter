use token;

pub struct Lexer {
    input: Vec<char>,
    position: usize,
    read_position: usize,
    ch: char,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let mut l = Lexer {
            input: input.chars().collect(),
            position: 0,
            read_position: 0,
            ch: '\0',
        };
        l.read_char();
        l
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '\0';
        } else {
            self.ch = self.input[self.read_position];
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    #[allow(dead_code)]
    fn peek(&self) -> char {
        if self.read_position >= self.input.len() {
            return '\0';
        }
        self.input[self.read_position]
    }
}

impl Iterator for Lexer {
    type Item = token::Token;

    fn next(&mut self) -> Option<Self::Item> {
        // Skip whitespace
        while self.ch.is_whitespace() {
            self.read_char();
        }

        // If the read head is past the end of the input plus the eof char, return None
        if self.read_position > self.input.len() + 1 {
            return None;
        }

        let token = match self.ch {
            '+' => token::Token::Plus,
            '-' => token::Token::Minus,
            '*' => token::Token::Asterisk,
            '/' => token::Token::Slash,
            '=' => {
                if self.peek() == '=' {
                    self.read_char();
                    token::Token::Equal
                } else {
                    token::Token::Assign
                }},
            '!' => {
                if self.peek() == '=' {
                    self.read_char();
                    token::Token::NotEqual
                } else {
                    token::Token::Bang
                }},
            '<' => {
                if self.peek() == '=' {
                    self.read_char();
                    token::Token::LessThanOrEqual
                } else {
                    token::Token::LessThan
                }},
            '>' => {
                if self.peek() == '=' {
                    self.read_char();
                    token::Token::GreaterThanOrEqual
                } else {
                    token::Token::GreaterThan
                }},
            ',' => token::Token::Comma,
            ';' => token::Token::Semicolon,
            ':' => token::Token::Colon,
            '(' => token::Token::LeftParen,
            ')' => token::Token::RightParen,
            '[' => token::Token::LeftSquareBracket,
            ']' => token::Token::RightSquareBracket,
            '{' => token::Token::LeftCurlyBracket,
            '}' => token::Token::RightCurlyBracket,
            '\0' => token::Token::Eof,
            'a'..='z'|'A'..='Z'|'_' => {
                let mut ident = String::new();
                while self.ch.is_alphabetic() || self.ch == '_' {
                    ident.push(self.ch);
                    self.read_char();
                }
                let token = match ident.as_str() {
                    "let" => token::Token::Let,
                    "fn" => token::Token::Fn,
                    "true" => token::Token::True,
                    "false" => token::Token::False,
                    "if" => token::Token::If,
                    "else" => token::Token::Else,
                    "return" => token::Token::Return,
                    _ => token::Token::Identifier(ident),
                };
                return Some(token);
            }
            '0'..='9' => {
                let mut number = String::new();
                while self.ch.is_digit(10) {
                    number.push(self.ch);
                    self.read_char();
                }
                return Some(token::Token::Integer(number));
            }
            '"' => {
                let mut string = String::new();
                string.push(self.ch);
                self.read_char();
                while self.ch != '"' {
                    if self.ch == '\0' || self.ch == '\n' {
                        return Some(token::Token::Illegal);
                    }
                    string.push(self.ch);
                    self.read_char();
                }
                string.push(self.ch);
                self.read_char();
                return Some(token::Token::String(string));
            }
            _ => token::Token::Illegal,
        };

        self.read_char();
        Some(token)
    }
}

#[test]
fn test_lexer_delimiters() {
    let l = Lexer::new(String::from("+-*/=,;:()[]{}"));
    let tokens = l.collect::<Vec<token::Token>>();

    assert_eq!(tokens[0], token::Token::Plus);
    assert_eq!(tokens[1], token::Token::Minus);
    assert_eq!(tokens[2], token::Token::Asterisk);
    assert_eq!(tokens[3], token::Token::Slash);
    assert_eq!(tokens[4], token::Token::Assign);
    assert_eq!(tokens[5], token::Token::Comma);
    assert_eq!(tokens[6], token::Token::Semicolon);
    assert_eq!(tokens[7], token::Token::Colon);
    assert_eq!(tokens[8], token::Token::LeftParen);
    assert_eq!(tokens[9], token::Token::RightParen);
    assert_eq!(tokens[10], token::Token::LeftSquareBracket);
    assert_eq!(tokens[11], token::Token::RightSquareBracket);
    assert_eq!(tokens[12], token::Token::LeftCurlyBracket);
    assert_eq!(tokens[13], token::Token::RightCurlyBracket);
    assert_eq!(tokens[14], token::Token::Eof);
}

#[test]
fn test_next_token() {
    let input = String::from(
        r#"let five = 5;
        let ten = 10;
        
        let add = fn(x, y) {
          x + y;
        };
        
        let result = add(five, ten);
        !-/*5;
        5 < 10 > 5;
        
        if (5 < 10) {
            return true;
        } else {
            return false;
        }
        
        10 == 10;
        10 != 9;
        10 >= 9;
        9 <= 10;
        "te st" != "test";
        "#);

    let mut l = Lexer::new(input);

    assert_eq!(l.next().unwrap(), token::Token::Let);
    assert_eq!(l.next().unwrap(), token::Token::Identifier(String::from("five")));
    assert_eq!(l.next().unwrap(), token::Token::Assign);
    assert_eq!(l.next().unwrap(), token::Token::Integer(String::from("5")));
    assert_eq!(l.next().unwrap(), token::Token::Semicolon);
    assert_eq!(l.next().unwrap(), token::Token::Let);
    assert_eq!(l.next().unwrap(), token::Token::Identifier(String::from("ten")));
    assert_eq!(l.next().unwrap(), token::Token::Assign);
    assert_eq!(l.next().unwrap(), token::Token::Integer(String::from("10")));
    assert_eq!(l.next().unwrap(), token::Token::Semicolon);
    assert_eq!(l.next().unwrap(), token::Token::Let);
    assert_eq!(l.next().unwrap(), token::Token::Identifier(String::from("add")));
    assert_eq!(l.next().unwrap(), token::Token::Assign);
    assert_eq!(l.next().unwrap(), token::Token::Fn);
    assert_eq!(l.next().unwrap(), token::Token::LeftParen);
    assert_eq!(l.next().unwrap(), token::Token::Identifier(String::from("x")));
    assert_eq!(l.next().unwrap(), token::Token::Comma);
    assert_eq!(l.next().unwrap(), token::Token::Identifier(String::from("y")));
    assert_eq!(l.next().unwrap(), token::Token::RightParen);
    assert_eq!(l.next().unwrap(), token::Token::LeftCurlyBracket);
    assert_eq!(l.next().unwrap(), token::Token::Identifier(String::from("x")));
    assert_eq!(l.next().unwrap(), token::Token::Plus);
    assert_eq!(l.next().unwrap(), token::Token::Identifier(String::from("y")));
    assert_eq!(l.next().unwrap(), token::Token::Semicolon);
    assert_eq!(l.next().unwrap(), token::Token::RightCurlyBracket);
    assert_eq!(l.next().unwrap(), token::Token::Semicolon);
    assert_eq!(l.next().unwrap(), token::Token::Let);
    assert_eq!(l.next().unwrap(), token::Token::Identifier(String::from("result")));
    assert_eq!(l.next().unwrap(), token::Token::Assign);
    assert_eq!(l.next().unwrap(), token::Token::Identifier(String::from("add")));
    assert_eq!(l.next().unwrap(), token::Token::LeftParen);
    assert_eq!(l.next().unwrap(), token::Token::Identifier(String::from("five")));
    assert_eq!(l.next().unwrap(), token::Token::Comma);
    assert_eq!(l.next().unwrap(), token::Token::Identifier(String::from("ten")));
    assert_eq!(l.next().unwrap(), token::Token::RightParen);
    assert_eq!(l.next().unwrap(), token::Token::Semicolon);
    assert_eq!(l.next().unwrap(), token::Token::Bang);
    assert_eq!(l.next().unwrap(), token::Token::Minus);
    assert_eq!(l.next().unwrap(), token::Token::Slash);
    assert_eq!(l.next().unwrap(), token::Token::Asterisk);
    assert_eq!(l.next().unwrap(), token::Token::Integer(String::from("5")));
    assert_eq!(l.next().unwrap(), token::Token::Semicolon);
    assert_eq!(l.next().unwrap(), token::Token::Integer(String::from("5")));
    assert_eq!(l.next().unwrap(), token::Token::LessThan);
    assert_eq!(l.next().unwrap(), token::Token::Integer(String::from("10")));
    assert_eq!(l.next().unwrap(), token::Token::GreaterThan);
    assert_eq!(l.next().unwrap(), token::Token::Integer(String::from("5")));
    assert_eq!(l.next().unwrap(), token::Token::Semicolon);
    assert_eq!(l.next().unwrap(), token::Token::If);
    assert_eq!(l.next().unwrap(), token::Token::LeftParen);
    assert_eq!(l.next().unwrap(), token::Token::Integer(String::from("5")));
    assert_eq!(l.next().unwrap(), token::Token::LessThan);
    assert_eq!(l.next().unwrap(), token::Token::Integer(String::from("10")));
    assert_eq!(l.next().unwrap(), token::Token::RightParen);
    assert_eq!(l.next().unwrap(), token::Token::LeftCurlyBracket);
    assert_eq!(l.next().unwrap(), token::Token::Return);
    assert_eq!(l.next().unwrap(), token::Token::True);
    assert_eq!(l.next().unwrap(), token::Token::Semicolon);
    assert_eq!(l.next().unwrap(), token::Token::RightCurlyBracket);
    assert_eq!(l.next().unwrap(), token::Token::Else);
    assert_eq!(l.next().unwrap(), token::Token::LeftCurlyBracket);
    assert_eq!(l.next().unwrap(), token::Token::Return);
    assert_eq!(l.next().unwrap(), token::Token::False);
    assert_eq!(l.next().unwrap(), token::Token::Semicolon);
    assert_eq!(l.next().unwrap(), token::Token::RightCurlyBracket);
    assert_eq!(l.next().unwrap(), token::Token::Integer(String::from("10")));
    assert_eq!(l.next().unwrap(), token::Token::Equal);
    assert_eq!(l.next().unwrap(), token::Token::Integer(String::from("10")));
    assert_eq!(l.next().unwrap(), token::Token::Semicolon);
    assert_eq!(l.next().unwrap(), token::Token::Integer(String::from("10")));
    assert_eq!(l.next().unwrap(), token::Token::NotEqual);
    assert_eq!(l.next().unwrap(), token::Token::Integer(String::from("9")));
    assert_eq!(l.next().unwrap(), token::Token::Semicolon);
    assert_eq!(l.next().unwrap(), token::Token::Integer(String::from("10")));
    assert_eq!(l.next().unwrap(), token::Token::GreaterThanOrEqual);
    assert_eq!(l.next().unwrap(), token::Token::Integer(String::from("9")));
    assert_eq!(l.next().unwrap(), token::Token::Semicolon);
    assert_eq!(l.next().unwrap(), token::Token::Integer(String::from("9")));
    assert_eq!(l.next().unwrap(), token::Token::LessThanOrEqual);
    assert_eq!(l.next().unwrap(), token::Token::Integer(String::from("10")));
    assert_eq!(l.next().unwrap(), token::Token::Semicolon);
    assert_eq!(l.next().unwrap(), token::Token::String(String::from("\"te st\"")));
    assert_eq!(l.next().unwrap(), token::Token::NotEqual);
    assert_eq!(l.next().unwrap(), token::Token::String(String::from("\"test\"")));
    assert_eq!(l.next().unwrap(), token::Token::Semicolon);
    assert_eq!(l.next().unwrap(), token::Token::Eof);
}
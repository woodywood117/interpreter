use token::{Token, TokenType};

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
    type Item = Token;

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
            '+' => {
                if self.peek() == '+' {
                    self.read_char();
                    Token::new(TokenType::Increment, "++".to_string())
                } else {
                    Token::new(TokenType::Plus, self.ch.to_string())
                }
            }
            '-' => {
                if self.peek() == '-' {
                    self.read_char();
                    Token::new(TokenType::Decrement, "--".to_string())
                } else {
                    Token::new(TokenType::Minus, self.ch.to_string())
                }
            }
            '*' => Token::new(TokenType::Asterisk, self.ch.to_string()),
            '/' => Token::new(TokenType::Slash, self.ch.to_string()),
            '?' => Token::new(TokenType::Question, self.ch.to_string()),
            '%' => Token::new(TokenType::Percent, self.ch.to_string()),
            '=' => {
                if self.peek() == '=' {
                    self.read_char();
                    Token::new(TokenType::Equal, "==".to_string())
                } else {
                    Token::new(TokenType::Assign, self.ch.to_string())
                }},
            '!' => {
                if self.peek() == '=' {
                    self.read_char();
                    Token::new(TokenType::NotEqual, "!=".to_string())
                } else {
                    Token::new(TokenType::Bang, self.ch.to_string())
                }},
            '<' => {
                if self.peek() == '=' {
                    self.read_char();
                    Token::new(TokenType::LessThanOrEqual, "<=".to_string())
                } else {
                    Token::new(TokenType::LessThan, self.ch.to_string())
                }},
            '>' => {
                if self.peek() == '=' {
                    self.read_char();
                    Token::new(TokenType::GreaterThanOrEqual, ">=".to_string())
                } else {
                    Token::new(TokenType::GreaterThan, self.ch.to_string())
                }},
            ',' => Token::new(TokenType::Comma, self.ch.to_string()),
            ';' => Token::new(TokenType::Semicolon, self.ch.to_string()),
            ':' => Token::new(TokenType::Colon, self.ch.to_string()),
            '(' => Token::new(TokenType::LeftParen, self.ch.to_string()),
            ')' => Token::new(TokenType::RightParen, self.ch.to_string()),
            '[' => Token::new(TokenType::LeftSquareBracket, self.ch.to_string()),
            ']' => Token::new(TokenType::RightSquareBracket, self.ch.to_string()),
            '{' => Token::new(TokenType::LeftCurlyBracket, self.ch.to_string()),
            '}' => Token::new(TokenType::RightCurlyBracket, self.ch.to_string()),
            '\0' => Token::new(TokenType::Eof, self.ch.to_string()),
            'a'..='z'|'A'..='Z'|'_' => {
                let mut ident = String::new();
                while self.ch.is_alphabetic() || self.ch == '_' {
                    ident.push(self.ch);
                    self.read_char();
                }
                let token = match ident.as_str() {
                    "let" => Token::new(TokenType::Let, ident),
                    "fn" => Token::new(TokenType::Fn, ident),
                    "true" => Token::new(TokenType::True, ident),
                    "false" => Token::new(TokenType::False, ident),
                    "if" => Token::new(TokenType::If, ident),
                    "else" => Token::new(TokenType::Else, ident),
                    "return" => Token::new(TokenType::Return, ident),
                    _ => Token::new(TokenType::Identifier, ident)
                };
                return Some(token);
            }
            '0'..='9' => {
                let mut number = String::new();
                while self.ch.is_digit(10) {
                    number.push(self.ch);
                    self.read_char();
                }
                return Some(Token::new(TokenType::Integer, number));
            }
            '"' => {
                let mut string = String::new();
                string.push(self.ch);
                self.read_char();
                while self.ch != '"' {
                    if self.ch == '\0' || self.ch == '\n' {
                        return Some(Token::new(TokenType::Illegal, string));
                    }
                    string.push(self.ch);
                    self.read_char();
                }
                string.push(self.ch);
                self.read_char();
                return Some(Token::new(TokenType::String, string));
            }
            _ => Token::new(TokenType::Illegal, self.ch.to_string())
        };

        self.read_char();
        Some(token)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer_delimiters() {
        let mut l = Lexer::new(String::from("+-*/=,;:()[]{}++--?%"));

        assert_eq!(l.next().unwrap().ttype, TokenType::Plus);
        assert_eq!(l.next().unwrap().ttype, TokenType::Minus);
        assert_eq!(l.next().unwrap().ttype, TokenType::Asterisk);
        assert_eq!(l.next().unwrap().ttype, TokenType::Slash);
        assert_eq!(l.next().unwrap().ttype, TokenType::Assign);
        assert_eq!(l.next().unwrap().ttype, TokenType::Comma);
        assert_eq!(l.next().unwrap().ttype, TokenType::Semicolon);
        assert_eq!(l.next().unwrap().ttype, TokenType::Colon);
        assert_eq!(l.next().unwrap().ttype, TokenType::LeftParen);
        assert_eq!(l.next().unwrap().ttype, TokenType::RightParen);
        assert_eq!(l.next().unwrap().ttype, TokenType::LeftSquareBracket);
        assert_eq!(l.next().unwrap().ttype, TokenType::RightSquareBracket);
        assert_eq!(l.next().unwrap().ttype, TokenType::LeftCurlyBracket);
        assert_eq!(l.next().unwrap().ttype, TokenType::RightCurlyBracket);
        assert_eq!(l.next().unwrap().ttype, TokenType::Increment);
        assert_eq!(l.next().unwrap().ttype, TokenType::Decrement);
        assert_eq!(l.next().unwrap().ttype, TokenType::Question);
        assert_eq!(l.next().unwrap().ttype, TokenType::Percent);
        assert_eq!(l.next().unwrap().ttype, TokenType::Eof);
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

        assert_eq!(l.next().unwrap().ttype, TokenType::Let);
        assert_eq!(l.next().unwrap(), Token::new(TokenType::Identifier, String::from("five")));
        assert_eq!(l.next().unwrap().ttype, TokenType::Assign);
        assert_eq!(l.next().unwrap(), Token::new(TokenType::Integer, String::from("5")));
        assert_eq!(l.next().unwrap().ttype, TokenType::Semicolon);
        assert_eq!(l.next().unwrap().ttype, TokenType::Let);
        assert_eq!(l.next().unwrap(), Token::new(TokenType::Identifier, String::from("ten")));
        assert_eq!(l.next().unwrap().ttype, TokenType::Assign);
        assert_eq!(l.next().unwrap(), Token::new(TokenType::Integer, String::from("10")));
        assert_eq!(l.next().unwrap().ttype, TokenType::Semicolon);
        assert_eq!(l.next().unwrap().ttype, TokenType::Let);
        assert_eq!(l.next().unwrap(), Token::new(TokenType::Identifier, String::from("add")));
        assert_eq!(l.next().unwrap().ttype, TokenType::Assign);
        assert_eq!(l.next().unwrap().ttype, TokenType::Fn);
        assert_eq!(l.next().unwrap().ttype, TokenType::LeftParen);
        assert_eq!(l.next().unwrap(), Token::new(TokenType::Identifier, String::from("x")));
        assert_eq!(l.next().unwrap().ttype, TokenType::Comma);
        assert_eq!(l.next().unwrap(), Token::new(TokenType::Identifier, String::from("y")));
        assert_eq!(l.next().unwrap().ttype, TokenType::RightParen);
        assert_eq!(l.next().unwrap().ttype, TokenType::LeftCurlyBracket);
        assert_eq!(l.next().unwrap(), Token::new(TokenType::Identifier, String::from("x")));
        assert_eq!(l.next().unwrap().ttype, TokenType::Plus);
        assert_eq!(l.next().unwrap(), Token::new(TokenType::Identifier, String::from("y")));
        assert_eq!(l.next().unwrap().ttype, TokenType::Semicolon);
        assert_eq!(l.next().unwrap().ttype, TokenType::RightCurlyBracket);
        assert_eq!(l.next().unwrap().ttype, TokenType::Semicolon);
        assert_eq!(l.next().unwrap().ttype, TokenType::Let);
        assert_eq!(l.next().unwrap(), Token::new(TokenType::Identifier, String::from("result")));
        assert_eq!(l.next().unwrap().ttype, TokenType::Assign);
        assert_eq!(l.next().unwrap(), Token::new(TokenType::Identifier, String::from("add")));
        assert_eq!(l.next().unwrap().ttype, TokenType::LeftParen);
        assert_eq!(l.next().unwrap(), Token::new(TokenType::Identifier, String::from("five")));
        assert_eq!(l.next().unwrap().ttype, TokenType::Comma);
        assert_eq!(l.next().unwrap(), Token::new(TokenType::Identifier, String::from("ten")));
        assert_eq!(l.next().unwrap().ttype, TokenType::RightParen);
        assert_eq!(l.next().unwrap().ttype, TokenType::Semicolon);
        assert_eq!(l.next().unwrap().ttype, TokenType::Bang);
        assert_eq!(l.next().unwrap().ttype, TokenType::Minus);
        assert_eq!(l.next().unwrap().ttype, TokenType::Slash);
        assert_eq!(l.next().unwrap().ttype, TokenType::Asterisk);
        assert_eq!(l.next().unwrap(), Token::new(TokenType::Integer, String::from("5")));
        assert_eq!(l.next().unwrap().ttype, TokenType::Semicolon);
        assert_eq!(l.next().unwrap(), Token::new(TokenType::Integer, String::from("5")));
        assert_eq!(l.next().unwrap().ttype, TokenType::LessThan);
        assert_eq!(l.next().unwrap(), Token::new(TokenType::Integer, String::from("10")));
        assert_eq!(l.next().unwrap().ttype, TokenType::GreaterThan);
        assert_eq!(l.next().unwrap(), Token::new(TokenType::Integer, String::from("5")));
        assert_eq!(l.next().unwrap().ttype, TokenType::Semicolon);
        assert_eq!(l.next().unwrap().ttype, TokenType::If);
        assert_eq!(l.next().unwrap().ttype, TokenType::LeftParen);
        assert_eq!(l.next().unwrap(), Token::new(TokenType::Integer, String::from("5")));
        assert_eq!(l.next().unwrap().ttype, TokenType::LessThan);
        assert_eq!(l.next().unwrap(), Token::new(TokenType::Integer, String::from("10")));
        assert_eq!(l.next().unwrap().ttype, TokenType::RightParen);
        assert_eq!(l.next().unwrap().ttype, TokenType::LeftCurlyBracket);
        assert_eq!(l.next().unwrap().ttype, TokenType::Return);
        assert_eq!(l.next().unwrap().ttype, TokenType::True);
        assert_eq!(l.next().unwrap().ttype, TokenType::Semicolon);
        assert_eq!(l.next().unwrap().ttype, TokenType::RightCurlyBracket);
        assert_eq!(l.next().unwrap().ttype, TokenType::Else);
        assert_eq!(l.next().unwrap().ttype, TokenType::LeftCurlyBracket);
        assert_eq!(l.next().unwrap().ttype, TokenType::Return);
        assert_eq!(l.next().unwrap().ttype, TokenType::False);
        assert_eq!(l.next().unwrap().ttype, TokenType::Semicolon);
        assert_eq!(l.next().unwrap().ttype, TokenType::RightCurlyBracket);
        assert_eq!(l.next().unwrap(), Token::new(TokenType::Integer, String::from("10")));
        assert_eq!(l.next().unwrap().ttype, TokenType::Equal);
        assert_eq!(l.next().unwrap(), Token::new(TokenType::Integer, String::from("10")));
        assert_eq!(l.next().unwrap().ttype, TokenType::Semicolon);
        assert_eq!(l.next().unwrap(), Token::new(TokenType::Integer, String::from("10")));
        assert_eq!(l.next().unwrap().ttype, TokenType::NotEqual);
        assert_eq!(l.next().unwrap(), Token::new(TokenType::Integer, String::from("9")));
        assert_eq!(l.next().unwrap().ttype, TokenType::Semicolon);
        assert_eq!(l.next().unwrap(), Token::new(TokenType::Integer, String::from("10")));
        assert_eq!(l.next().unwrap().ttype, TokenType::GreaterThanOrEqual);
        assert_eq!(l.next().unwrap(), Token::new(TokenType::Integer, String::from("9")));
        assert_eq!(l.next().unwrap().ttype, TokenType::Semicolon);
        assert_eq!(l.next().unwrap(), Token::new(TokenType::Integer, String::from("9")));
        assert_eq!(l.next().unwrap().ttype, TokenType::LessThanOrEqual);
        assert_eq!(l.next().unwrap(), Token::new(TokenType::Integer, String::from("10")));
        assert_eq!(l.next().unwrap().ttype, TokenType::Semicolon);
        assert_eq!(l.next().unwrap(), Token::new(TokenType::String, String::from("\"te st\"")));
        assert_eq!(l.next().unwrap().ttype, TokenType::NotEqual);
        assert_eq!(l.next().unwrap(), Token::new(TokenType::String, String::from("\"test\"")));
        assert_eq!(l.next().unwrap().ttype, TokenType::Semicolon);
        assert_eq!(l.next().unwrap().ttype, TokenType::Eof);
    }
}
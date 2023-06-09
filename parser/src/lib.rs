use token::{Token, TokenType};
use lexer::Lexer;
#[allow(unused_imports)]
use ast::{
    Program, Statement,
    LetStatement, ReturnStatement, ExpressionStatement,
    Identifier, Expression,
    IntegerLiteral, Prefix, Infix, Postfix, Ternary,
};

#[allow(dead_code)]
#[derive(Debug, PartialEq, Clone, PartialOrd, Eq, Ord)]
enum Precedence {
    Lowest,
    Ternary,        // ?
    Equals,         // ==
    LessGreater,    // > or <
    Sum,            // +
    Product,        // *
    Prefix,         // -X or !X
    Call,           // myFunction(X)
    Index,          // array[index]
}

fn precedence_for_op(op: TokenType) -> Precedence {
    match op {
        TokenType::Question => Precedence::Ternary,
        TokenType::Equal | TokenType::NotEqual => Precedence::Equals,
        TokenType::LessThan | TokenType::GreaterThan | TokenType::LessThanOrEqual | TokenType::GreaterThanOrEqual => Precedence::LessGreater,
        TokenType::Plus | TokenType::Minus => Precedence::Sum,
        TokenType::Asterisk | TokenType::Slash | TokenType::Percent => Precedence::Product,
        TokenType::LeftParen => Precedence::Call,
        TokenType::LeftSquareBracket => Precedence::Index,
        _ => Precedence::Lowest,
    }
}

fn is_infix_op(op: TokenType) -> bool {
    match op {
        TokenType::Plus | TokenType::Minus | TokenType::Asterisk | TokenType::Slash | TokenType::Percent | TokenType::Equal | TokenType::NotEqual | TokenType::LessThan | TokenType::GreaterThan | TokenType::LessThanOrEqual | TokenType::GreaterThanOrEqual => true,
        _ => false,
    }
}

fn is_postfix_op(op: TokenType) -> bool {
    match op {
        TokenType::Increment | TokenType::Decrement => true,
        _ => false,
    }
}

pub struct Parser {
    l: Lexer,

    cur_token: Option<Token>,
    peek_token: Option<Token>,
}

impl Parser {
    pub fn new(l: Lexer) -> Parser {
        let mut p = Parser {
            l,
            cur_token: Some(Token::new(TokenType::Illegal, "".to_string())),
            peek_token: Some(Token::new(TokenType::Illegal, "".to_string())),
        };
        p.next_token();
        p.next_token();
        return p;
    }

    pub fn next_token(&mut self) {
        self.cur_token = self.peek_token.take();
        self.peek_token = self.l.next();
    }

    fn current_token_is(&self, t: TokenType) -> bool {
        if self.cur_token == None {
            return false;
        }
        return self.cur_token.clone().unwrap().ttype == t;
    }

    fn peek_token_is(&self, t: TokenType) -> bool {
        if self.peek_token == None {
            return false;
        }
        return self.peek_token.clone().unwrap().ttype == t;
    }

    fn expect_peek(&mut self, t: TokenType) -> Result<(), String> {
        return if self.peek_token_is(t.clone()) {
            self.next_token();
            Ok(())
        } else {
            Err(format!("expected next token to be {:?}, got {:?} instead", t, self.peek_token))
        }
    }

    pub fn parse_program(&mut self) -> Result<Program, String> {
        let mut statements: Vec<Statement> = Vec::new();

        while self.cur_token != None && !self.current_token_is(TokenType::Eof) {
            let stmt = self.parse_statement()?;
            statements.push(stmt);
            self.next_token();
        }

        return Ok(Program{statements});
    }

    fn parse_statement(&mut self) -> Result<Statement, String> {
        match self.cur_token {
            Some(ref token) => match token.ttype {
                TokenType::Let => self.parse_let_statement(),
                TokenType::Return => self.parse_return_statement(),
                _ => self.parse_expression_statement(),
            },
            _ => Err(format!("parse_statement() not implemented for {:?}", self.cur_token)),
        }
    }

    fn parse_let_statement(&mut self) -> Result<Statement, String> {
        let token = self.cur_token.clone().unwrap();

        self.expect_peek(TokenType::Identifier)?;
        let name = Identifier{
            token: self.cur_token.clone().unwrap(),
            value: self.cur_token.clone().unwrap().literal.clone()
        };
        self.expect_peek(TokenType::Assign)?;
        self.next_token();

        let value = self.parse_expression(Precedence::Lowest)?;

        if self.peek_token_is(TokenType::Semicolon) {
            self.next_token();
        }

        return Ok(Statement::LetStatement(LetStatement{
            token,
            name,
            value,
        }));
    }

    fn parse_return_statement(&mut self) -> Result<Statement, String> {
        let token = self.cur_token.clone().unwrap();
        self.next_token();

        let return_value = self.parse_expression(Precedence::Lowest)?;

        if self.peek_token_is(TokenType::Semicolon) {
            self.next_token();
        }

        return Ok(Statement::ReturnStatement(ReturnStatement{
            token,
            return_value,
        }));
    }

    fn parse_expression_statement(&mut self) -> Result<Statement, String> {
        let token = self.cur_token.clone().unwrap();
        let expression = self.parse_expression(Precedence::Lowest)?;

        if self.peek_token_is(TokenType::Semicolon) {
            self.next_token();
        }

        return Ok(Statement::ExpressionStatement(ExpressionStatement{
            token,
            expression,
        }));
    }

    fn parse_expression(&mut self, precedence: Precedence) -> Result<Expression, String> {
        let current = self.cur_token.clone().unwrap();
        let mut left: Expression = match current.ttype {
            TokenType::Identifier => {
                let mut left = Expression::Identifier(Identifier{
                    token: current.clone(),
                    value: current.literal.clone(),
                });
                if is_postfix_op(self.peek_token.clone().unwrap().ttype) {
                    self.next_token();
                    left = self.parse_postfix_expression(left)?;
                }
                left
            },
            TokenType::Integer => {
                let mut left = Expression::IntegerLiteral(IntegerLiteral{token: current.clone(), value: current.literal.parse::<i64>().unwrap()});
                if is_postfix_op(self.peek_token.clone().unwrap().ttype) {
                    self.next_token();
                    left = self.parse_postfix_expression(left)?;
                }
                left
            },
            TokenType::String => {
                Expression::StringLiteral(current.clone())
            },
            TokenType::True | TokenType::False => {
                Expression::BooleanLiteral(current.clone())
            },
            TokenType::Bang | TokenType::Minus | TokenType::Increment | TokenType::Decrement => {
                let mut left = self.parse_prefix_expression()?;
                match current.ttype {
                    TokenType::Minus | TokenType::Increment | TokenType::Decrement => {
                        if is_postfix_op(self.peek_token.clone().unwrap().ttype) {
                            self.next_token();
                            left = self.parse_postfix_expression(left)?;
                        }
                        left
                    },
                    _ => left,
                }
            },
            _ => {return Err(format!("parse_expression() not implemented for {:?}", current));},
        };

        while !self.peek_token_is(TokenType::Semicolon) && precedence < self.peek_precedence() {
            // println!("peek_precedence: {:?}", self.peek_precedence());
            if is_infix_op(self.peek_token.clone().unwrap().ttype) {
                self.next_token();
                left = self.parse_infix_expression(left)?;
            }
        }

        return Ok(left);
    }

    fn parse_prefix_expression(&mut self) -> Result<Expression, String> {
        // Cannot perform prefix operations on a string
        if self.peek_token.clone().unwrap().ttype == TokenType::String {
            return Err(format!("parse_prefix_expression() not implemented for {:?}", self.cur_token));
        }

        let token = self.cur_token.clone().unwrap();
        self.next_token();
        let right = self.parse_expression(Precedence::Prefix)?;
        return Ok(Expression::Prefix(Prefix{
            operator: token,
            right: Box::new(right),
        }));
    }

    fn parse_infix_expression(&mut self, left: Expression) -> Result<Expression, String> {
        let token = self.cur_token.clone().unwrap();
        let precedence = self.current_precedence();
        self.next_token();
        let right = self.parse_expression(precedence)?;
        return Ok(Expression::Infix(Infix{
            left: Box::new(left),
            operator: token,
            right: Box::new(right),
        }));
    }

    fn parse_postfix_expression(&mut self, left: Expression) -> Result<Expression, String> {
        let token = self.cur_token.clone().unwrap();
        return Ok(Expression::Postfix(Postfix{
            left: Box::new(left),
            operator: token,
        }));
    }

    fn peek_precedence(&self) -> Precedence {
        return precedence_for_op(self.peek_token.clone().unwrap().ttype);
    }

    fn current_precedence(&self) -> Precedence {
        return precedence_for_op(self.cur_token.clone().unwrap().ttype);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_let_statements() {
        let input = String::from(r#"
        let x = 5;
        let y = 10;
        let foobar = 838383;
        "#);

        let l = Lexer::new(input);
        let mut p = Parser::new(l);

        let program = p.parse_program();
        assert!(program.is_ok(), "parse_program() returned an error: {:?}", program.err().unwrap());

        let program = program.unwrap();
        assert_eq!(program.statements.len(), 3, "program.Statements does not contain 3 statements. got={}", program.statements.len());

        assert_eq!(program.statements[0].string(), "let x = 5;".to_string());
        assert_eq!(program.statements[1].string(), "let y = 10;".to_string());
        assert_eq!(program.statements[2].string(), "let foobar = 838383;".to_string());
    }

    #[test]
    fn test_return_statements() {
        let input = String::from(r#"
        return 5;
        return 10;
        return 993322;
        "#);

        let l = Lexer::new(input);
        let mut p = Parser::new(l);

        let program = p.parse_program();
        assert!(program.is_ok(), "parse_program() returned an error: {:?}", program.err().unwrap());

        let program = program.unwrap();
        assert_eq!(program.statements.len(), 3, "program.Statements does not contain 3 statements. got={}", program.statements.len());

        assert_eq!(program.statements[0].string(), "return 5;");
        assert_eq!(program.statements[1].string(), "return 10;".to_string());
        assert_eq!(program.statements[2].string(), "return 993322;".to_string());
    }

    #[test]
    fn test_identifier_expression() {
        let input = String::from("foobar;");

        let l = Lexer::new(input);
        let mut p = Parser::new(l);

        let program = p.parse_program();
        assert!(program.is_ok(), "parse_program() returned an error: {:?}", program.err().unwrap());

        let program = program.unwrap();
        assert_eq!(program.statements.len(), 1, "program.Statements does not contain 1 statements. got={}", program.statements.len());

        if let Statement::ExpressionStatement(expr) = &program.statements[0] {
            assert_eq!(expr.string(), "foobar;");
        } else {
            panic!("program.statements[0] is not ast.ExpressionStatement. got={:?}", program.statements[0]);
        }
    }

    #[test]
    fn test_literal_expression() {
        let input = String::from("5; \"test\";");

        let l = Lexer::new(input);
        let mut p = Parser::new(l);

        let program = p.parse_program();
        assert!(program.is_ok(), "parse_program() returned an error: {:?}", program.err().unwrap());

        let program = program.unwrap();
        assert_eq!(program.statements.len(), 2, "program.Statements does not contain 1 statements. got={}", program.statements.len());

        if let Statement::ExpressionStatement(expr) = &program.statements[0] {
            if let Expression::IntegerLiteral(int) = &expr.expression {
                assert_eq!(int.value, 5);
            } else {
                panic!("expr.expression is not ast.IntegerLiteral. got={:?}", expr.expression);
            }
            assert_eq!(expr.string(), "5;");
        } else {
            panic!("program.statements[0] is not ast.ExpressionStatement. got={:?}", program.statements[0]);
        }

        if let Statement::ExpressionStatement(expr) = &program.statements[1] {
            if let Expression::StringLiteral(str) = &expr.expression {
                assert_eq!(str.literal, "\"test\"");
            } else {
                panic!("expr.expression is not ast.StringLiteral. got={:?}", expr.expression);
            }
            assert_eq!(expr.string(), "\"test\";");
        } else {
            panic!("program.statements[1] is not ast.ExpressionStatement. got={:?}", program.statements[1]);
        }
    }

    struct PrefixTest {
        str: String,
        operator: TokenType,
        value: i64,
    }
    #[test]
    fn test_prefix_expression() {
        let input = String::from("!5; -15; ++5; --5; !test");

        let l = Lexer::new(input);
        let mut p = Parser::new(l);

        let program = p.parse_program();
        assert!(program.is_ok(), "parse_program() returned an error: {:?}", program.err().unwrap());

        let program = program.unwrap();
        assert_eq!(program.statements.len(), 5, "program.Statements does not contain 5 statements. got={}", program.statements.len());

        let tests = vec![
            PrefixTest { str: "(!5);".to_string(), operator: TokenType::Bang, value: 5},
            PrefixTest { str: "(-15);".to_string(), operator: TokenType::Minus, value: 15},
            PrefixTest { str: "(++5);".to_string(), operator: TokenType::Increment, value: 5},
            PrefixTest { str: "(--5);".to_string(), operator: TokenType:: Decrement, value: 5},
        ];

        for (i, test) in tests.iter().enumerate() {
            if let Statement::ExpressionStatement(expr) = &program.statements[i] {
                if let Expression::Prefix(pre) = &expr.expression {
                    assert_eq!(pre.operator.ttype, test.operator);
                    if let Expression::IntegerLiteral(int) = &*pre.right {
                        assert_eq!(int.value, test.value);
                    } else {
                        panic!("pre.right is not ast.IntegerLiteral. got={:?}", pre.right);
                    }
                } else {
                    panic!("expr.expression is not ast.Prefix. got={:?}", expr.expression);
                }
                assert_eq!(expr.string(), test.str);
            } else {
                panic!("program.statements[{}] is not ast.ExpressionStatement. got={:?}", i, program.statements[i]);
            }
        }

        if let Statement::ExpressionStatement(expr) = &program.statements[4] {
            if let Expression::Prefix(pre) = &expr.expression {
                assert_eq!(pre.operator.ttype, TokenType::Bang);
                if let Expression::Identifier(ident) = &*pre.right {
                    assert_eq!(ident.value, "test".to_string());
                } else {
                    panic!("pre.right is not ast.Identifier. got={:?}", pre.right);
                }
            } else {
                panic!("expr.expression is not ast.Prefix. got={:?}", expr.expression);
            }
            assert_eq!(expr.string(), "(!test);");
        } else {
            panic!("program.statements[3] is not ast.ExpressionStatement. got={:?}", program.statements[4]);
        }
    }

    struct InfixTest {
        str: String,
        lvalue: i64,
        operator: TokenType,
        rvalue: i64,
    }
    #[test]
    fn test_infix_expression() {
        let input = String::from(r#"
        5 + 5;
        5 - 5;
        5 * 5;
        5 / 5;
        5 % 5;
        5 == 5;
        5 != 5;
        5 < 5;
        5 > 5;
        5 <= 5;
        5 >= 5;
        "#);

        let l = Lexer::new(input);
        let mut p = Parser::new(l);

        let program = p.parse_program();
        assert!(program.is_ok(), "parse_program() returned an error: {:?}", program.err().unwrap());

        let program = program.unwrap();
        assert_eq!(program.statements.len(), 11, "program.Statements does not contain 11 statements. got={}", program.statements.len());

        let tests = vec![
            InfixTest { str: "(5 + 5);".to_string(), lvalue: 5, operator: TokenType::Plus, rvalue: 5},
            InfixTest { str: "(5 - 5);".to_string(), lvalue: 5, operator: TokenType::Minus, rvalue: 5},
            InfixTest { str: "(5 * 5);".to_string(), lvalue: 5, operator: TokenType::Asterisk, rvalue: 5},
            InfixTest { str: "(5 / 5);".to_string(), lvalue: 5, operator: TokenType::Slash, rvalue: 5},
            InfixTest { str: "(5 % 5);".to_string(), lvalue: 5, operator: TokenType::Percent, rvalue: 5},
            InfixTest { str: "(5 == 5);".to_string(), lvalue: 5, operator: TokenType::Equal, rvalue: 5},
            InfixTest { str: "(5 != 5);".to_string(), lvalue: 5, operator: TokenType::NotEqual, rvalue: 5},
            InfixTest { str: "(5 < 5);".to_string(), lvalue: 5, operator: TokenType::LessThan, rvalue: 5},
            InfixTest { str: "(5 > 5);".to_string(), lvalue: 5, operator: TokenType::GreaterThan, rvalue: 5},
            InfixTest { str: "(5 <= 5);".to_string(), lvalue: 5, operator: TokenType::LessThanOrEqual, rvalue: 5},
            InfixTest { str: "(5 >= 5);".to_string(), lvalue: 5, operator: TokenType::GreaterThanOrEqual, rvalue: 5},
        ];

        for (i, test) in tests.iter().enumerate() {
            if let Statement::ExpressionStatement(expr) = &program.statements[i] {
                if let Expression::Infix(inf) = &expr.expression {
                    assert_eq!(inf.operator.ttype, test.operator);
                    if let Expression::IntegerLiteral(int) = &*inf.left {
                        assert_eq!(int.value, test.lvalue);
                    } else {
                        panic!("inf.left is not ast.IntegerLiteral. got={:?}", inf.left);
                    }
                    if let Expression::IntegerLiteral(int) = &*inf.right {
                        assert_eq!(int.value, test.rvalue);
                    } else {
                        panic!("inf.right is not ast.IntegerLiteral. got={:?}", inf.right);
                    }
                } else {
                    panic!("expr.expression is not ast.Infix. got={:?}", expr.expression);
                }
                assert_eq!(expr.string(), test.str);
            } else {
                panic!("program.statements[{}] is not ast.ExpressionStatement. got={:?}", i, program.statements[i]);
            }
        }
    }

    struct PrecedenceTest {
        str: String,
        expected: String,
    }
    #[test]
    fn test_operator_precedence_parsing() {
        let tests = vec![
            PrecedenceTest{str: "-a * b".to_string(), expected: "((-a) * b);".to_string()},
            PrecedenceTest{str: "!-a".to_string(), expected: "(!(-a));".to_string()},
            PrecedenceTest{str: "a + b + c".to_string(), expected: "((a + b) + c);".to_string()},
            PrecedenceTest{str: "a + b - c".to_string(), expected: "((a + b) - c);".to_string()},
            PrecedenceTest{str: "a * b * c".to_string(), expected: "((a * b) * c);".to_string()},
            PrecedenceTest{str: "a * b / c".to_string(), expected: "((a * b) / c);".to_string()},
            PrecedenceTest{str: "a + b / c".to_string(), expected: "(a + (b / c));".to_string()},
            PrecedenceTest{str: "a + b * c + d / e - f".to_string(), expected: "(((a + (b * c)) + (d / e)) - f);".to_string()},
            PrecedenceTest{str: "3 + 4; -5 * 5".to_string(), expected: "(3 + 4);((-5) * 5);".to_string()},
            PrecedenceTest{str: "5 > 4 == 3 < 4".to_string(), expected: "((5 > 4) == (3 < 4));".to_string()},
            PrecedenceTest{str: "5 < 4 != 3 > 4".to_string(), expected: "((5 < 4) != (3 > 4));".to_string()},
            PrecedenceTest{str: "3 + 4 * 5 == 3 * 1 + 4 * 5".to_string(), expected: "((3 + (4 * 5)) == ((3 * 1) + (4 * 5)));".to_string()},
        ];

        for test in tests {
            let l = Lexer::new(test.str);
            let mut p = Parser::new(l);
            let program = p.parse_program().unwrap();

            let actual = program.string();
            assert_eq!(test.expected, actual)
        }
    }

    struct PostfixTest {
        str: String,
        expected: String,
    }
    #[test]
    fn test_postfix_parsing() {
        let tests = vec![
            PostfixTest{str: "a++".to_string(), expected: "(a++);".to_string()},
            PostfixTest{str: "!a--".to_string(), expected: "(!(a--));".to_string()},
            PostfixTest{str: "a++ + b + c--".to_string(), expected: "(((a++) + b) + (c--));".to_string()},
            PostfixTest{str: "a + b++ * c + d / --e - f".to_string(), expected: "(((a + ((b++) * c)) + (d / (--e))) - f);".to_string()},
        ];

        for test in tests {
            let l = Lexer::new(test.str);
            let mut p = Parser::new(l);
            let program = p.parse_program().unwrap();

            let actual = program.string();
            assert_eq!(test.expected, actual)
        }
    }

    #[test]
    fn test_expression() {
        let l = Lexer::new("a b c".to_string());
        let mut p = Parser::new(l);
        let program = p.parse_program().unwrap();

        assert_eq!(program.statements.len(), 3);
    }
}

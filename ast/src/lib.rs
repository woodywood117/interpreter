#[allow(unused_imports)]
use token::{Token, TokenType};

#[allow(dead_code)]
pub struct Program {
    pub statements: Vec<Statement>,
}
impl Program {
    pub fn string(&self) -> String {
        let mut s = String::new();
        for statement in &self.statements {
            s.push_str(&statement.string());
        }
        s
    }
}

#[derive(Debug, PartialEq)]
pub enum Statement {
    LetStatement(LetStatement),
    ReturnStatement(ReturnStatement),
    ExpressionStatement(ExpressionStatement),
}

impl Statement {
    pub fn string(&self) -> String {
        match self {
            Statement::LetStatement(ls) => ls.string(),
            Statement::ReturnStatement(rs) => rs.string(),
            Statement::ExpressionStatement(es) => es.string(),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    IntegerLiteral(IntegerLiteral),
    StringLiteral(Token),
    BooleanLiteral(Token),
    Identifier(Identifier),
    Prefix(Prefix),
    Infix(Infix),
    Postfix(Postfix),
    Ternary(Ternary),
}

impl Expression {
    pub fn string(&self) -> String {
        match self {
            Expression::IntegerLiteral(l) => l.string(),
            Expression::StringLiteral(l) => l.literal.clone(),
            Expression::BooleanLiteral(l) => l.literal.clone(),
            Expression::Identifier(i) => i.string(),
            Expression::Prefix(p) => p.string(),
            Expression::Infix(i) => i.string(),
            Expression::Postfix(p) => p.string(),
            Expression::Ternary(t) => t.string(),
        }
    }
}


#[derive(Debug, PartialEq)]
pub struct LetStatement {
    pub token: Token,
    pub name: Identifier,
    pub value: Expression,
}
impl LetStatement {
    pub fn string(&self) -> String {
        format!("{} {} = {};", self.token.literal, self.name.value, self.value.string())
    }
}


#[derive(Debug, PartialEq)]
pub struct ReturnStatement {
    pub token: Token,
    pub return_value: Expression,
}
impl ReturnStatement {
    pub fn string(&self) -> String {
        format!("{} {};", self.token.literal, self.return_value.string())
    }
}


#[derive(Debug, PartialEq)]
pub struct ExpressionStatement {
    pub token: Token,
    pub expression: Expression,
}
impl ExpressionStatement {
    pub fn string(&self) -> String {
        format!("{};", self.expression.string())
    }
}


#[derive(Debug, PartialEq, Clone)]
pub struct IntegerLiteral {
    pub token: Token,
    pub value: i64,
}
impl IntegerLiteral {
    pub fn string(&self) -> String {
        self.token.literal.clone()
    }
}


#[derive(Debug, PartialEq, Clone)]
pub struct Identifier {
    pub token: Token,
    pub value: String,
}
impl Identifier {
    pub fn string(&self) -> String {
        self.value.clone()
    }
}


#[derive(Debug, PartialEq, Clone)]
pub struct Prefix {
    pub operator: Token,
    pub right: Box<Expression>,
}
impl Prefix {
    pub fn string(&self) -> String {
        format!("({}{})", self.operator.literal, self.right.clone().string())
    }
}


#[derive(Debug, PartialEq, Clone)]
pub struct Infix {
    pub left: Box<Expression>,
    pub operator: Token,
    pub right: Box<Expression>,
}
impl Infix {
    pub fn string(&self) -> String {
        format!("({} {} {})", self.left.string(), self.operator.literal, self.right.string())
    }
}


#[derive(Debug, PartialEq, Clone)]
pub struct Postfix {
    pub left: Box<Expression>,
    pub operator: Token,
}
impl Postfix {
    pub fn string(&self) -> String {
        format!("({}{})", self.left.clone().string(), self.operator.literal)
    }
}

// TODO
#[derive(Debug, PartialEq, Clone)]
pub struct Ternary {
    pub condition: Box<Expression>,
    pub if_true: Box<Expression>,
    pub if_false: Box<Expression>,
}
impl Ternary {
    pub fn string(&self) -> String {
        format!("{} ? {} : {}", self.condition.string(), self.if_true.string(), self.if_false.string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string() {
        let program = Program {
            statements: vec![
                Statement::LetStatement(
                    LetStatement {
                        token: Token::new(TokenType::Let, "let".to_string()),
                        name: Identifier {
                            token: Token::new(TokenType::Identifier, "myVar".to_string()),
                            value: "myVar".to_string(),
                        },
                        value: Expression::Identifier(
                            Identifier {
                                token: Token::new(TokenType::Identifier, "anotherVar".to_string()),
                                value: "anotherVar".to_string(),
                            }
                        ),
                    }
                ),
                Statement::ReturnStatement(
                    ReturnStatement {
                        token: Token::new(TokenType::Return, "return".to_string()),
                        return_value: Expression::Identifier(
                            Identifier {
                                token: Token::new(TokenType::Identifier, "myVar".to_string()),
                                value: "myVar".to_string(),
                            }
                        ),
                    }
                ),
            ],
        };

        assert_eq!(program.statements[0].string(), "let myVar = anotherVar;");
        assert_eq!(program.statements[1].string(), "return myVar;");
    }
}
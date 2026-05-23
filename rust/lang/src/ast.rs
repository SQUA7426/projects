use crate::token::Token;

#[derive(Debug, PartialEq, PartialOrd)]
pub enum Expr {
    Literal(Token),

    Comparison(Box<Expr>, Token, Box<Expr>),

    Addition(Box<Expr>, Box<Expr>),
    Subtraction(Box<Expr>, Box<Expr>),
    Multiplication(Box<Expr>, Box<Expr>),
    Division(Box<Expr>, Box<Expr>),
}

#[derive(Debug)]
pub enum Statement {
    LET(String, Expr),
    ASSIGN(String, Expr),

    Expr(Expr),

    IF(Expr, Vec<Statement>),
    WHILE(Expr, Vec<Statement>),
}

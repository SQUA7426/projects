use crate::ast::{Expr, Statement};
use crate::token::Token;

#[derive(Debug, Clone)]
pub struct Parser {
    tokens: Vec<Token>,
    curr_pos: usize,
}

#[allow(dead_code)]
impl Parser {
    pub fn new(toks: Vec<Token>) -> Self {
        Self {
            tokens: toks,
            curr_pos: 0,
        }
    }
    pub fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.curr_pos)
    }

    pub fn next(&mut self) -> Option<&Token> {
        if self.curr_pos < self.tokens.len() {
            self.curr_pos += 1;
            self.tokens.get(self.curr_pos - 1)
        } else {
            None
        }
    }

    pub fn parse(&mut self) -> Option<Expr> {
        self.parse_addition()
    }

    pub fn parse_primary(&mut self) -> Option<Expr> {
        let token = self.peek()?.clone();
        match token {
            Token::Number(_) | Token::Str(_) | Token::Var(_) => {
                self.next();
                Some(Expr::Literal(token))
            }
            _ => None,
        }
    }

    pub fn parse_addition(&mut self) -> Option<Expr> {
        let mut expr = self.parse_multiplication()?;
        while let Some(token) = self.peek() {
            match token {
                Token::PLUS => {
                    self.next();
                    let right = self.parse_multiplication()?;
                    expr = Expr::Addition(Box::new(expr), Box::new(right));
                }
                Token::MINUS => {
                    self.next();
                    let right = self.parse_multiplication()?;
                    expr = Expr::Subtraction(Box::new(expr), Box::new(right));
                }
                _ => break,
            }
        }
        Some(expr)
    }

    pub fn parse_multiplication(&mut self) -> Option<Expr> {
        let mut expr = self.parse_primary()?;
        while let Some(token) = self.peek() {
            match token {
                Token::STAR => {
                    self.next();
                    let right = self.parse_primary()?;
                    expr = Expr::Multiplication(Box::new(expr), Box::new(right));
                }
                Token::SLASH => {
                    self.next();
                    let right = self.parse_primary()?;
                    expr = Expr::Division(Box::new(expr), Box::new(right));
                }
                _ => break,
            }
        }
        Some(expr)
    }

    pub fn parse_cond(&mut self) -> Option<Expr> {
        let left = self.parse_addition()?;

        if let Some(token) = self.peek().cloned() {
            if token == Token::EQ
                || token == Token::UNEQ
                || token == Token::GE
                || token == Token::LE
                || token == Token::GT
                || token == Token::LT
            {
                self.next();

                let right = if self.peek() == Some(&Token::Bool(true)) {
                    self.next();
                    Expr::Literal(Token::Bool(true))
                } else if self.peek() == Some(&Token::Bool(false)) {
                    self.next();
                    Expr::Literal(Token::Bool(false))
                } else {
                    self.parse_addition()?
                };
                return Some(Expr::Comparison(Box::new(left), token, Box::new(right)));
            }
        }
        Some(left)
    }

    pub fn parse_if_cond(&mut self) -> Option<Statement> {
        self.next();
        // println!("Debugger Found IF, parsing condition...");

        let condition = self.parse_cond()?;
        // println!("Parsed condition!");

        if self.peek() == Some(&Token::THEN) {
            // println!("Debugger Found THEN, parsing body...");
            self.next();
        } else {
            panic!("Syntax Error; Expected 'then'!")
        }

        let mut body = Vec::new();

        while self.peek() != Some(&Token::END) && self.peek().is_some() {
            if let Some(stmt) = self.parse_statement() {
                body.push(stmt);
            } else {
                // println!("Debugger Found END, if Statement complete!");
                break;
            }
        }

        if self.peek() == Some(&Token::END) {
            self.next();
        } else {
            panic!("Syntax Error: Expected 'end' !");
        }
        Some(Statement::IF(condition, body))
    }

    pub fn parse_while_cond(&mut self) -> Option<Statement> {
        // println!("Debugger found WHILE.. parse condition..");
        self.next();

        let condition = self.parse_cond()?;
        // println!("Debugger parsed condition!");

        if self.peek() == Some(&Token::DO) {
            // println!("Debugger found DO.. parse body..");
            self.next();
        } else {
            panic!("Syntax Error; Expected 'do'!")
        }

        let mut body = Vec::new();

        while self.peek() != Some(&Token::END) && self.peek().is_some() {
            if let Some(stmt) = self.parse_statement() {
                body.push(stmt);
            } else {
                break;
            }
        }

        if self.peek() == Some(&Token::END) {
            self.next();
        } else {
            panic!("Syntax Error: Expected 'end' !");
        }
        Some(Statement::WHILE(condition, body))
    }

    pub fn parse_statement(&mut self) -> Option<Statement> {
        match self.peek() {
            Some(Token::LET) => {
                self.next();
                if let Some(Token::Var(name)) = self.next().cloned() {
                    if let Some(Token::ASSIGN) = self.peek() {
                        self.next();
                        let expr = self.parse_addition()?;
                        return Some(Statement::LET(name, expr));
                    }
                }
                None
            }
            Some(Token::Var(name)) => {
                if self.curr_pos + 1 < self.tokens.len()
                    && self.tokens[self.curr_pos + 1] == Token::ASSIGN
                {
                    let cloned_name = name.clone();
                    self.next();
                    self.next();

                    let expr = self.parse_addition()?;

                    return Some(Statement::ASSIGN(cloned_name, expr));
                }
                self.parse_addition().map(Statement::Expr)
            }

            Some(Token::IF) => self.parse_if_cond(),

            Some(Token::WHILE) => self.parse_while_cond(),

            _ => self.parse_addition().map(Statement::Expr),
        }
    }
}

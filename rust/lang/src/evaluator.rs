use std::collections::HashMap;

use crate::ast::{Expr, Statement};
use crate::token::Token;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum RuntimeEval {
    Number(i64),
    Float(f64),
    Bool(bool),
    Str(String),
}

type Env = HashMap<String, RuntimeEval>;

pub fn evaluate_expr(expr: &Expr, env: &Env) -> RuntimeEval {
    match expr {
        // unpack
        Expr::Literal(token) => match token {
            Token::Number(num) => RuntimeEval::Number(*num),
            Token::Float(flt) => RuntimeEval::Float(*flt),
            Token::Bool(b) => RuntimeEval::Bool(*b),
            Token::Var(name) => env.get(name).cloned().unwrap_or_else(|| {
                panic!("Unknown Literal VAR: {} in Evaluator", name);
            }),
            Token::Str(name) => RuntimeEval::Str(name.clone().to_string()),
            _ => panic!("Runtime Error: Unknown Token for Literal"),
        },

        Expr::Comparison(left_box, op, right_box) => {
            // println!("IN EVAL Comparison...");
            let l = evaluate_expr(left_box, env);
            let r = evaluate_expr(right_box, env);
            match op {
                Token::EQ => RuntimeEval::Bool(l == r),
                Token::UNEQ => RuntimeEval::Bool(l != r),
                Token::GT => RuntimeEval::Bool(l > r),
                Token::GE => RuntimeEval::Bool(l >= r),
                Token::LT => RuntimeEval::Bool(l < r),
                Token::LE => RuntimeEval::Bool(l <= r),
                _ => panic!("Only Operations accepted!"),
            }
        }

        Expr::Addition(l, r) => match (evaluate_expr(l, env), evaluate_expr(r, env)) {
            (RuntimeEval::Number(a), RuntimeEval::Number(b)) => RuntimeEval::Number(a + b),
            (RuntimeEval::Str(a), RuntimeEval::Number(b)) => {
                RuntimeEval::Str(format!("{}{}", a, b))
            }
            (RuntimeEval::Number(a), RuntimeEval::Str(b)) => {
                RuntimeEval::Str(format!("{}{}", a, b))
            }

            (RuntimeEval::Float(a), RuntimeEval::Float(b)) => RuntimeEval::Float(a + b),

            (RuntimeEval::Float(a), RuntimeEval::Number(b)) => RuntimeEval::Float(a + b as f64),
            (RuntimeEval::Number(a), RuntimeEval::Float(b)) => RuntimeEval::Float(a as f64 + b),

            (RuntimeEval::Str(a), RuntimeEval::Float(b)) => RuntimeEval::Str(format!("{}{}", a, b)),
            (RuntimeEval::Float(a), RuntimeEval::Str(b)) => RuntimeEval::Str(format!("{}{}", a, b)),

            (RuntimeEval::Str(a), RuntimeEval::Str(b)) => RuntimeEval::Str(format!("{}{}", a, b)),

            _ => panic!("Runtime Error: Can only Add Strings, Floats and Numbers!"),
        },

        Expr::Subtraction(l, r) => match (evaluate_expr(l, env), evaluate_expr(r, env)) {
            (RuntimeEval::Number(a), RuntimeEval::Number(b)) => RuntimeEval::Number(a - b),
            (RuntimeEval::Number(a), RuntimeEval::Float(b)) => RuntimeEval::Float(a as f64 - b),
            (RuntimeEval::Float(a), RuntimeEval::Number(b)) => RuntimeEval::Float(a - b as f64),
            (RuntimeEval::Float(a), RuntimeEval::Float(b)) => RuntimeEval::Float(a - b),
            _ => panic!("Runtime Error: Can only subtract Numbers!"),
        },

        Expr::Multiplication(l, r) => match (evaluate_expr(l, env), evaluate_expr(r, env)) {
            (RuntimeEval::Number(a), RuntimeEval::Number(b)) => RuntimeEval::Number(a * b),
            (RuntimeEval::Number(a), RuntimeEval::Str(b)) => {
                if a < 0 {
                    panic!(
                        "Runtime Error: Cannot Multiply a String with a Negative Value: {} !",
                        a
                    )
                }
                RuntimeEval::Str(b.repeat(a as usize))
            }
            (RuntimeEval::Str(a), RuntimeEval::Number(b)) => {
                if b < 0 {
                    panic!(
                        "Runtime Error: Cannot Multiply a String with a Negative Value: {} !",
                        b
                    )
                }
                RuntimeEval::Str(a.repeat(b as usize))
            }
            _ => panic!("Runtime Error: Can not multiply Strings!"),
        },

        Expr::Division(l, r) => match (evaluate_expr(l, env), evaluate_expr(r, env)) {
            (RuntimeEval::Number(a), RuntimeEval::Number(b)) => {
                if b == 0 {
                    panic!("Cannot divide by Zero");
                }
                RuntimeEval::Number(a / b)
            }
            (RuntimeEval::Number(a), RuntimeEval::Float(b)) => {
                if b == 0. {
                    panic!("Cannot divide by Zero");
                }
                RuntimeEval::Float(a as f64 / b)
            }
            (RuntimeEval::Float(a), RuntimeEval::Number(b)) => {
                if b == 0 {
                    panic!("Cannot divide by Zero");
                }
                RuntimeEval::Float(a / b as f64)
            }
            (RuntimeEval::Float(a), RuntimeEval::Float(b)) => {
                if b == 0. {
                    panic!("Cannot divide by Zero");
                }
                RuntimeEval::Float(a / b)
            }
            _ => panic!("Runtime Error: Can only divide two Numbers!"),
        },
    }
}

pub fn exe_stmt(stmt: &Statement, env: &mut Env) {
    match stmt {
        Statement::LET(name, expr) => {
            let value = evaluate_expr(expr, env);
            env.insert(name.clone(), value);
        }
        Statement::ASSIGN(name, expr) => {
            let value = evaluate_expr(expr, env);

            // if env.contains_key(name) {
            env.insert(name.clone(), value);
            // } else {
            //     panic!("Runtime Error: Asserting non declared VAR: '{}'!", name);
            // }
        }
        Statement::Expr(expr) => {
            evaluate_expr(expr, env);
        }
        Statement::IF(cond, body) => {
            let cond_val = evaluate_expr(cond, env);

            match cond_val {
                RuntimeEval::Bool(b) => {
                    if b {
                        for body_stmt in body {
                            exe_stmt(body_stmt, env);
                        }
                    }
                }
                _ => panic!("Runtime Error: IF Clause incorrect!"),
            }
        }
        Statement::WHILE(cond, body) => {
            // println!("IN Statement::WHILE...");
            while let RuntimeEval::Bool(true) = evaluate_expr(cond, env) {
                for body_stmt in body {
                    // println!("In body_stmt inside body..");
                    exe_stmt(body_stmt, env);
                }
            }
        }
    }
}

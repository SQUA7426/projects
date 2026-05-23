use std::collections::HashMap;

mod ast;
mod evaluator;
mod parser;
mod token;
mod tokenizer;

use crate::evaluator::exe_stmt;
use crate::{parser::Parser, tokenizer::Tokenizer};
fn main() {
    let mut env = HashMap::new();

    let tok = vec![
        "a = 1",
        "-- Comment a = 1",
        "while a <= 10 do x = 10 a = a + 1 end",
        "if a > 10 then x = 4 end",
    ];
    for i in 0..tok.len() {
        println!("Input: {}", tok[i]);

        let t = Tokenizer::tokenize(tok[i].into());
        println!("Tokens: {}", t.to_string());

        let mut p = Parser::new(t.get_tokens());

        if let Some(stmt) = p.parse_statement() {
            println!("Parser created AST: {:?}\n", stmt);
            exe_stmt(&stmt, &mut env);
            println!("After executing Statement:\n{:?}", stmt);
        }
        println!("{}", "-".repeat(50));
        env.iter().for_each(|entry| println!("{entry:?}"));
        println!("{}", "-".repeat(50));
    }
}

#[cfg(test)]
mod test {
    use crate::tokenizer::Tokenizer;
    use std::collections::HashMap;

    use crate::evaluator::{RuntimeEval, exe_stmt};
    use crate::parser::Parser;
    #[test]
    fn example_tokenize() {
        let mut tok = "x = 100";
        println!("Token: {}", tok);
        let mut t = Tokenizer::tokenize(tok.into());
        println!("{}", t.to_string());

        tok = "for i in 10";
        println!("Token: {}", tok);
        t = Tokenizer::tokenize(tok.into());
        println!("{}", t.to_string());

        tok = "true false func x 10000 +6 -111 for ( in ) 1.2 + -";
        println!("Token: {}", tok);
        t = Tokenizer::tokenize(tok.into());
        println!("{}", t.to_string());
    }

    #[test]
    fn eval_num_str() {
        let mut env = HashMap::new();

        let tok = "x = 3 * 4 + \"M\"";
        let t = Tokenizer::tokenize(tok.into());

        let mut p = Parser::new(t.get_tokens());
        if let Some(stmt) = p.parse_statement() {
            println!("Parser created AST: {:?}", stmt);
            exe_stmt(&stmt, &mut env);
            println!("After executing Statement:\n{:?}", stmt);
        }

        if env.contains_key("x") {
            print!("{:?}", env.entry("x".into()));
        }

        assert_eq!(env.contains_key("x"), true);
        assert_eq!(env.get("x"), Some(&RuntimeEval::Str("12M".into())));
    }

    #[test]
    fn test_while_if() {
        let mut env = HashMap::new();

        let tok = vec![
            "a = 1",
            "while a <= 10 do x = 10 a = a + 1 end",
            "if a > 10 then x = 4 end",
        ];
        for i in 0..tok.len() {
            println!("Input: {}", tok[i]);

            let t = Tokenizer::tokenize(tok[i].into());
            println!("Tokens: {}", t.to_string());

            let mut p = Parser::new(t.get_tokens());

            if let Some(stmt) = p.parse_statement() {
                println!("Parser created AST: {:?}\n", stmt);
                exe_stmt(&stmt, &mut env);
                println!("After executing Statement:\n{:?}", stmt);
            }
            if env.contains_key("a") {
                println!("{:?}", env.entry("a".into()));
            }
            if env.contains_key("x") {
                println!("{:?}", env.entry("x".into()));
            }
        }
    }
}

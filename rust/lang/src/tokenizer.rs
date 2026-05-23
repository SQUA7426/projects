use crate::token::Token;
use regex::Regex;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Tokenizer {
    tokens: Vec<Token>,
}

#[allow(dead_code)]
impl Tokenizer {
    pub fn get_tokens(&self) -> Vec<Token> {
        self.tokens.clone()
    }

    pub fn tokenize(lines: String) -> Self {
        let var = Regex::new(r"\w+").unwrap();

        let char = Regex::new(r"\'\w{1}\'").unwrap();
        let string = Regex::new(r#""[^"\r\n]*""#).unwrap();
        let boolean = Regex::new(r"(true)|(false)").unwrap();
        let nums = Regex::new(r"^-?\d+\.\d+|-?\d+").unwrap();

        let plus = Regex::new(r"(\+{1})").unwrap();
        let minus = Regex::new(r"(\-{1})").unwrap();
        let star = Regex::new(r"(\*{1})").unwrap();
        let slash = Regex::new(r"(\/{1})").unwrap();
        let sq = Regex::new(r"(\^{1})").unwrap();

        let eq = Regex::new(r"^(\=\=){1}").unwrap();
        let uneq = Regex::new(r"^(\!\=){1}").unwrap();

        let ge = Regex::new(r"^>=").unwrap();
        let gt = Regex::new(r"^>").unwrap();
        let le = Regex::new(r"^<=").unwrap();
        let lt = Regex::new(r"^<").unwrap();

        let letting = Regex::new(r"^let\b").unwrap();
        let assign = Regex::new(r"^=").unwrap();

        let i_f = Regex::new(r"(if)").unwrap();
        let elseif = Regex::new(r"(elseif)").unwrap();
        let el_se = Regex::new(r"(else)").unwrap();

        let then = Regex::new(r"(then)").unwrap();

        let wh_ile = Regex::new(r"(while)").unwrap();
        let d0 = Regex::new(r"(do)").unwrap();

        let f0r = Regex::new(r"(for)").unwrap();
        let i_n = Regex::new(r"(in)").unwrap();

        let ending = Regex::new(r"(end)").unwrap();

        let sparent = Regex::new(r"\({1}").unwrap();
        let eparent = Regex::new(r"\){1}").unwrap();

        let curls = Regex::new(r"\{{1}").unwrap();
        let curle = Regex::new(r"\}{1}").unwrap();

        let square_s = Regex::new(r"\[{1}").unwrap();
        let square_e = Regex::new(r"\[{1}").unwrap();

        let func = Regex::new(r"func").unwrap();

        let mut liner: Vec<Token> = vec![];

        for line in lines.lines() {
            let clean_line = match line.split("--").next() {
                Some(c) => c,
                None => line,
            };

            for s in clean_line.split_whitespace() {
                if let Some(_t) = char.find(s) {
                    liner.push(Token::Char);
                } else if let Some(_t) = string.find(s) {
                    liner.push(Token::Str(String::from(&s[1..s.len() - 1])));
                } else if let Some(_t) = boolean.find(s) {
                    if s == "true" {
                        liner.push(Token::Bool(true));
                    } else {
                        liner.push(Token::Bool(false));
                    }
                } else if let Some(_t) = nums.find(s) {
                    if s.contains(".") {
                        liner.push(Token::Float(s.parse::<f64>().unwrap()));
                    } else {
                        liner.push(Token::Number(s.parse::<i64>().unwrap()));
                    }
                } else if let Some(_t) = plus.find(s) {
                    liner.push(Token::PLUS);
                } else if let Some(_t) = minus.find(s) {
                    liner.push(Token::MINUS);
                } else if let Some(_t) = star.find(s) {
                    liner.push(Token::STAR);
                } else if let Some(_t) = slash.find(s) {
                    liner.push(Token::SLASH);
                } else if let Some(_t) = sq.find(s) {
                    liner.push(Token::SQ);
                } else if let Some(_t) = letting.find(s) {
                    liner.push(Token::LET);
                } else if let Some(_t) = i_f.find(s) {
                    liner.push(Token::IF);
                } else if let Some(_t) = elseif.find(s) {
                    liner.push(Token::ELSEIF);
                } else if let Some(_t) = el_se.find(s) {
                    liner.push(Token::ELSE);
                } else if let Some(_t) = then.find(s) {
                    liner.push(Token::THEN);
                } else if let Some(_t) = wh_ile.find(s) {
                    liner.push(Token::WHILE);
                } else if let Some(_t) = d0.find(s) {
                    liner.push(Token::DO);
                } else if let Some(_t) = f0r.find(s) {
                    liner.push(Token::FOR);
                } else if let Some(_t) = i_n.find(s) {
                    liner.push(Token::IN);
                } else if let Some(_t) = ending.find(s) {
                    liner.push(Token::END);
                } else if let Some(_t) = sparent.find(s) {
                    liner.push(Token::SPARENT);
                } else if let Some(_t) = eparent.find(s) {
                    liner.push(Token::EPARENT);
                } else if let Some(_t) = curls.find(s) {
                    liner.push(Token::CurlSbracket);
                } else if let Some(_t) = curle.find(s) {
                    liner.push(Token::CurlEbreacket);
                } else if let Some(_t) = square_s.find(s) {
                    liner.push(Token::SquareSbracket);
                } else if let Some(_t) = square_e.find(s) {
                    liner.push(Token::SquareEbracket);
                } else if let Some(_t) = func.find(s) {
                    liner.push(Token::Func);
                } else if let Some(_t) = var.find(s) {
                    liner.push(Token::Var(String::from(s)));
                } else if let Some(_t) = eq.find(s) {
                    liner.push(Token::EQ);
                } else if let Some(_t) = uneq.find(s) {
                    liner.push(Token::UNEQ);
                } else if let Some(_t) = ge.find(s) {
                    liner.push(Token::GE);
                } else if let Some(_t) = le.find(s) {
                    liner.push(Token::LE);
                } else if let Some(_t) = assign.find(s) {
                    liner.push(Token::ASSIGN);
                } else if let Some(_t) = gt.find(s) {
                    liner.push(Token::GT);
                } else if let Some(_t) = lt.find(s) {
                    liner.push(Token::LT);
                } else {
                    liner.push(Token::None);
                }
            }
        }
        Self { tokens: liner }
    }
}

impl fmt::Display for Tokenizer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for v in self.tokens.iter() {
            write!(f, "<{:?}>", v)?;
        }
        Ok(())
    }
}

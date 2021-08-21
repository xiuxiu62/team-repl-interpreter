use logos::{Lexer, Logos};
use std::fmt;

pub type Tokens = Vec<Token>;

pub fn lex(source: &str) -> Tokens {
    Token::lexer(&source).collect::<Tokens>()
}

// Encodes token if any, otherwise None
fn stringify(lex: &mut Lexer<Token>) -> Option<String> {
    Some(lex.slice().parse::<String>().ok()?)
}

fn stringify_comment(lex: &mut Lexer<Token>) -> Option<String> {
    Some(lex.slice().parse::<String>().ok()?)
}

fn lex_proc(lex: &mut Lexer<Token>) -> Option<Procedure> {
    match lex.slice().parse::<String>().ok()?.as_str() {
        "const" => Some(Procedure::Const),
        "=" => Some(Procedure::Assign),
        "+" => Some(Procedure::Add),
        "-" => Some(Procedure::Sub),
        "*" => Some(Procedure::Mul),
        "/" => Some(Procedure::Div),
        "%" => Some(Procedure::Mod),
        _ => None,
    }
}

fn lex_comp(lex: &mut Lexer<Token>) -> Option<Comparison> {
    match lex.slice().parse::<String>().ok()?.as_str() {
        "and" => Some(Comparison::And),
        "or" => Some(Comparison::Or),
        "xor" => Some(Comparison::Or),
        "not" => Some(Comparison::Not),
        "eq" => Some(Comparison::Equal),
        "neq" => Some(Comparison::NotEqual),
        "gt" => Some(Comparison::Gt),
        "gte" => Some(Comparison::Gte),
        "lt" => Some(Comparison::Lt),
        "lte" => Some(Comparison::Lte),
        _ => None,
    }
}

#[derive(Logos, Debug, Clone, PartialEq)]
pub enum Token {
    #[token("(")]
    Lparen,
    #[token(")")]
    Rparen,
    #[token("{")]
    Lbrace,
    #[token("}")]
    Rbrace,
    #[token(",")]
    Comma,

    #[token("const", lex_proc)]
    #[token("=", lex_proc)]
    #[token("+", lex_proc)]
    #[token("-", lex_proc)]
    #[token("*", lex_proc)]
    #[token("/", lex_proc)]
    #[token("%", lex_proc)]
    Proc(Procedure),

    #[token("and", lex_comp)]
    #[token("or", lex_comp)]
    #[token("xor", lex_comp)]
    #[token("not", lex_comp)]
    #[token("eq", lex_comp)]
    #[token("neq", lex_comp)]
    #[token("gt", lex_comp)]
    #[token("gte", lex_comp)]
    #[token("lt", lex_comp)]
    #[token("lte", lex_comp)]
    Cmp(Comparison),

    #[token("//")]
    #[token("/*")]
    #[token("*/")]
    Comment(String),

    #[regex("@([_a-z=A-Z]+)", stringify)]
    Var(String),

    #[regex(
        r#"(( )?[a-zA-Z0-9\$%^*\-_\+\[\]\\./:'"]+)([ a-zA-Z0-9\$%^*\-_\+\[\]\\./:';&"]+)*"#,
        stringify
    )]
    Str(String),

    #[error]
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Error,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Procedure {
    Const,
    Assign,
    Add,
    Sub,
    Mul,
    Div,
    Mod,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Comparison {
    And,
    Or,
    Xor,
    Not,
    Equal,
    NotEqual,
    Gt,
    Gte,
    Lt,
    Lte,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}

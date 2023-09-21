use std::fmt::Display;

use crate::{
    interpreter::Object,
    token::{self, Literal, Token},
};

#[derive(Debug, Clone)]
pub enum Expr {
    Unary(Token, Box<Expr>),
    Binary(Box<Expr>, Token, Box<Expr>),
    Logical(Box<Expr>, Token, Box<Expr>),
    Grouping(Box<Expr>),
    Literal(Literal),
    Var(Token),
    Assign(Token, Box<Expr>),
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Binary(left, op, right) => {
                write!(f, "({} {} {})", op, left, right)
            }
            Expr::Unary(op, value) => {
                write!(f, "({}{})", op, value)
            }
            Expr::Grouping(expr) => {
                write!(f, "(group {})", expr)
            }
            Expr::Literal(value) => {
                write!(f, "{}", value)
            }
            Expr::Var(token) => write!(f, "(var {})", token.lexeme),
            Expr::Assign(_token, value) => write!(f, "(= {})", value),
            Expr::Logical(left, op, right) => {
                write!(f, "({} {} {})", op, left, right)
            }
        }
    }
}

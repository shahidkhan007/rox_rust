use std::fs;
use std::io::{stdin, stdout, Write};
use std::str::from_utf8;

use expression::Expr;
use interpreter::Interpreter;
use parser::Parser;
use scanner::Scanner;
use token::{Token, TokenType};

use crate::error::{Log, LogLevel};
use crate::token::Literal;

mod core;
mod env;
mod error;
mod expression;
mod interpreter;
mod keywords;
mod parser;
mod scanner;
mod statement;
mod token;

fn main() {
    let logger = Log {
        level: LogLevel::Debug,
    };

    let source = fs::read_to_string("source.rox").unwrap();

    let mut s = Scanner::new(source, logger);

    s.scan_tokens();

    let mut parser = Parser::new(s.tokens.clone(), &logger);

    let stmts = parser.parse().unwrap();

    let mut interpreter = Interpreter::new(logger);

    interpreter.interpret(stmts);
}

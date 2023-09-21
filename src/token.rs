use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // Single-character tokens.
    LEFT_PAREN,
    RIGHT_PAREN,
    LEFT_BRACE,
    RIGHT_BRACE,
    COMMA,
    DOT,
    MINUS,
    PLUS,
    SEMICOLON,
    SLASH,
    STAR,

    // One or two character tokens.
    BANG,
    BANG_EQUAL,
    EQUAL,
    EQUAL_EQUAL,
    GREATER,
    GREATER_EQUAL,
    LESS,
    LESS_EQUAL,

    // Literals.
    IDENTIFIER,
    STRING,
    NUMBER,

    // Keywords.
    AND,
    CLASS,
    ELSE,
    FALSE,
    FUN,
    FOR,
    IF,
    NIL,
    OR,
    PRINT,
    RETURN,
    SUPER,
    THIS,
    TRUE,
    VAR,
    WHILE,

    EOF,
}

#[derive(Debug, Clone)]
pub enum Literal {
    Number(f64),
    String(String),
    Nil,
    Bool(bool),
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: i32,
    pub literal: Literal,
}

impl Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Literal::Number(n) => write!(f, "{}", n),
            Literal::String(s) => write!(f, "{}", s),
            Literal::Bool(b) => write!(f, "{}", b),
            Literal::Nil => write!(f, "nil"),
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.literal {
            Literal::Number(_n) => Display::fmt(&self.literal, f),
            Literal::Nil => match self.token_type {
                TokenType::PLUS => write!(f, "+"),
                TokenType::STAR => write!(f, "*"),
                TokenType::MINUS => write!(f, "-"),
                TokenType::SLASH => write!(f, "/"),
                TokenType::EQUAL_EQUAL => write!(f, "=="),
                TokenType::TRUE => write!(f, "True"),
                TokenType::FALSE => write!(f, "False"),
                TokenType::BANG => write!(f, "!"),
                _ => write!(f, "Not implemented"),
            },
            _ => {
                write!(f, "Not implemented")
            }
        }
    }
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, line: i32, literal: Literal) -> Token {
        Token {
            token_type,
            lexeme,
            line,
            literal,
        }
    }
}

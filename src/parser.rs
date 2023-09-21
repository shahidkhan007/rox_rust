use std::process::{exit, ExitCode};

use crate::{
    error::Log,
    expression::Expr,
    interpreter::Object,
    statement::Stmt,
    token::{self, Token, TokenType},
};

#[derive(Debug)]
pub enum ParseError {
    Generic(String),
}

pub struct Parser<'a> {
    tokens: Vec<Token>,
    current: usize,
    logger: &'a Log,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: Vec<Token>, logger: &'a Log) -> Parser {
        Parser {
            tokens,
            current: 0,
            logger,
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Stmt>, ParseError> {
        let mut statements: Vec<Stmt> = Vec::new();

        while !self.is_at_end() {
            let stmt = self.declaration();
            statements.push(stmt);
        }

        Ok(statements)
    }

    fn declaration(&mut self) -> Stmt {
        if self.matches(vec![TokenType::VAR]) {
            self.var_decl()
        } else {
            self.statement()
        }
    }

    fn var_decl(&mut self) -> Stmt {
        let ident = self
            .consume(TokenType::IDENTIFIER, "Expected a variable name")
            .unwrap();

        let initializer: Option<Expr>;

        if self.matches(vec![TokenType::EQUAL]) {
            initializer = Some(self.expression());
        } else {
            initializer = Some(Expr::Literal(token::Literal::Nil));
        }

        self.consume(
            TokenType::SEMICOLON,
            "Expected a ';' after variable declaration",
        )
        .unwrap();

        return Stmt::Var(ident, initializer);
    }

    fn statement(&mut self) -> Stmt {
        if self.matches(vec![TokenType::PRINT]) {
            return self.print_statement();
        } else if self.matches(vec![TokenType::IF]) {
            return self.if_statement();
        } else if self.matches(vec![TokenType::LEFT_BRACE]) {
            return self.block_statement();
        } else if self.matches(vec![TokenType::WHILE]) {
            return self.while_statement();
        } else {
            return self.expr_statement();
        }
    }

    fn if_statement(&mut self) -> Stmt {
        self.consume(
            TokenType::LEFT_PAREN,
            "Expected a '(' after the if statement.",
        )
        .unwrap();
        let condition = self.expression();
        self.consume(
            TokenType::RIGHT_PAREN,
            "Expected a ')' after the if condition.",
        )
        .unwrap();

        let then_branch = self.statement();
        let else_branch = match self.matches(vec![TokenType::ELSE]) {
            true => Some(self.statement()),
            false => None,
        };

        return Stmt::If(condition, Box::new(then_branch), Box::new(else_branch));
    }

    fn block_statement(&mut self) -> Stmt {
        let mut statements = Vec::new();

        while !self.check(TokenType::RIGHT_BRACE) && !self.is_at_end() {
            statements.push(self.declaration());
        }

        self.consume(TokenType::RIGHT_BRACE, "Expected '}' after the block.")
            .unwrap();

        return Stmt::Block(statements);
    }

    fn while_statement(&mut self) -> Stmt {
        self.consume(
            TokenType::LEFT_PAREN,
            "Expected a '(' after the while keyword.",
        )
        .unwrap();
        let cond = self.expression();
        self.consume(
            TokenType::RIGHT_PAREN,
            "Expected a ')' after the condition.",
        )
        .unwrap();

        let block = self.statement();
        return Stmt::While(cond, Box::new(block));
    }

    fn print_statement(&mut self) -> Stmt {
        let value = self.expression();
        self.consume(
            TokenType::SEMICOLON,
            "Expected ';' after the print statement.",
        )
        .unwrap();
        return Stmt::Print(value);
    }

    fn expr_statement(&mut self) -> Stmt {
        let expr = self.expression();
        self.consume(TokenType::SEMICOLON, "Expected ';' after the expression.")
            .unwrap();
        return Stmt::Expression(expr);
    }

    fn expression(&mut self) -> Expr {
        return self.assignment();
    }

    fn assignment(&mut self) -> Expr {
        let expr = self.or();

        if self.matches(vec![TokenType::EQUAL]) {
            let equals = self.previous();
            let value = self.assignment();

            match expr {
                Expr::Var(token) => Expr::Assign(token, Box::new(value)),
                _ => {
                    self.report_error(equals, "Invalid assignment target");
                    panic!();
                }
            }
        } else {
            expr
        }
    }

    fn or(&mut self) -> Expr {
        let mut expr = self.and();

        while self.matches(vec![TokenType::OR]) {
            let op = self.previous();
            let right = self.and();
            expr = Expr::Logical(Box::new(expr), op, Box::new(right))
        }

        return expr;
    }

    fn and(&mut self) -> Expr {
        let mut expr = self.equality();

        while self.matches(vec![TokenType::AND]) {
            let op = self.previous();
            let right = self.equality();
            expr = Expr::Logical(Box::new(expr), op, Box::new(right))
        }

        return expr;
    }

    fn equality(&mut self) -> Expr {
        let mut expr = self.comparison();

        while self.matches(vec![TokenType::BANG_EQUAL, TokenType::EQUAL_EQUAL]) {
            let op = self.previous();
            let right = self.comparison();
            expr = Expr::Binary(Box::new(expr), op, Box::new(right));
        }

        return expr;
    }

    fn comparison(&mut self) -> Expr {
        let mut expr = self.term();

        while self.matches(vec![
            TokenType::GREATER,
            TokenType::GREATER_EQUAL,
            TokenType::LESS,
            TokenType::LESS_EQUAL,
        ]) {
            let op = self.previous();
            let right = self.term();
            expr = Expr::Binary(Box::new(expr), op, Box::new(right));
        }

        return expr;
    }

    fn term(&mut self) -> Expr {
        let mut expr = self.factor();

        while self.matches(vec![TokenType::MINUS, TokenType::PLUS]) {
            let op = self.previous();
            let right = self.factor();
            expr = Expr::Binary(Box::new(expr), op, Box::new(right));
        }

        return expr;
    }

    fn factor(&mut self) -> Expr {
        let mut expr = self.unary();

        while self.matches(vec![TokenType::STAR, TokenType::SLASH]) {
            let op = self.previous();
            let right = self.unary();
            expr = Expr::Binary(Box::new(expr), op, Box::new(right));
        }

        return expr;
    }

    fn unary(&mut self) -> Expr {
        if self.matches(vec![TokenType::BANG, TokenType::MINUS]) {
            let op = self.previous();
            let right = self.unary();
            return Expr::Unary(op, Box::new(right));
        }

        return self.primary();
    }

    fn primary(&mut self) -> Expr {
        if self.matches(vec![TokenType::FALSE]) {
            return Expr::Literal(token::Literal::Bool(false));
        } else if self.matches(vec![TokenType::TRUE]) {
            return Expr::Literal(token::Literal::Bool(true));
        } else if self.matches(vec![TokenType::NIL]) {
            return Expr::Literal(token::Literal::Nil);
        } else if self.matches(vec![TokenType::NUMBER, TokenType::STRING]) {
            return Expr::Literal(self.previous().literal);
        } else if self.matches(vec![TokenType::LEFT_PAREN]) {
            let expr = self.expression();

            match self.consume(TokenType::RIGHT_PAREN, "Expected ')' after expression.") {
                Ok(_) => {}
                Err(parse_error) => match parse_error {
                    ParseError::Generic(error_message) => {
                        self.logger.error(error_message);
                        exit(1);
                    }
                    _ => {}
                },
            };
            return Expr::Grouping(Box::new(expr));
        } else if self.matches(vec![TokenType::IDENTIFIER]) {
            Expr::Var(self.previous())
        } else {
            let err = self.report_error(self.peek(), "Expected expression.");
            match err {
                ParseError::Generic(error_message) => {
                    self.logger.error(error_message);
                    exit(1);
                }
                _ => {
                    panic!("Unhandled ParseError Arm");
                }
            };
        }
    }

    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().token_type == TokenType::SEMICOLON {
                return;
            }

            match self.peek().token_type {
                TokenType::CLASS => return,
                TokenType::FUN => return,
                TokenType::VAR => return,
                TokenType::FOR => return,
                TokenType::IF => return,
                TokenType::WHILE => return,
                TokenType::PRINT => return,
                TokenType::RETURN => return,
                _ => {
                    self.advance();
                }
            };
        }
    }

    fn consume(&mut self, token_type: TokenType, message: &str) -> Result<Token, ParseError> {
        if self.check(token_type) {
            return Ok(self.advance());
        }

        return Err(self.report_error(self.peek(), message));
    }

    fn matches(&mut self, token_types: Vec<TokenType>) -> bool {
        for t in token_types {
            if self.check(t) {
                self.advance();
                return true;
            }
        }
        return false;
    }

    fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }

        return self.peek().token_type == token_type;
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }

        return self.previous();
    }

    fn is_at_end(&self) -> bool {
        return self.peek().token_type == TokenType::EOF;
    }

    fn peek(&self) -> Token {
        return self.tokens[self.current].clone();
    }

    fn previous(&mut self) -> Token {
        return self.tokens[self.current - 1].clone();
    }

    fn report_error(&self, token: Token, message: &str) -> ParseError {
        if token.token_type == TokenType::EOF {
            ParseError::Generic(format!(
                "Parse error at line {} at end, {}",
                token.line, message
            ))
        } else {
            ParseError::Generic(format!(
                "Parse error at line {} at '{}', {}",
                token.line, token.lexeme, message
            ))
        }
    }
}

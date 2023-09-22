use std::fmt::Display;

use crate::{
    env::Env,
    error::Log,
    expression::Expr,
    statement::Stmt,
    token::{Literal, Token, TokenType},
};

#[derive(Debug, Clone)]
pub struct Object {
    value: Literal,
}

pub struct Interpreter {
    env: Env,
    logger: Log,
}

impl Interpreter {
    pub fn new(logger: Log) -> Interpreter {
        Interpreter {
            env: Env::new(None),
            logger,
        }
    }

    fn is_truthy(&mut self, obj: Object) -> bool {
        match obj.value {
            Literal::Bool(x) => x,
            Literal::Number(x) => x == 0.0,
            Literal::String(x) => x.len() == 0,
            Literal::Nil => false,
        }
    }

    fn eval_literal(&mut self, lit_val: Literal) -> Object {
        Object { value: lit_val }
    }

    fn eval_group(&mut self, g_val: Expr) -> Object {
        match g_val {
            Expr::Literal(lit_val) => self.eval_literal(lit_val),
            _ => self.eval_expr(g_val),
        }
    }

    fn eval_unary(&mut self, op: Token, right: Box<Expr>) -> Object {
        let right = self.eval_expr(*right);

        match op.token_type {
            TokenType::MINUS => match right.value {
                Literal::Number(x) => Object {
                    value: Literal::Number(x * -1.0),
                },
                x => {
                    panic!("Cannot apply {:?} to a non-number '{}'", op.token_type, x);
                }
            },
            TokenType::BANG => {
                let obj_val = match right.value {
                    Literal::Bool(x) => !x,
                    // Literal::Object => false,
                    Literal::String(x) => x.len() > 0,
                    Literal::Nil => false,
                    Literal::Number(x) => x == 0.0,
                };

                Object {
                    value: Literal::Bool(obj_val),
                }
            }
            x => {
                panic!("Cannot apply {:?} to '{:?}'", x, right.value);
            }
        }
    }

    fn eval_binary(&mut self, left: Expr, op: Token, right: Expr) -> Object {
        let left = self.eval_expr(left);
        let right = self.eval_expr(right);

        let value = match op.token_type {
            TokenType::MINUS => {
                if let Literal::Number(lvalue) = left.value {
                    if let Literal::Number(rvalue) = right.value {
                        Literal::Number(lvalue - rvalue)
                    } else {
                        self.logger.error(format!(
                            "Cannot apply - to '{}' and '{}'",
                            left.value, right.value
                        ));
                        panic!();
                    }
                } else {
                    self.logger.error(format!(
                        "Cannot apply - to '{}' and '{}'",
                        left.value, right.value
                    ));
                    panic!();
                }
            }
            TokenType::PLUS => {
                if let Literal::Number(lvalue) = left.value {
                    if let Literal::Number(rvalue) = right.value {
                        Literal::Number(lvalue + rvalue)
                    } else {
                        self.logger.error(format!(
                            "Cannot apply + to '{}' and '{}'",
                            left.value, right.value
                        ));
                        panic!();
                    }
                } else {
                    self.logger.error(format!(
                        "Cannot apply + to '{}' and '{}'",
                        left.value, right.value
                    ));
                    panic!();
                }
            }
            TokenType::STAR => {
                if let Literal::Number(lvalue) = left.value {
                    if let Literal::Number(rvalue) = right.value {
                        Literal::Number(lvalue * rvalue)
                    } else {
                        self.logger.error(format!(
                            "Cannot apply * to '{}' and '{}'",
                            left.value, right.value
                        ));
                        panic!();
                    }
                } else {
                    self.logger.error(format!(
                        "Cannot apply * to '{}' and '{}'",
                        left.value, right.value
                    ));
                    panic!();
                }
            }
            TokenType::SLASH => {
                if let Literal::Number(lvalue) = left.value {
                    if let Literal::Number(rvalue) = right.value {
                        if rvalue == 0.0 {
                            self.logger.error(format!("Cannot divide by zero"));
                            panic!();
                        }
                        Literal::Number(lvalue / rvalue)
                    } else {
                        self.logger.error(format!(
                            "Cannot apply / to '{}' and '{}'",
                            left.value, right.value
                        ));
                        panic!();
                    }
                } else {
                    self.logger.error(format!(
                        "Cannot apply / to '{}' and '{}'",
                        left.value, right.value
                    ));
                    panic!();
                }
            }
            TokenType::LESS => match left.value {
                Literal::Number(x) => match right.value {
                    Literal::Number(y) => Literal::Bool(x < y),
                    _ => {
                        self.logger.error("Cannot compare non-numbers.".into());
                        panic!();
                    }
                },
                _ => {
                    self.logger.error("Cannot compare non-numbers.".into());
                    panic!();
                }
            },
            TokenType::EQUAL_EQUAL => match left.value {
                Literal::Number(x) => match right.value {
                    Literal::Number(y) => Literal::Bool(x == y),
                    _ => {
                        self.logger.error("Cannot compare non-numbers.".into());
                        panic!();
                    }
                },
                _ => {
                    self.logger.error("Cannot compare non-numbers.".into());
                    panic!();
                }
            },
            TokenType::GREATER => match left.value {
                Literal::Number(x) => match right.value {
                    Literal::Number(y) => Literal::Bool(x > y),
                    _ => {
                        self.logger.error("Cannot compare non-numbers.".into());
                        panic!();
                    }
                },
                _ => {
                    self.logger.error("Cannot compare non-numbers.".into());
                    panic!();
                }
            },
            TokenType::GREATER_EQUAL => match left.value {
                Literal::Number(x) => match right.value {
                    Literal::Number(y) => Literal::Bool(x >= y),
                    _ => {
                        self.logger.error("Cannot compare non-numbers.".into());
                        panic!();
                    }
                },
                _ => {
                    self.logger.error("Cannot compare non-numbers.".into());
                    panic!();
                }
            },
            x => {
                panic!("No such operator as {:?}", x);
            }
        };

        Object { value }
    }

    fn eval_logical(&mut self, left: Expr, op: Token, right: Expr) -> Object {
        let left_val = self.eval_expr(left);

        let is_op_or = match op.token_type {
            TokenType::OR => true,
            _ => false,
        };

        if is_op_or {
            if self.is_truthy(left_val.clone()) {
                return left_val;
            }
        } else {
            if !self.is_truthy(left_val.clone()) {
                return left_val;
            }
        }

        return self.eval_expr(right);
    }

    fn assign_expr(&mut self, token: Token, expr: Expr) -> Object {
        let expr_val = self.eval_expr(expr);

        self.env.assign(token.lexeme, expr_val).unwrap();

        Object {
            value: Literal::Nil,
        }
    }

    fn eval_expr(&mut self, expr: Expr) -> Object {
        match expr {
            Expr::Literal(lit_val) => self.eval_literal(lit_val),
            Expr::Grouping(inner) => self.eval_group(*inner),
            Expr::Unary(op, right) => self.eval_unary(op, right),
            Expr::Binary(left, op, right) => self.eval_binary(*left, op, *right),
            Expr::Var(var) => self.env.get(var.lexeme).unwrap(),
            Expr::Assign(token, expr) => self.assign_expr(token, *expr),
            Expr::Logical(left, op, right) => self.eval_logical(*left, op, *right),
        }
    }

    fn eval_var_expr(&mut self, token: Token, initializer: Object) -> Object {
        self.env.define(token.lexeme, initializer).unwrap();

        Object {
            value: Literal::Nil,
        }
    }

    fn exec_block(&mut self, statements: Vec<Stmt>) -> Object {
        let local_env = Env::new(Some(self.env.clone()));
        self.env = local_env;

        for stmt in statements.into_iter() {
            self.execute(stmt);
        }

        self.env = self.env.get_enclosing().unwrap();

        Object {
            value: Literal::Nil,
        }
    }

    fn eval_if(&mut self, condition: Expr, then_block: Stmt, else_block: Option<Stmt>) {
        let cond_val = self.eval_expr(condition);

        if self.is_truthy(cond_val) {
            self.execute(then_block);
        } else if else_block.is_some() {
            self.execute(else_block.unwrap());
        }
    }

    fn exec_while(&mut self, cond: Expr, block: Stmt) {
        let mut cond_val = self.eval_expr(cond.clone());

        while self.is_truthy(cond_val.clone()) {
            self.execute(block.clone());
            cond_val = self.eval_expr(cond.clone());
        }
    }

    pub fn execute(&mut self, stmt: Stmt) {
        match stmt {
            Stmt::Expression(expr) => Some(self.eval_expr(expr)),
            Stmt::Print(expr) => {
                let value = self.eval_expr(expr);
                println!("{}", value);
                None
            }
            Stmt::Var(token, initializer) => {
                let init = match initializer {
                    Some(expr) => self.eval_expr(expr),
                    None => Object {
                        value: Literal::Nil,
                    },
                };

                self.eval_var_expr(token, init);

                None
            }
            Stmt::Block(statements) => {
                self.exec_block(statements);
                None
            }
            Stmt::If(condition, then_block, else_block) => {
                self.eval_if(condition, *then_block, *else_block);
                None
            }
            Stmt::While(cond, block) => {
                self.exec_while(cond, *block);
                None
            }
        };
    }

    pub fn interpret(&mut self, stmts: Vec<Stmt>) {
        for stmt in stmts.into_iter() {
            self.execute(stmt);
        }
    }
}

impl Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.value.clone() {
            Literal::Bool(x) => write!(f, "{}", x),
            Literal::Number(x) => write!(f, "{}", x),
            Literal::Nil => write!(f, "nil"),
            Literal::String(x) => write!(f, "{}", x),
        }
    }
}

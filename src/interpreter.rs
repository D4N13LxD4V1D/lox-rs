use crate::{token::{TokenType}, error_handling::error};
use crate::ast::{Expression, Statement};
use crate::environment::{Environment, Value};

fn print(expr: &Expression, env: &Environment) {
    match evaluate(expr, env) {
        Value::Number(n) => println!("{}", n),
        Value::String(s) => println!("{}", s),
        Value::Boolean(b) => println!("{}", b),
        Value::Null => println!("null"),
    }
}

pub fn execute(statements: &Vec<Statement>, env: &mut Environment) {
    for statement in statements {
        match statement {
            Statement::Expression(expr) => { evaluate(&expr, env); () },
            Statement::Print(expr) => { print(&expr, env); () },
            Statement::Var(token, expr) => {
                let val = evaluate(&expr, env);
                env.define(token.lexeme.clone(), val);
                ()
            }
        }
    }
}

pub fn evaluate(expr: &Expression, env: &Environment) -> Value {
    match expr {
        Expression::Binary(left, op, right) => {
            let left = evaluate(&*left, env);
            let right = evaluate(&*right, env);

            match (left, right) {
                (Value::Number(left), Value::Number(right)) => {
                    match op.token_type {
                        TokenType::PLUS => Value::Number(left + right),
                        TokenType::MINUS => Value::Number(left - right),
                        TokenType::SLASH => Value::Number(left / right),
                        TokenType::STAR => Value::Number(left * right),
                        TokenType::GREATER => Value::Boolean(left > right),
                        TokenType::GREATER_EQUAL => Value::Boolean(left >= right),
                        TokenType::LESS => Value::Boolean(left < right),
                        TokenType::LESS_EQUAL => Value::Boolean(left <= right),
                        TokenType::BANG_EQUAL => Value::Boolean(left != right),
                        TokenType::EQUAL_EQUAL => Value::Boolean(left == right),
                        _ => {error(op.line, op.index, "invalid operator"); Value::Null},
                    }
                }
                (Value::String(left), Value::String(right)) => {
                    match op.token_type {
                        TokenType::PLUS => Value::String(left.clone() + &right),
                        _ => {error(op.line, op.index, "invalid operator"); Value::String("".to_string())},
                    }
                },
                _ => {error(op.line, op.index, "invalid operands"); Value::Null},
            }
        }
        Expression::Grouping(expr) => evaluate(&*expr, env),
        Expression::Literal(value) => {
            match value.token_type {
                TokenType::STRING => Value::String(value.lexeme.clone()),
                TokenType::NUMBER => Value::Number(value.lexeme.parse::<f64>().unwrap()),
                TokenType::TRUE => Value::Boolean(true),
                TokenType::FALSE => Value::Boolean(false),
                TokenType::NIL => Value::Null,
                _ => {error(value.line, value.index, "invalid literal"); Value::Null},
            }
        },
        Expression::Unary(op, expr) => {
            let right = evaluate(&*expr, env);

            match right {
                Value::Number(right) => {
                    match op.token_type {
                        TokenType::MINUS => Value::Number(-right),
                        _ => {error(op.line, op.index, "invalid operator"); Value::Null},
                    }
                },
                Value::Boolean(right) => {
                    match op.token_type {
                        TokenType::BANG => Value::Boolean(!right),
                        _ => {error(op.line, op.index, "invalid operator"); Value::Null},
                    }
                },
                _ => {error(op.line, op.index, "invalid operands"); Value::Null},
            }
        },
        Expression::Variable(token) => {
            let val = env.get(&token);
            val.clone()
        }
    }
}
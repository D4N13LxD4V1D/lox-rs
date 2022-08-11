use crate::{expression::Expression, token::TokenType, error_handling::error};

pub enum Value {
    Number(f64),
    String(String),
    Boolean(bool),
}

impl Value {
    pub fn unwrap_num(self: &Self) -> f64 {
        match self {
            Value::Number(n) => *n,
            _ => {error(0, 0, "expected number"); f64::NAN}
        }
    }

    pub fn unwrap_str(self: &Self) -> String {
        match self {
            Value::String(n) => n.clone(),
            _ => {error(0, 0, "expected string"); "".to_string()}
        }
    }

    pub fn unwrap_bool(self: &Self) -> bool{
        match self {
            Value::Boolean(n) => *n,
            _ => {error(0, 0, "expected boolean"); bool::default()}
        }
    }
}

pub fn print(expr: &Expression) {
    match evaluate(expr) {
        Value::Number(n) => println!("{}", n),
        Value::String(s) => println!("{}", s),
        Value::Boolean(b) => println!("{}", b),
    }
}

pub fn evaluate(expr: &Expression) -> Value {
    match expr {
        Expression::Binary(left, op, right) => {
            let left = evaluate(&*left);
            let right = evaluate(&*right);

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
                        _ => {error(op.line, op.index, "invalid operator"); Value::Number(f64::NAN)},
                    }
                }
                _ => {error(op.line, op.index, "invalid operands"); Value::Number(f64::NAN)},
            }
        }
        Expression::Grouping(expr) => evaluate(&*expr),
        Expression::Literal(value) => {
            match value.token_type {
                TokenType::STRING => Value::String(value.lexeme.clone()),
                TokenType::NUMBER => Value::Number(value.lexeme.parse::<f64>().unwrap()),
                TokenType::TRUE => Value::Boolean(true),
                TokenType::FALSE => Value::Boolean(false),
                _ => {error(value.line, value.index, "invalid literal"); Value::Number(f64::NAN)},
            }
        },
        Expression::Unary(op, expr) => {
            let right = evaluate(&*expr);

            match right {
                Value::Number(right) => {
                    match op.token_type {
                        TokenType::MINUS => Value::Number(-right),
                        _ => {error(op.line, op.index, "invalid operator"); Value::Number(f64::NAN)},
                    }
                },
                Value::Boolean(right) => {
                    match op.token_type {
                        TokenType::BANG => Value::Boolean(!right),
                        _ => {error(op.line, op.index, "invalid operator"); Value::Number(f64::NAN)},
                    }
                },
                _ => {error(op.line, op.index, "invalid operands"); Value::Number(f64::NAN)},
            }
        }
    }
}
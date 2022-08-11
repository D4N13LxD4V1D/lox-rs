use std::collections::HashMap;

use crate::{token::Token, error_handling::error};
#[derive(Debug, Clone)]
pub enum Expression {
    Literal(Token),
    Unary(Token, Box<Expression>),
    Binary(Box<Expression>, Token, Box<Expression>),
    Grouping(Box<Expression>),
    Variable(Token)
}

#[derive(Debug, Clone)]
pub enum Statement {
    Expression(Expression),
    Print(Expression),
    Var(Token, Expression)
}

pub struct State {
    pub tokens: Vec<Token>,
    pub current: usize,
}
#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    String(String),
    Boolean(bool),
    Null,
}

pub struct Environment {
    values: HashMap<String, Value>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            values: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: String, value: Value) {
        self.values.insert(name, value);
    }

    pub fn get(&self, token: &Token) -> Value {
        if !self.values.contains_key(token.lexeme.as_str()) {
            error(token.line, token.index, &("undefined variable '".to_owned() + token.lexeme.as_str() + "'"));
            return Value::Null;
        }
        let val = self.values.get(token.lexeme.as_str()).unwrap();
        val.clone()
    }
}
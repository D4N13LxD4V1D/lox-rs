use std::io::{self, Write};

use crate::token::{TokenType::{*, self}, Token};
use crate::error_handling;

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

struct State {
    tokens: Vec<Token>,
    current: usize,
}

pub fn print(statements: &Vec<Statement>) {
    for stmt in statements {
        match stmt {
            Statement::Print(e) => {
                _print(&Expression::Unary(Token::new(PRINT, "print".to_string(), vec!['p', 'r', 'i', 'n', 't'], 0, 1), Box::from(e.clone())));
            },
            Statement::Expression(e) => {
                _print(e);
            },
            Statement::Var(t, e) => {
                _print(&Expression::Binary(Box::from(Expression::Variable(t.clone())), Token::new(VAR, "=".to_string(), vec!['='], 0, 1), Box::from(e.clone())));
            }
        }
    }
}

fn _print(expr: &Expression) {
    match expr {
        Expression::Binary(left, op, right) => {
            print!("\x1b[1;34m( \x1b[0m{} ", op.lexeme); io::stdout().flush().unwrap();
            _print(&*left); io::stdout().flush().unwrap();
            _print(&*right); io::stdout().flush().unwrap();
            print!("\x1b[1;34m)\x1b[0m "); io::stdout().flush().unwrap();
        },
        Expression::Grouping(expr) => {
            print!(" \x1b[1;35m( \x1b[0m"); io::stdout().flush().unwrap();
            _print(&*expr); io::stdout().flush().unwrap();
            print!("\x1b[1;35m)\x1b[0m "); io::stdout().flush().unwrap();
        },
        Expression::Literal(value) => {
            print!("{} ", value.lexeme); io::stdout().flush().unwrap();
        },
        Expression::Unary(op, expr) => {
            print!(" \x1b[1;33m( \x1b[0m{} ", op.lexeme); io::stdout().flush().unwrap();
            _print(&*expr); io::stdout().flush().unwrap();
            print!("\x1b[1;33m)\x1b[0m "); io::stdout().flush().unwrap();
        },
        Expression::Variable(value) => {
            print!("{} ", value.lexeme); io::stdout().flush().unwrap();
        }
    }
}

pub fn parse(tokens: Vec<Token>) -> Vec<Statement> {
    let mut state = State {
        tokens,
        current: 0,
    };

    let mut statements: Vec<Statement> = Vec::new();

    while !is_at_end(&mut state) {
        statements.push(declaration(&mut state));
    }
    
    statements
}

fn declaration(state: &mut State) -> Statement {
    if _match(state, vec![VAR]) {
        return var_declaration(state);
    }

    statement(state)
}

fn var_declaration(state: &mut State) -> Statement {
    let name = consume(state, IDENTIFIER, "variable");
    
    let initializer;
    if _match(state, vec![EQUAL]) {
        initializer = expression(state);
    } else {
        initializer = Expression::Literal(Token::new(NIL, "nil".to_string(), vec!['n', 'i', 'l'], 0, 1));
    }
    consume(state, SEMICOLON, ";");
    Statement::Var(name, initializer)
}

fn statement(state: &mut State) -> Statement {
    if _match(state, vec![PRINT]) {
        return print_stmt(state);
    }
    expr_stmt(state)
}

fn print_stmt(state: &mut State) -> Statement {
    let value = expression(state);
    consume(state, SEMICOLON, ";");
    Statement::Print(value)
}

fn expr_stmt(state: &mut State) -> Statement {
    let expr = expression(state);
    consume(state, SEMICOLON, ";");
    Statement::Expression(expr)
}

fn expression(state: &mut State) -> Expression {
    equality(state)
}

fn equality(state: &mut State) -> Expression {
    let mut expr = comparison(state);

    while _match(state, vec![BANG_EQUAL, EQUAL_EQUAL]) {
        let operator = previous(state);
        let right = comparison(state);
        expr = Expression::Binary(Box::new(expr), operator, Box::new(right));
    }

    expr
}

fn comparison(state: &mut State) -> Expression {
    let mut expr = term(state);

    while _match(state, vec![GREATER, GREATER_EQUAL, LESS, LESS_EQUAL]) {
        let operator = previous(state);
        let right = term(state);
        expr = Expression::Binary(Box::new(expr), operator, Box::new(right));
    }

    expr
}

fn term(state: &mut State) -> Expression {
    let mut expr = factor(state);

    while _match(state, vec![MINUS, PLUS]) {
        let operator = previous(state);
        let right = factor(state);
        expr = Expression::Binary(Box::new(expr), operator, Box::new(right));
    }

    expr
}

fn factor(state: &mut State) -> Expression {
    let mut expr = unary(state);

    if _match(state, vec![SLASH, STAR]) {
        let operator = previous(state);
        let right = unary(state);
        expr = Expression::Binary(Box::new(expr), operator, Box::new(right));
    }

    expr
}

fn unary(state: &mut State) -> Expression {
    if _match(state, vec![BANG, MINUS]) {
        let operator = previous(state);
        let right = unary(state);
        return Expression::Unary(operator, Box::new(right));
    }

    primary(state)
}

fn primary(state: &mut State) -> Expression {
    if _match(state, vec![FALSE]) {
        return Expression::Literal(previous(state));
    }

    if _match(state, vec![TRUE]) {
        return Expression::Literal(previous(state));
    }
    
    if _match(state, vec![NIL]) {
        return Expression::Literal(previous(state));
    }
    
    if _match(state, vec![NUMBER]) {
        return Expression::Literal(previous(state));
    }
    
    if _match(state, vec![STRING]) {
        return Expression::Literal(previous(state));
    }
    
    if _match(state, vec![IDENTIFIER]) {
        return Expression::Variable(previous(state));
    }
    
    if _match(state, vec![LEFT_PAREN]) {
        let expr = expression(state);
        consume(state, RIGHT_PAREN, ")");
        return Expression::Grouping(Box::new(expr));
    }
    let token = &state.tokens[state.current];
    error_handling::error(token.line, token.index, "expected expression");
    Expression::Literal(Token::new(NIL, "".to_string(), vec![], token.line, token.index))
}

fn consume(state: &mut State, token_type: TokenType, expected: &str) -> Token {
    if check(state, token_type) {
        advance(state)
    } else {
        let token = &state.tokens[state.current];
        error_handling::error(token.line, token.index, &("expecting '".to_owned() + expected + "'"));
        advance(state)
    }
}

fn synchronize(state: &mut State) {
    advance(state);
    
    while !is_at_end(state) {
        if check(state, SEMICOLON) {
            return
        }

        match peek(state).token_type {
            CLASS | FUN | VAR | FOR | IF | WHILE | PRINT | RETURN => return,
            _ => {advance(state);},
        }
    }

    return
}

fn _match(state: &mut State, types: Vec<TokenType>) -> bool {
    for token_type in types {
        if check(state, token_type) {
            advance(state);
            return true;
        }
    }

    false
}

fn check(state: &mut State, token_type: TokenType) -> bool {
    if is_at_end(state) {
        return false;
    }
    peek(state).token_type == token_type
}

fn advance(state: &mut State) -> Token {
    if !is_at_end(state) {
        state.current += 1;
    }

    previous(state)
}

fn is_at_end(state: &mut State) -> bool {
    peek(state).token_type == EOF
}

fn peek(state: &mut State) -> Token {
    state.tokens[state.current].clone()
}

fn previous(state: &mut State) -> Token {
    state.tokens[state.current - 1].clone()
}
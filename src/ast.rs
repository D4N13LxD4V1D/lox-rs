use crate::expression::Expression;
use crate::token::{TokenType::{*, self}, Token};
use crate::error_handling;
struct State {
    tokens: Vec<Token>,
    current: usize,
}

pub fn parse(tokens: Vec<Token>) -> Expression {
    let mut state = State {
        tokens,
        current: 0,
    };
    
    expression(&mut state)
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
        return Expression::Literal(previous(state));
    }
    
    if _match(state, vec![LEFT_PAREN]) {
        let expr = expression(state);
        consume(state, RIGHT_PAREN, "Expect ')' after expression.");
        return Expression::Grouping(Box::new(expr));
    }
    let token = &state.tokens[state.current];
    error_handling::error(token.line, token.index, "Expect expression");
    Expression::Literal(Token::new(NIL, "".to_string(), vec![], token.line, token.index))
}

fn consume(state: &mut State, token_type: TokenType, message: &str) -> Token {
    if check(state, token_type) {
        advance(state)
    } else {
        let token = &state.tokens[state.current];
        error_handling::error(token.line, token.index, "Syntax error");
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
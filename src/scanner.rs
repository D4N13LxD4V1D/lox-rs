use crate::token::*;
use crate::error_handling::error;

pub fn scan(source: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut line = 0;
    let mut word_index = 0;

    let mut i = 0;
    let source = source.chars().collect::<Vec<char>>();
    while i < source.len() {
        let mut literal = Vec::new();
        literal.push(source[i]);

        match source[i] {
            '/' => {
                if i + 1 < source.len() && source[i + 1] == '/' {
                    while i + 1 < source.len() && source[i + 1] != '\n' {
                        i += 1;
                    }
                }

                if i + 1 < source.len() && source[i + 1] == '*' {
                    while i + 2 < source.len() && source[i + 1] != '*' && source[i + 2] != '/' {
                        i += 1;
                    }
                }

                tokens.push(Token::new(TokenType::SLASH, "/".to_string(), literal, line, word_index))
            },
            '{' => tokens.push(Token::new(TokenType::LEFT_BRACE, "{".to_string(), literal, line, word_index)),
            '}' => tokens.push(Token::new(TokenType::RIGHT_BRACE, "}".to_string(), literal, line, word_index)),
            '(' => tokens.push(Token::new(TokenType::LEFT_PAREN, "(".to_string(), literal, line, word_index)),
            ')' => tokens.push(Token::new(TokenType::RIGHT_PAREN, ")".to_string(), literal, line, word_index)),
            ',' => tokens.push(Token::new(TokenType::COMMA, ",".to_string(), literal, line, word_index)),
            '.' => tokens.push(Token::new(TokenType::DOT, ".".to_string(), literal, line, word_index)),
            '-' => tokens.push(Token::new(TokenType::MINUS, "-".to_string(), literal, line, word_index)),
            '+' => tokens.push(Token::new(TokenType::PLUS, "+".to_string(), literal, line, word_index)),
            ';' => tokens.push(Token::new(TokenType::SEMICOLON, ";".to_string(), literal, line, word_index)),
            '*' => tokens.push(Token::new(TokenType::STAR, "*".to_string(), literal, line, word_index)),
            '!' => {
                if i + 1 < source.len() && source[i + 1] == '=' {
                    tokens.push(Token::new(TokenType::BANG_EQUAL, "!=".to_string(), literal, line, word_index));
                    i += 1;
                } else {
                    tokens.push(Token::new(TokenType::BANG, "!".to_string(), literal, line, word_index));
                }
            },
            '=' => {
                if i + 1 < source.len() && source[i + 1] == '=' {
                    tokens.push(Token::new(TokenType::EQUAL_EQUAL, "==".to_string(), literal, line, word_index));
                    i += 1;
                } else {
                    tokens.push(Token::new(TokenType::EQUAL, "=".to_string(), literal, line, word_index));
                }
            },
            '>' => {
                if i + 1 < source.len() && source[i + 1] == '=' {
                    tokens.push(Token::new(TokenType::GREATER_EQUAL, ">=".to_string(), literal, line, word_index));
                    i += 1;
                } else {
                    tokens.push(Token::new(TokenType::GREATER, ">".to_string(), literal, line, word_index));
                }
            },
            '<' => {
                if i + 1 < source.len() && source[i + 1] == '=' {
                    tokens.push(Token::new(TokenType::LESS_EQUAL, "<=".to_string(), literal, line, word_index));
                    i += 1;
                } else {
                    tokens.push(Token::new(TokenType::LESS, "<".to_string(), literal, line, word_index));
                }
            },
            ' ' => {
                word_index += 1;
            },
            '\t' => {
                word_index += 1;
            },
            '\n' => {
                line += 1;
                word_index = 0;
            },
            '\r' => {
                line += 1;
                word_index = 0;
            },
            '"' => {
                literal.pop();
                while i + 1 < source.len() && source[i + 1] != '"' {
                    literal.push(source[i + 1]);
                    i += 1;
                }

                if i + 1 == source.len() {
                    error(line, word_index, "Unterminated string");
                }

                i += 1;
                tokens.push(Token::new(TokenType::STRING, literal.iter().collect::<String>(), literal, line, word_index));
            },
            '0'..='9' => {
                while i + 1 < source.len() && source[i + 1] >= '0' && source[i + 1] <= '9' {
                    literal.push(source[i + 1]);
                    i += 1;
                }
                tokens.push(Token::new(TokenType::NUMBER, literal.iter().collect::<String>(), literal, line, word_index));
            },
            'a'..='z' | 'A'..='Z' | '_' => {
                while i + 1 < source.len() && (source[i + 1] >= 'a' && source[i + 1] <= 'z') || (source[i + 1] >= 'A' && source[i + 1] <= 'Z') || source[i + 1] == '_' {
                    literal.push(source[i + 1]);
                    i += 1;
                }

                match literal.iter().collect::<String>().as_ref() {
                    "and" => tokens.push(Token::new(TokenType::AND, "and".to_string(), literal, line, word_index)),
                    "class" => tokens.push(Token::new(TokenType::CLASS, "class".to_string(), literal, line, word_index)),
                    "else" => tokens.push(Token::new(TokenType::ELSE, "else".to_string(), literal, line, word_index)),
                    "false" => tokens.push(Token::new(TokenType::FALSE, "false".to_string(), literal, line, word_index)),
                    "for" => tokens.push(Token::new(TokenType::FOR, "for".to_string(), literal, line, word_index)),
                    "fn" => tokens.push(Token::new(TokenType::FUN, "fun".to_string(), literal, line, word_index)),
                    "if" => tokens.push(Token::new(TokenType::IF, "if".to_string(), literal, line, word_index)),
                    "nul" => tokens.push(Token::new(TokenType::NIL, "nil".to_string(), literal, line, word_index)),
                    "or" => tokens.push(Token::new(TokenType::OR, "or".to_string(), literal, line, word_index)),
                    "print" => tokens.push(Token::new(TokenType::PRINT, "print".to_string(), literal, line, word_index)),
                    "return" => tokens.push(Token::new(TokenType::RETURN, "return".to_string(), literal, line, word_index)),
                    "super" => tokens.push(Token::new(TokenType::SUPER, "super".to_string(), literal, line, word_index)),
                    "this" => tokens.push(Token::new(TokenType::THIS, "this".to_string(), literal, line, word_index)),
                    "true" => tokens.push(Token::new(TokenType::TRUE, "true".to_string(), literal, line, word_index)),
                    "var" => tokens.push(Token::new(TokenType::VAR, "var".to_string(), literal, line, word_index)),
                    "while" => tokens.push(Token::new(TokenType::WHILE, "while".to_string(), literal, line, word_index)),
                    "exit" => std::process::exit(0),
                    "exit()" => std::process::exit(0),
                    _ => tokens.push(Token::new(TokenType::IDENTIFIER, literal.iter().collect::<String>(), literal, line, word_index)),
                }
            },
            _ => error(line, word_index, "Unexpected token"),
        }
        i += 1;
    }

    tokens.push(Token::new(TokenType::EOF, "".to_string(), vec![], line, word_index));

    tokens
}
#![allow(dead_code)]
use std::fmt::{self};

enum TokenType {
    Func,
    Uses,
    Colon,
    Int,
    Double,
    If,
    Then,
    Return,
    Else,
    Done,
    Set,
    To,
    Lt,
    Lte,
    Eq,
    Gt,
    Gte,
    Not,
    Quote,
    OpenParenthesis,
    CloseParenthesis,
    Add,
    Subtract,
    Multiply,
    Divide,
    Percent,
    Power,
    Identifier,
    Comma,
    Newline,
    Integer(i64),
    Float(f64),
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TokenType::Func => write!(f, "Func"),
            TokenType::Uses => write!(f, "Uses"),
            TokenType::Colon => write!(f, "Colon"),
            TokenType::Int => write!(f, "Int"),
            TokenType::Double => write!(f, "Double"),
            TokenType::If => write!(f, "If"),
            TokenType::Then => write!(f, "Then"),
            TokenType::Return => write!(f, "Return"),
            TokenType::Else => write!(f, "Else"),
            TokenType::Done => write!(f, "Done"),
            TokenType::Set => write!(f, "Set"),
            TokenType::To => write!(f, "To"),
            TokenType::Lt => write!(f, "Lt"),
            TokenType::Lte => write!(f, "Lte"),
            TokenType::Eq => write!(f, "Eq"),
            TokenType::Gt => write!(f, "Gt"),
            TokenType::Gte => write!(f, "Gte"),
            TokenType::Not => write!(f, "Not"),
            TokenType::Quote => write!(f, "Quote"),
            TokenType::OpenParenthesis => write!(f, "OpenParenthesis"),
            TokenType::CloseParenthesis => write!(f, "CloseParenthesis"),
            TokenType::Add => write!(f, "Add"),
            TokenType::Subtract => write!(f, "Subtract"),
            TokenType::Multiply => write!(f, "Multiply"),
            TokenType::Divide => write!(f, "Divide"),
            TokenType::Percent => write!(f, "Percent"),
            TokenType::Power => write!(f, "Power"),
            TokenType::Identifier => write!(f, "Identifier"),
            TokenType::Comma => write!(f, "Comma"),
            TokenType::Newline => write!(f, "Newline"),
            TokenType::Integer(n) => write!(f, "integer: {n}"),
            TokenType::Float(n) => write!(f, "integer: {n}"),
        }
    }
}

impl TokenType {
    fn from_str(s: &str) -> TokenType {
        match s.to_lowercase().as_str() {
            "func" => TokenType::Func,
            "uses" => TokenType::Uses,
            ":" => TokenType::Colon,
            "int" => TokenType::Int,
            "double" => TokenType::Double,
            "if" => TokenType::If,
            "then" => TokenType::Then,
            "return" => TokenType::Return,
            "else" => TokenType::Else,
            "done" => TokenType::Done,
            "set" => TokenType::Set,
            "to" => TokenType::To,
            "lt" => TokenType::Lt,
            "lte" => TokenType::Lte,
            "eq" => TokenType::Eq,
            "gt" => TokenType::Gt,
            "gte" => TokenType::Gte,
            "not" => TokenType::Not,
            "\"" => TokenType::Quote,
            "(" => TokenType::OpenParenthesis,
            ")" => TokenType::CloseParenthesis,
            "+" => TokenType::Add,
            "-" => TokenType::Subtract,
            "*" => TokenType::Multiply,
            "/" => TokenType::Divide,
            "%" => TokenType::Percent,
            "^" => TokenType::Power,
            "," => TokenType::Comma,
            _ => TokenType::Identifier,
        }
    }
}

pub struct Token {
    token: TokenType,
    lexeme: String,
}

impl Token {
    fn new(token: TokenType, lexeme: String) -> Token {
        Token { token, lexeme }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "token type: {} | text: {}", self.token, self.lexeme)
    }
}

pub fn get_token(data: String) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut buffer = String::new();

    // newline, number (int or float), identifier (starts with alphanumeric always), and all other tokens
    for c in data.chars() {
        if c == ' ' {
            // save buffer as token + value
            let token_type = TokenType::from_str(&buffer);
            tokens.push(Token::new(token_type, buffer.clone()));
            buffer.clear();
        } else if c == '\n' {
            // add newline token, will things be in the buffer?
            tokens.push(Token::new(TokenType::Newline, String::from("")));
            buffer.clear();
        } else if c.is_alphanumeric() {
            buffer.push(c);
        } else if c.is_digit(10) {
            while c.is_digit(10) {
                buffer.push(c);
                continue;
            }
            if c == '.' {
                buffer.push('.');
                while c.is_digit(10) {
                    buffer.push(c);
                }

                let token_float = match buffer.parse::<f64>() {
                    Ok(n) => n,
                    Err(_) => {
                        println!("Failed to parse float: {}", buffer);
                        0.0
                    }
                };
                tokens.push(Token::new(TokenType::Float(token_float), buffer.clone()));
                buffer.clear();
                continue;
            } else {
                let token_int = match buffer.parse::<i64>() {
                    Ok(n) => n,
                    Err(_) => {
                        println!("Failed to parse integer: {}", buffer);
                        0
                    }
                };

                tokens.push(Token::new(TokenType::Integer(token_int), buffer.clone()));
                buffer.clear();
            }
        } else {
            // undefined token?
            println!("the undefined token is {c}");
        }
    }

    // test code
    // tokens.push(Token::new(TokenType::Eq, Some(5)));
    return tokens;
}

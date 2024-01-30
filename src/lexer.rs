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
            TokenType::Float(n) => write!(f, "float: {n}"),
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
    line_number: i32,
}

impl Token {
    fn new(token: TokenType, lexeme: String, line_number: i32) -> Token {
        Token {
            token,
            lexeme,
            line_number,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "token type: {} | text: {} | on line {}",
            self.token, self.lexeme, self.line_number
        )
    }
}

fn process_buffer(buffer: &mut String, tokens: &mut Vec<Token>, line_number: i32) {
    if !buffer.is_empty() {
        let token_type = if let Ok(n) = buffer.parse::<i64>() {
            TokenType::Integer(n)
        } else if let Ok(n) = buffer.parse::<f64>() {
            TokenType::Float(n)
        } else {
            TokenType::from_str(&buffer)
        };

        tokens.push(Token::new(token_type, buffer.clone(), line_number));
        buffer.clear();
    }
}

pub fn get_tokens(data: String) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut buffer = String::new();
    let mut line_number: i32 = 1;
    let special_chars = ['(', ')', ',', '+', '-', '*', '/', '^', '%', '\"'];

    let mut chars = data.chars().peekable();

    while let Some(&c) = chars.peek() {
        if c.is_whitespace() {
            process_buffer(&mut buffer, &mut tokens, line_number);
            if c == '\n' {
                line_number += 1;
                tokens.push(Token::new(
                    TokenType::Newline,
                    String::from(""),
                    line_number,
                ));
            }
            chars.next();
        } else if c.is_alphanumeric() {
            buffer.push(chars.next().unwrap());
        } else if special_chars.contains(&c) {
            process_buffer(&mut buffer, &mut tokens, line_number);
            buffer.clear();
            buffer.push(c);
            process_buffer(&mut buffer, &mut tokens, line_number);
            chars.next();
        } else if c.is_digit(10) || (c == '.' || buffer.chars().all(char::is_numeric)) {
            buffer.push(chars.next().unwrap());
        } else {
            process_buffer(&mut buffer, &mut tokens, line_number);
            buffer.push(chars.next().unwrap());
            if let Some(&next_char) = chars.peek() {
                if next_char.is_whitespace() || next_char.is_digit(10) {
                    process_buffer(&mut buffer, &mut tokens, line_number);
                }
            }
        }
    }

    tokens
}

// unit testing
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_adds_two() {
        assert_eq!(4, 2 + 2);
    }
}

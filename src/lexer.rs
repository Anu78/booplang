#![allow(dead_code)]
use std::fmt;

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
    value: Option<i64>,
    lexeme: String,
}

impl Token {
    fn new(token: TokenType, value: Option<i64>, lexeme: String) -> Token {
        Token {
            token,
            value,
            lexeme,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.value {
            Some(value) => write!(
                f,
                "token type: {} | optional value: {} | text: {}",
                self.token, value, self.lexeme
            ),
            None => write!(f, "token type: {} | text: {}", self.token, self.lexeme),
        }
    }
}

pub fn get_token(data: String) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut buffer = String::new();

    for c in data.chars() {
        if c.is_whitespace() || c == '\n' {
            let token_type = TokenType::from_str(&buffer);
            tokens.push(Token::new(token_type, None, buffer.clone()));
            buffer.clear();
            continue;
        }

        while c.is_alphanumeric() {
            buffer += &c.to_string();
            continue;
        }

        buffer += &c.to_string();
    }

    // test code
    // tokens.push(Token::new(TokenType::Eq, Some(5)));
    return tokens;
}

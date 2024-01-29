use std::fmt;

enum TokenType {
    Func,
    Accepts,
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
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TokenType::Func => write!(f, "Func"),
            TokenType::Accepts => write!(f, "Accepts"),
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
        }
    }
}

pub struct Token {
    token: TokenType,
    value: Option<i64>,
}

impl Token {
    fn new(token: TokenType, value: Option<i64>) -> Token {
        Token { token, value }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.value {
            Some(value) => write!(f, "token type: {} | optional value: {}", self.token, value),
            None => write!(f, "token type: {} | no value", self.token),
        }
    }
}

pub fn get_token(file: String) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();

    tokens.push(Token::new(TokenType::Eq, Some(5)));
    return tokens;
}

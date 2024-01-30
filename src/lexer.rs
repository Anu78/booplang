#![allow(dead_code)]
use std::fmt;

#[derive(Debug, PartialEq)]
enum TokenType {
    Note,
    Lte,
    Gte,
    Gt,
    Lt,
    Neq,
    Eq,
    For,
    Until,
    Break,
    Continue,
    If,
    Then,
    Else,
    ElseIf,
    Do,
    End,
    Is,
    False,
    True,
    Function,
    Uses,
    Or,
    And,
    Not,
    Return,
    Multiply,
    Add,
    Subtract,
    Divide,
    Power,
    OpenParenthesis,
    CloseParenthesis,
    Identifier(String),
    StringLiteral(String),
    Comma,
    Newline,
    Integer(i64),
    Float(f64),
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenType::Note => write!(f, "Note"),
            TokenType::Lte => write!(f, "Lte"),
            TokenType::Gte => write!(f, "Gte"),
            TokenType::Gt => write!(f, "Gt"),
            TokenType::Lt => write!(f, "Lt"),
            TokenType::Neq => write!(f, "Neq"),
            TokenType::Eq => write!(f, "Eq"),
            TokenType::For => write!(f, "For"),
            TokenType::Until => write!(f, "Until"),
            TokenType::Break => write!(f, "Break"),
            TokenType::Continue => write!(f, "Continue"),
            TokenType::If => write!(f, "If"),
            TokenType::Then => write!(f, "Then"),
            TokenType::Else => write!(f, "Else"),
            TokenType::ElseIf => write!(f, "ElseIf"),
            TokenType::Do => write!(f, "Do"),
            TokenType::End => write!(f, "End"),
            TokenType::Is => write!(f, "Is"),
            TokenType::False => write!(f, "False"),
            TokenType::True => write!(f, "True"),
            TokenType::Function => write!(f, "Function"),
            TokenType::Uses => write!(f, "Uses"),
            TokenType::Or => write!(f, "Or"),
            TokenType::And => write!(f, "And"),
            TokenType::Not => write!(f, "Not"),
            TokenType::Return => write!(f, "Return"),
            TokenType::Multiply => write!(f, "Multiply"),
            TokenType::Add => write!(f, "Add"),
            TokenType::Subtract => write!(f, "Subtract"),
            TokenType::Divide => write!(f, "Divide"),
            TokenType::Power => write!(f, "Power"),
            TokenType::OpenParenthesis => write!(f, "OpenParenthesis"),
            TokenType::CloseParenthesis => write!(f, "CloseParenthesis"),
            TokenType::StringLiteral(s) => write!(f, "String: {}", s),
            TokenType::Identifier(val) => write!(f, "Identifier({})", val),
            TokenType::Comma => write!(f, "Comma"),
            TokenType::Newline => write!(f, "Newline"),
            TokenType::Integer(val) => write!(f, "Integer({})", val),
            TokenType::Float(val) => write!(f, "Float({})", val),
        }
    }
}

impl TokenType {
    fn from_str(s: &str) -> TokenType {
        match s.to_lowercase().as_str() {
            "func" => TokenType::Function,
            "uses" => TokenType::Uses,
            "if" => TokenType::If,
            "then" => TokenType::Then,
            "return" => TokenType::Return,
            "else" => TokenType::Else,
            "end" => TokenType::End,
            "lt" => TokenType::Lt,
            "lte" => TokenType::Lte,
            "eq" => TokenType::Eq,
            "gt" => TokenType::Gt,
            "gte" => TokenType::Gte,
            "not" => TokenType::Not,
            "(" => TokenType::OpenParenthesis,
            ")" => TokenType::CloseParenthesis,
            "+" => TokenType::Add,
            "-" => TokenType::Subtract,
            "*" => TokenType::Multiply,
            "/" => TokenType::Divide,
            "^" => TokenType::Power,
            "," => TokenType::Comma,
            "note" => TokenType::Note,
            "until" => TokenType::Until,
            "break" => TokenType::Break,
            "continue" => TokenType::Continue,
            "elif" => TokenType::ElseIf,
            "do" => TokenType::Do,
            "is" => TokenType::Is,
            "false" => TokenType::False,
            "true" => TokenType::True,
            "or" => TokenType::Or,
            "and" => TokenType::And,
            "newline" => TokenType::Newline,
            _ => {
                if let Ok(i) = s.parse::<i64>() {
                    TokenType::Integer(i)
                } else if let Ok(f) = s.parse::<f64>() {
                    TokenType::Float(f)
                } else {
                    TokenType::Identifier(String::from(s))
                }
            }
        }
    }
}
#[derive(Debug, PartialEq)]
pub struct Token {
    token: TokenType,
    line_number: i32,
}

impl Token {
    fn new(token: TokenType, line_number: i32) -> Token {
        Token { token, line_number }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} | on line {}", self.token, self.line_number)
    }
}

fn process_buffer(buffer: &mut String, tokens: &mut Vec<Token>, line_number: i32) {
    if !buffer.is_empty() && !buffer.chars().all(char::is_whitespace){
        let token_type = TokenType::from_str(&buffer);

        tokens.push(Token::new(token_type, line_number));
        buffer.clear();
    }
}

pub fn get_tokens(data: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut buffer = String::new();
    let mut line_number: i32 = 1;
    let special_chars = ['(', ')', ',', '+', '-', '*', '/', '^', '%'];

    let mut chars = data.chars().peekable();

    while let Some(&c) = chars.peek() {
        if c.is_whitespace() {
            process_buffer(&mut buffer, &mut tokens, line_number);
            if c == '\n' {
                tokens.push(Token::new(TokenType::Newline, line_number));
                line_number += 1;
            }
            chars.next();
        } else if c == '"' {
            process_buffer(&mut buffer, &mut tokens, line_number); // quote is a delimiter

            chars.next(); // advance iterator to get over initial char
            while let Some(next) = chars.next() {
                if next == '"' {
                    break;
                }
                buffer.push(next);
            }
            tokens.push(Token::new(
                TokenType::StringLiteral(buffer.clone()),
                line_number,
            ));
            buffer.clear();
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

    if !buffer.is_empty() {
        process_buffer(&mut buffer, &mut tokens, line_number);
    }

    tokens
}

// unit testing for lexer
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // test integer and float parsing
    fn test_integer_float_parsing() {
        let test_input = "40.65 21 2.004";
        let tokens = get_tokens(test_input);
        let expected = [
            Token::new(TokenType::Float(40.65), 1),
            Token::new(TokenType::Integer(21), 1),
            Token::new(TokenType::Float(2.004), 1),
        ];

        assert_eq!(tokens, expected);
    }
    // test parenthesis and content between
    #[test]
    fn test_parenthesis() {
        let test_input = "fib(x-1) + fib(x-2) (())";
        let tokens = get_tokens(test_input);

        let expected = [
            Token::new(TokenType::Identifier(String::from("fib")), 1),
            Token::new(TokenType::OpenParenthesis, 1),
            Token::new(TokenType::Identifier(String::from("x")), 1),
            Token::new(TokenType::Subtract, 1),
            Token::new(TokenType::Integer(1), 1),
            Token::new(TokenType::CloseParenthesis, 1),
            Token::new(TokenType::Add, 1),
            Token::new(TokenType::Identifier(String::from("fib")), 1),
            Token::new(TokenType::OpenParenthesis, 1),
            Token::new(TokenType::Identifier(String::from("x")), 1),
            Token::new(TokenType::Subtract, 1),
            Token::new(TokenType::Integer(2), 1),
            Token::new(TokenType::CloseParenthesis, 1),
            Token::new(TokenType::OpenParenthesis, 1),
            Token::new(TokenType::OpenParenthesis, 1),
            Token::new(TokenType::CloseParenthesis, 1),
            Token::new(TokenType::CloseParenthesis, 1),
        ];

        assert_eq!(tokens, expected);
    }

    // test quotations
    #[test]
    fn test_quotes() {
        let test_input = "\"this is a sample string\"";
        let tokens = get_tokens(test_input);

        let expected = [Token::new(
            TokenType::StringLiteral(String::from("this is a sample string")),
            1,
        )];

        assert_eq!(tokens, expected);
    }
    // test all keywords
    #[test]
    fn test_complete_program() {
        let file_string = " func fib uses x, y, z
                if x lt 3 return 1
                else return fib(x-1) + fib(x-2)
            end

            x is 40
            y is \"hello world\"
            fib(x) "; // !TODO: fix this later

        let tokens = get_tokens(file_string);

        let expected = [
            // func fib uses x, y, z
            Token::new(TokenType::Function, 1),
            Token::new(TokenType::Identifier(String::from("fib")), 1),
            Token::new(TokenType::Uses, 1),
            Token::new(TokenType::Identifier(String::from("x")), 1),
            Token::new(TokenType::Comma, 1),
            Token::new(TokenType::Identifier(String::from("y")), 1),
            Token::new(TokenType::Comma, 1),
            Token::new(TokenType::Identifier(String::from("z")), 1),
            Token::new(TokenType::Newline, 1),
            // if x lt 3 return 1
            Token::new(TokenType::If, 2),
            Token::new(TokenType::Identifier(String::from("x")), 2),
            Token::new(TokenType::Lt, 2),
            Token::new(TokenType::Integer(3), 2),
            Token::new(TokenType::Return, 2),
            Token::new(TokenType::Integer(1), 2),
            Token::new(TokenType::Newline, 2),
            // else return fib(x-1) + fib(x-2)
            Token::new(TokenType::Else, 3),
            Token::new(TokenType::Return, 3),
            Token::new(TokenType::Identifier(String::from("fib")), 3),
            Token::new(TokenType::OpenParenthesis, 3),
            Token::new(TokenType::Identifier(String::from("x")), 3),
            Token::new(TokenType::Subtract, 3),
            Token::new(TokenType::Integer(1), 3),
            Token::new(TokenType::CloseParenthesis, 3),
            Token::new(TokenType::Add, 3),
            Token::new(TokenType::Identifier(String::from("fib")), 3),
            Token::new(TokenType::OpenParenthesis, 3),
            Token::new(TokenType::Identifier(String::from("x")), 3),
            Token::new(TokenType::Subtract, 3),
            Token::new(TokenType::Integer(2), 3),
            Token::new(TokenType::CloseParenthesis, 3),
            Token::new(TokenType::Newline, 3),
            // end
            Token::new(TokenType::End, 4),
            Token::new(TokenType::Newline, 4),
            // newline
            Token::new(TokenType::Newline, 5),
            // x is 40
            Token::new(TokenType::Identifier(String::from("x")), 6),
            Token::new(TokenType::Is, 6),
            Token::new(TokenType::Integer(40), 6),
            Token::new(TokenType::Newline, 6),
            // y is "hello world"
            Token::new(TokenType::Identifier(String::from("y")), 7),
            Token::new(TokenType::Is, 7),
            Token::new(TokenType::StringLiteral(String::from("hello world")), 7),
            Token::new(TokenType::Newline, 7),
            // fib(x)
            Token::new(TokenType::Identifier(String::from("fib")), 8),
            Token::new(TokenType::OpenParenthesis, 8),
            Token::new(TokenType::Identifier(String::from("x")), 8),
            Token::new(TokenType::CloseParenthesis, 8),
        ];

        assert_eq!(tokens, expected);
    }
    
}

#![allow(dead_code)]
use crate::lexer::Token;
use crate::lexer::TokenType;

#[derive(Debug, PartialEq)]
pub enum ASTNode {
    Program(Vec<ASTNode>),
    Function {
        name: String,
        params: Vec<Expression>,
        body: Vec<ASTNode>,
    },
    PrintStatement(Expression),
    ReturnStatement(Expression),
    Expression(Expression),
    CreateVariable {
        name: String,
        value: Expression,
    },
    IfStatement {
        condition: Condition,
        then_branch: Vec<ASTNode>,
        elif_branches: Vec<(Expression, Vec<ASTNode>)>,
        else_branch: Option<Vec<ASTNode>>,
    },
}

#[derive(Debug, PartialEq)]
pub enum Condition {
    Binary {
        left: Box<Expression>,
        operator: TokenType,
        right: Box<Expression>,
    },
    Unary {
        operator: TokenType,
        right: Box<Expression>,
    },
}

#[derive(Debug, PartialEq)]
pub enum Expression {
    Number(i32),
    StringLiteral(String),
    Variable(String),
    FunctionCall {
        name: String,
        params: Vec<Expression>,
    },
    BinaryOperation {
        left: Box<Expression>,
        operator: TokenType,
        right: Box<Expression>,
    },
}

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser { tokens, current: 0 }
    }

    // entry point for parser
    pub fn parse_program(&mut self) -> ASTNode {
        let mut statements = Vec::new();

        while !self.at_end() {

            statements.push(self.parse_statement());
        }

        ASTNode::Program(statements)
    }

    // secondary parse methods
    fn parse_statement(&mut self) -> ASTNode {
        match self.peek() {
            TokenType::Function => self.parse_function(),
            TokenType::Return => self.parse_return(),
            // TokenType::If => self.parse_conditional(),
            TokenType::Identifier(_) => {
                let next_token = self.peek_n(2);

                match next_token {
                    Some(TokenType::OpenParenthesis) => self.parse_function_call(),
                    Some(TokenType::Is) => self.parse_variable_declaration(),
                    _ => panic!(
                        "invalid variable declaration syntax on line {}",
                        self.get_line_number()
                    ),
                }
            }

            undefined => {
                panic!(
                    "undefined token {} found on line {}",
                    undefined,
                    self.get_line_number()
                )
            }
        }
    }

    fn parse_variable_declaration(&mut self) -> ASTNode {
        let var_ident = self.consume(
            TokenType::Identifier(String::from("")),
            "expected a variable name",
        );

        let var_name = match var_ident.token {
            TokenType::Identifier(str) => str,
            _ => {
                panic!("expected a valid identifier name")
            }
        };

        self.consume(TokenType::Is, "expected an is keyword");

        let var_value = self.parse_expression(TokenType::Newline);

        ASTNode::CreateVariable {
            name: var_name,
            value: var_value,
        }
    }

    fn parse_conditional(&mut self) -> ASTNode {
        let mut if_branch: Vec<ASTNode> = Vec::new();
        self.consume(TokenType::If, "expected an if keyword");
        let conditional = self.parse_expression(TokenType::Newline);

        // parse conditional into condition
        ASTNode::IfStatement {
            condition: Condition::Unary {
                operator: TokenType::Not,
                right: Box::new(Expression::Variable(String::from("test"))),
            },
            then_branch: Vec::new(),
            elif_branches: Vec::new(),
            else_branch: None,
        }
    }

    fn parse_function(&mut self) -> ASTNode {
        let mut body = Vec::new(); // body of function
        let mut params = Vec::new(); // array of expressions

        // consume function keyword
        self.consume(TokenType::Function, "function keyword expected");
        let name_token = self.consume(
            TokenType::Identifier(String::from("")), // identifier with any variant
            "function name expected",
        );
        let name = if let TokenType::Identifier(name) = name_token.token {
            name
        } else {
            panic!("invalid function name on line {}", self.get_line_number());
        };

        // parse function parameters
        self.consume(TokenType::Uses, "expected a uses keyword");

        // assume one parameter for now
        self.consume(
            TokenType::Identifier(String::from("")),
            "expected a parameter name",
        );

        // parse function body
        while !self.at_end() && !self.check(&TokenType::End) {
            body.push(self.parse_statement());
        }

        // return function ASTNode
        ASTNode::Function { name, params, body }
    }

    fn parse_return(&mut self) -> ASTNode {
        let value = self.parse_expression(TokenType::Newline);

        ASTNode::ReturnStatement(Expression::Number(20))
    }

    fn parse_function_call(&mut self) -> ASTNode {
        let mut params = Vec::new();
        // parse function name
        let func_name = self.consume(
            TokenType::Identifier(String::from("")),
            "expected a function name",
        );

        let name = if let TokenType::Identifier(name) = func_name.token {
            name
        } else {
            panic!("invalid function name on line {}", self.get_line_number());
        };

        ASTNode::Expression(Expression::FunctionCall { name, params })
    }

    fn parse_expression(&mut self, until: TokenType) -> Expression {
        let tokens = self.peek_until(&until);

        for i in tokens.iter() {
            println!("{}", i.token)
        }

        // actual parsing logic here
        //

        Expression::Number(20)
    }

    // utility functions start here ⬇️
    fn consume(&mut self, expected: TokenType, message: &str) -> Token {
        if self.check(&expected) {
            let token = self.tokens[self.current].clone(); // Clone the token to return it
            self.advance(); // Move to the next token
            token
        } else {
            panic!("{} on line {}", message, self.get_line_number());
        }
    }

    fn get_line_number(&self) -> i32 {
        self.tokens[self.current].line_number
    }

    fn peek_until(&self, until: &TokenType) -> Vec<&Token> {
        let mut tokens = Vec::new();
        let mut index = self.current; // Use a local index instead of modifying `self.current`

        // Loop until the end is reached or the `until` token type is found
        while index < self.tokens.len()
            && std::mem::discriminant(&self.tokens[index].token) != std::mem::discriminant(until)
        {
            tokens.push(&self.tokens[index]);
            index += 1;
        }

        tokens
    }

    fn advance(&mut self) -> Option<&Token> {
        if self.at_end() {
            None
        } else {
            let token = &self.tokens[self.current];
            self.current += 1;
            Some(token)
        }
    }

    fn at_end(&self) -> bool {
        self.check(&TokenType::Eof)
    }

    // default peek is one token ahead
    fn peek(&self) -> &TokenType {
        &self.tokens[self.current].token
    }

    fn peek_n(&self, n: usize) -> Option<&TokenType> {
        self.tokens.get(self.current + n).map(|token| &token.token)
    }

    fn check(&self, token: &TokenType) -> bool {
        if let Some(next_token) = self.tokens.get(self.current) {
            std::mem::discriminant(&next_token.token) == std::mem::discriminant(token)
        } else {
            false
        }
    }
}

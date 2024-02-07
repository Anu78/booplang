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
    Declaration {
        name: String,
        value: Expression,
    },
    IfStatement {
        condition: Expression,
        then_branch: Vec<ASTNode>,
        elif_branches: Vec<(Expression, Vec<ASTNode>)>,
        else_branch: Option<Vec<ASTNode>>,
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
            TokenType::If => self.parse_conditional(),
            TokenType::Identifier(s) => {
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

    fn parse_variable_declaration(&mut self) -> ASTNode {}

    fn parse_conditional(&mut self) -> ASTNode {}

    fn parse_function(&mut self) -> ASTNode {
        let mut body = Vec::new(); // body of function
        let params = Vec::new(); // array of expressions

        // parse function body
        while !self.at_end() && !self.check(TokenType::End) {
            body.push(self.parse_statement());
        }

        // return function ASTNode
        ASTNode::Function { name, params, body }
    }

    fn parse_return(&mut self) -> ASTNode {
        let value = self.parse_expression();

        ASTNode::ReturnStatement(value)
    }

    fn parse_function_call(&mut self) -> ASTNode {
        // parse function name
        // parse function parameters

        ASTNode::Expression(Expression::Number(20))
    }

    fn parse_expression(&mut self) -> Expression {
        Expression::Number(20)
    }

    // utility functions
    fn consume(&mut self, expected: TokenType, message: &str) -> Option<&Token> {
        if self.check(expected) {
            self.advance()
        } else {
            panic!("Error at line {}: {}", self.get_line_number(), message);
        }
    }

    fn get_line_number(&self) -> i32 {
        self.tokens[self.current].line_number
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
        self.check(TokenType::Eof)
    }

    // default peek is one token ahead
    fn peek(&self) -> &TokenType {
        &self.tokens[self.current].token
    }

    fn peek_n(&self, n: usize) -> Option<&TokenType> {
        if self.current + n >= self.tokens.len() {
            None
        } else {
            Some(&self.tokens[self.current + n].token)
        }
    }

    fn check(&self, token: TokenType) -> bool {
        if self.at_end() {
            false
        } else {
            let other = self.peek();

            matches!(token, other)
        }
    }
}

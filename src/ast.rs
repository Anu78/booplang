#![allow(dead_code)]
use crate::lexer::Token;
use crate::lexer::TokenType;

#[derive(Debug, PartialEq)]
enum ASTNode {
    Program(Vec<ASTNode>),
    Function {
        name: String,
        params: Vec<Expression>,
        body: Vec<ASTNode>,
    },
    PrintStatement(Expression),
    ReturnStatement(Expression),
    Expression(Expression),
}

#[derive(Debug, PartialEq)]
enum Expression {
    NumberLiteral(i32), // Represents a numeric literal
    Variable(String),   // Represents a variable (which might be used in future extensions)
    FunctionCall {
        name: String,
        params: Vec<Expression>,
    },
}

struct Parser {
    tokens: Vec<Token>,
    current: usize,
    length: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>, current: usize) -> Parser {
        Parser {
            tokens,
            current,
            length: tokens.len(),
        }
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
        match self.peek().token {
            TokenType::Function => self.parse_function(),
            TokenType::Return => self.parse_return(),

            undefined => {
                panic!(
                    "undefined token {} found on line {}",
                    undefined,
                    self.get_line_number()
                )
            }
        }
    }

    fn parse_function(&mut self) -> ASTNode {
        let mut body = Vec::new(); // body of function
        let mut params = Vec::new(); // string array of params
        let mut name: String;

        // parse function body
        while !self.check(TokenType::End) {
            body.push(self.parse_statement());
        }

        // return function ASTNode
        ASTNode::Function { name, params, body }
    }

    fn parse_return(&mut self) -> ASTNode {
        let value = self.parse_expression();

        ASTNode::ReturnStatement(value)
    }

    fn parse_function_call(&mut self) -> Expression {
        // parse function name
        // parse function parameters

        // Expression::FunctionCall {
        //     name: "",
        //     params: ,
        // }
    }

    fn parse_expression(&mut self) -> Expression {}

    // utility functions
    fn consume(&mut self, token: TokenType, message: &str) {
        if self.check(token) {
            self.advance();
        } else {
            println!("missing {} on line {}", message, self.get_line_number());
        }
    }

    fn get_line_number(&self) -> i32 {
        self.tokens[self.current].line_number
    }

    fn advance(&mut self) -> &Token {
        if !self.at_end() {
            self.current += 1;
        }
        &self.tokens[self.current - 1]
    }

    fn at_end(&self) -> bool {
        self.current > self.length
    }

    fn peek(&self) -> &TokenType {
        &self.tokens[self.current].token
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

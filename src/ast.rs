#![allow(dead_code)]
use crate::lexer::Token;
use crate::lexer::TokenType;

struct VariableDeclaration {
    name: String,
    var_type: Type,
    value: Expression,
}

enum Type {
    Integer,
    Float,
    Constant,
    Inferred,
}

struct Function {
    name: String,
    parameters: Vec<Parameter>,
    return_type: Type,
    statements: Vec<Statement>,
}

struct Parameter {
    var_type: Type,
    name: String,
}

enum Statement {
    ForLoop(ForLoop),
    UntilLoop(UntilLoop),
    VariableDeclaration(VariableDeclaration),
    IfStatement(IfStatement),
    Expression(Expression),
    Return(Expression),
    FunctionCall(FunctionCall),
}

struct ForLoop {
    init: Box<Statement>,
    condition: Expression,
    post_loop: Box<Statement>,
    body: Vec<Statement>,
}

struct UntilLoop {
    condition: Expression,
    body: Vec<Statement>,
}

struct IfStatement {
    condition: Expression,
    body: Vec<Statement>,
    elif_branches: Option<Vec<ElifBranch>>, // optional
    else_branch: Option<Vec<Statement>>,    // optional
}

struct ElifBranch {
    condition: Expression,
    body: Vec<Statement>,
}

struct FunctionCall {
    name: String,
    paremeters: Vec<Expression>,
}

enum Expression {
    BinaryExpression(BinaryExpression),
    VariableAccess(String),
    Value(Type),
}

struct BinaryExpression {
    left: Box<Expression>,
    operation: Operation,
    right: Box<Expression>,
}

enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
    GreaterThan,
    GreaterThanEquals,
    LessThan,
    LessThanEquals,
    Equals,
}

pub struct Program {
    global_variables: Vec<VariableDeclaration>,
    functions: Vec<Function>,
}

pub fn generate_ast(tokens: Vec<Token>) -> Program {
    let mut in_function = false;

    let mut program = Program {
        global_variables: Vec::new(),
        functions: Vec::new(),
    };

    let mut tokens_peekable = tokens.iter().peekable();

    while let Some(tok) = tokens_peekable.next() {
        match &tok.token {
            TokenType::Function => {
                println!("function found!");
            }
            TokenType::Identifier(s) => {}
            TokenType::If => {}
            TokenType::Return => {}
            TokenType::End => {}
            TokenType::OpenParenthesis => {}
            // function: assume params until newline / start function scope. check if identifier is main / label
            // identifier: check for "is" and then parse next expression until newline
            // if: check for condition
            // elif: add condition, add branch to if statement if if statement was the last token
            // else:
            // return: combine expressions until newline.
            // end: end function scope and return to global scope
            // paren: parse until close paren, create expressions
            // identifier( = function call. parse until )
            //
            invalid_token => {
                println!("Invalid token: {}", invalid_token);
            }
        }
    }

    program
}
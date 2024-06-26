use crate::parser::parser::Node;
use crate::parser::visitor::ASTVisitor;
use crate::tokenizer::{TokenInfo};

pub mod parser;
pub mod visitor;

pub struct Ast {
    pub statements: Vec<ASTStatement>,
}

impl Ast {
    pub fn new() -> Self {
        Self {
            statements: Vec::new(),
        }
    }

    pub fn add_statement(&mut self, statement: ASTStatement) {
        self.statements.push(statement);
    }
    pub fn visit(&self, visitor: &mut dyn ASTVisitor) {
        for statement in &self.statements {
            visitor.visit_statement(statement);
        }
        visitor.finalize();
    }
}

pub fn parse(tokens: Vec<TokenInfo>) -> Option<Ast> {
    let mut ast: Ast = Ast::new();
    let mut parser = Node::new(tokens);

    loop {
        match parser.next_statement() {
            Some(stmt) => ast.add_statement(stmt),
            None => {
                if parser.is_at_end() {
                    break;
                } else {
                    return None;
                }
            }
        }
    }

    return Some(ast);
}

#[derive(Debug, Clone)]
pub enum ASTStatementKind {
    Expression(ASTExpression),
    Let(ASTLetStatement),
    If(ASTIfStatement),
    Block(ASTBlockStatement),
    While(ASTWhileStatement),
    FuncDecl(ASTFuncDeclStatement),
    Return(ASTReturnStatement),
    For(ASTForStatement),
}

#[derive(Debug, Clone)]
pub struct ASTReturnStatement {
    pub return_keyword: TokenInfo,
    pub return_value: Option<ASTExpression>,
}

#[derive(Debug, Clone)]
pub struct ASTTypeAnnotationExpression {
    pub base: TokenInfo,
    pub generics: Vec<TokenInfo>,
}

#[derive(Debug, Clone)]
pub struct FuncDeclParameter {
    pub identifier: TokenInfo,
    pub type_annotation: Option<ASTExpression>,
}

#[derive(Debug, Clone)]
pub struct ASTFuncDeclStatement {
    pub identifier: TokenInfo,
    pub parameters: Vec<FuncDeclParameter>,
    pub type_annotation: Option<ASTExpression>,
    pub body: Box<ASTStatement>,
}
#[derive(Debug, Clone)]
pub struct ASTWhileStatement {
    pub while_keyword: TokenInfo,
    pub condition: ASTExpression,
    pub body: Box<ASTStatement>,
}

#[derive(Debug, Clone)]
pub struct ASTForStatement {
    pub for_keyword: TokenInfo,
    pub identifier: TokenInfo,
    pub type_annotation: Option<ASTExpression>,
    pub iterable: ASTExpression,
    pub body: Box<ASTStatement>,
}

#[derive(Debug, Clone)]
pub struct ASTStdCallExpression {
    pub std_keyword: TokenInfo,
    pub double_colon: TokenInfo,
    pub identifier: TokenInfo,
    pub arguments: Vec<ASTExpression>,
}

impl ASTForStatement {
    pub fn new(
        for_keyword: TokenInfo,
        identifier: TokenInfo,
        type_annotation: Option<ASTExpression>,
        iterable: ASTExpression,
        body: ASTStatement,
    ) -> Self {
        ASTForStatement {
            for_keyword,
            identifier,
            type_annotation,
            iterable,
            body: Box::new(body),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ASTBlockStatement {
    pub statements: Vec<ASTStatement>,
}
#[derive(Debug, Clone)]
pub struct ASTElseStatement {
    pub else_keyword: TokenInfo,
    pub else_statement: Box<ASTStatement>,
}

impl ASTElseStatement {
    pub fn new(else_keyword: TokenInfo, else_statement: ASTStatement) -> Self {
        ASTElseStatement {
            else_keyword,
            else_statement: Box::new(else_statement),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ASTIfStatement {
    pub if_keyword: TokenInfo,
    pub condition: ASTExpression,
    pub then_branch: Box<ASTStatement>,
    pub else_branch: Option<ASTElseStatement>,
}

#[derive(Debug, Clone)]
pub struct ASTLetStatement {
    pub is_mut: bool,
    pub identifier: TokenInfo,
    pub type_annotation: Option<ASTExpression>,
    pub initializer: ASTExpression,
}

#[derive(Debug, Clone)]
pub struct ASTStatement {
    pub kind: ASTStatementKind,
}

impl ASTStatement {
    pub fn new(kind: ASTStatementKind) -> Self {
        ASTStatement { kind }
    }
    pub fn expression(expr: ASTExpression) -> Self {
        ASTStatement::new(ASTStatementKind::Expression(expr))
    }

    pub fn let_statement(
        identifier: TokenInfo,
        type_annotation: Option<ASTExpression>,
        is_mut: bool,
        initializer: ASTExpression,
    ) -> Self {
        ASTStatement::new(ASTStatementKind::Let(ASTLetStatement {
            identifier,
            type_annotation,
            is_mut,
            initializer,
        }))
    }

    pub fn for_statement(
        for_keyword: TokenInfo,
        identifier: TokenInfo,
        type_annotation: Option<ASTExpression>,
        iterable: ASTExpression,
        body: ASTStatement,
    ) -> Self {
        ASTStatement::new(ASTStatementKind::For(ASTForStatement::new(
            for_keyword,
            identifier,
            type_annotation,
            iterable,
            body,
        )))
    }

    pub fn if_statement(
        if_keyword: TokenInfo,
        condition: ASTExpression,
        then: ASTStatement,
        else_statement: Option<ASTElseStatement>,
    ) -> Self {
        ASTStatement::new(ASTStatementKind::If(ASTIfStatement {
            if_keyword,
            condition,
            then_branch: Box::new(then),
            else_branch: else_statement,
        }))
    }

    pub fn block_statement(statements: Vec<ASTStatement>) -> Self {
        ASTStatement::new(ASTStatementKind::Block(ASTBlockStatement { statements }))
    }

    pub fn while_statement(
        while_keyword: TokenInfo,
        condition: ASTExpression,
        body: ASTStatement,
    ) -> Self {
        ASTStatement::new(ASTStatementKind::While(ASTWhileStatement {
            while_keyword,
            condition,
            body: Box::new(body),
        }))
    }

    pub fn return_statement(
        return_keyword: TokenInfo,
        return_value: Option<ASTExpression>,
    ) -> Self {
        ASTStatement::new(ASTStatementKind::Return(ASTReturnStatement {
            return_keyword,
            return_value,
        }))
    }

    pub fn func_decl_statement(
        identifier: TokenInfo,
        parameters: Vec<FuncDeclParameter>,
        type_annotation: Option<ASTExpression>,
        body: ASTStatement,
    ) -> Self {
        ASTStatement::new(ASTStatementKind::FuncDecl(ASTFuncDeclStatement {
            identifier,
            parameters,
            type_annotation,
            body: Box::new(body),
        }))
    }
}

#[derive(Debug, Clone)]
pub enum ASTExpressionKind {
    Number(ASTNumberExpression),
    String(ASTStringExpression),
    Binary(ASTBinaryExpression),
    Unary(ASTUnaryExpression),
    Parenthesized(ASTParenthesizedExpression),
    Variable(ASTVariableExpression),
    Assignment(ASTAssignmentExpression),
    ArrayAssignment(ASTArrayAssignmentExpression),
    Boolean(ASTBooleanExpression),
    Call(ASTCallExpression),
    StdCall(ASTStdCallExpression),
    Range(ASTRangeExpression),
    Array(ASTArrayExpression),
    ArrayIndex(ASTArrayIndexExpression),
    TypeAnnotation(ASTTypeAnnotationExpression),
}

#[derive(Debug, Clone)]
pub struct ASTRangeExpression {
    pub start: Box<ASTExpression>,
    pub end: Box<ASTExpression>,
}

#[derive(Debug, Clone)]
pub struct ASTCallExpression {
    pub identifier: TokenInfo,
    pub arguments: Vec<ASTExpression>,
}
#[derive(Debug, Clone)]
pub struct ASTBooleanExpression {
    pub value: bool,
    pub token: TokenInfo,
}
#[derive(Debug, Clone)]
pub struct ASTAssignmentExpression {
    pub identifier: TokenInfo,
    pub expression: Box<ASTExpression>,
}

#[derive(Debug, Clone)]
pub struct ASTArrayAssignmentExpression {
    pub identifier: TokenInfo,
    pub index_expression: Box<ASTExpression>,
    pub expression: Box<ASTExpression>,
}

#[derive(Debug, Clone)]
pub enum ASTUnaryOperatorKind {
    Subtraction,
    BwNot,
}
#[derive(Debug, Clone)]
pub struct ASTUnaryOperator {
    pub kind: ASTUnaryOperatorKind,
    pub token: TokenInfo,
}

impl ASTUnaryOperator {
    pub fn new(kind: ASTUnaryOperatorKind, token: TokenInfo) -> Self {
        ASTUnaryOperator { kind, token }
    }
}

#[derive(Debug, Clone)]
pub struct ASTUnaryExpression {
    pub operator: ASTUnaryOperator,
    pub operand: Box<ASTExpression>,
}

#[derive(Debug, Clone)]
pub struct ASTArrayExpression {
    pub elements: Vec<ASTExpression>,
}

#[derive(Debug, Clone)]
pub struct ASTArrayIndexExpression {
    pub array: Box<ASTExpression>,
    pub index: Box<ASTExpression>,
}

#[derive(Debug, Clone)]
pub struct ASTVariableExpression {
    pub identifier: TokenInfo,
}
impl ASTVariableExpression {
    pub fn identifier(&self) -> &str {
        &self.identifier.lexeme
    }
}

#[derive(Debug, Clone)]
pub enum ASTBinaryOperatorKind {
    // Arithmetic
    Addition,
    Subtraction,

    Star,
    Division,
    // Power,
    // Bitwise
    BwAnd,
    BwOr,
    BwXor,
    // Relational
    Equals,
    Inequal,
    LowerThan,
    GreaterThan,
}

#[derive(Debug, Clone)]
pub struct ASTBinaryOperator {
    pub kind: ASTBinaryOperatorKind,
    pub token: TokenInfo,
}
impl ASTBinaryOperator {
    pub fn new(kind: ASTBinaryOperatorKind, token: TokenInfo) -> Self {
        ASTBinaryOperator { kind, token }
    }
    pub fn precedence(&self) -> u8 {
        match self.kind {
            ASTBinaryOperatorKind::Star => 19,
            ASTBinaryOperatorKind::Division => 19,
            ASTBinaryOperatorKind::Addition => 18,
            ASTBinaryOperatorKind::Subtraction => 18,
            ASTBinaryOperatorKind::BwAnd => 17,
            ASTBinaryOperatorKind::BwXor => 16,
            ASTBinaryOperatorKind::BwOr => 15,
            ASTBinaryOperatorKind::Equals => 30,
            ASTBinaryOperatorKind::Inequal => 30,
            ASTBinaryOperatorKind::LowerThan => 29,
            ASTBinaryOperatorKind::GreaterThan => 29,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ASTBinaryExpression {
    pub left: Box<ASTExpression>,
    pub operator: ASTBinaryOperator,
    pub right: Box<ASTExpression>,
}

#[derive(Debug, Clone)]
pub struct ASTNumberExpression {
    pub num: TokenInfo,
}

#[derive(Debug, Clone)]
pub struct ASTStringExpression {
    pub token: TokenInfo,
}

#[derive(Debug, Clone)]
pub struct ASTParenthesizedExpression {
    pub expression: Box<ASTExpression>,
}

#[derive(Debug, Clone)]
pub struct ASTExpression {
    pub kind: ASTExpressionKind,
}

impl ASTExpression {
    pub fn new(kind: ASTExpressionKind) -> Self {
        ASTExpression { kind }
    }

    pub fn range(start: Box<ASTExpression>, end: Box<ASTExpression>) -> Self {
        Self {
            kind: ASTExpressionKind::Range(ASTRangeExpression { start, end }),
        }
    }
    pub fn number(num: TokenInfo) -> Self {
        ASTExpression::new(ASTExpressionKind::Number(ASTNumberExpression { num }))
    }
    pub fn binary(operator: ASTBinaryOperator, left: ASTExpression, right: ASTExpression) -> Self {
        ASTExpression::new(ASTExpressionKind::Binary(ASTBinaryExpression {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        }))
    }
    pub fn parenthesized(expression: ASTExpression) -> Self {
        ASTExpression::new(ASTExpressionKind::Parenthesized(
            ASTParenthesizedExpression {
                expression: Box::new(expression),
            },
        ))
    }
    pub fn identifier(identifier: TokenInfo) -> Self {
        ASTExpression::new(ASTExpressionKind::Variable(ASTVariableExpression {
            identifier,
        }))
    }
    pub fn unary(operator: ASTUnaryOperator, operand: ASTExpression) -> Self {
        ASTExpression::new(ASTExpressionKind::Unary(ASTUnaryExpression {
            operator,
            operand: Box::new(operand),
        }))
    }

    pub fn assignment(identifier: TokenInfo, expression: ASTExpression) -> Self {
        ASTExpression::new(ASTExpressionKind::Assignment(ASTAssignmentExpression {
            identifier,
            expression: Box::new(expression),
        }))
    }

    pub fn array_assignment(
        identifier: TokenInfo,
        index_expression: ASTExpression,
        expression: ASTExpression,
    ) -> Self {
        ASTExpression::new(ASTExpressionKind::ArrayAssignment(
            ASTArrayAssignmentExpression {
                identifier,
                index_expression: Box::new(index_expression),
                expression: Box::new(expression),
            },
        ))
    }

    pub fn boolean(token: TokenInfo, value: bool) -> Self {
        ASTExpression::new(ASTExpressionKind::Boolean(ASTBooleanExpression {
            token,
            value,
        }))
    }

    pub fn string(token: TokenInfo) -> Self {
        ASTExpression::new(ASTExpressionKind::String(ASTStringExpression { token }))
    }

    pub fn call(identifier: TokenInfo, arguments: Vec<ASTExpression>) -> Self {
        ASTExpression::new(ASTExpressionKind::Call(ASTCallExpression {
            identifier,
            arguments,
        }))
    }

    pub fn std_call(
        std_keyword: TokenInfo,
        double_colon: TokenInfo,
        identifier: TokenInfo,
        arguments: Vec<ASTExpression>,
    ) -> Self {
        ASTExpression::new(ASTExpressionKind::StdCall(ASTStdCallExpression {
            std_keyword,
            double_colon,
            identifier,
            arguments,
        }))
    }

    pub fn array(elements: Vec<ASTExpression>) -> Self {
        ASTExpression {
            kind: ASTExpressionKind::Array(ASTArrayExpression { elements }),
        }
    }

    pub fn array_index(array: Box<ASTExpression>, index: Box<ASTExpression>) -> Self {
        ASTExpression {
            kind: ASTExpressionKind::ArrayIndex(ASTArrayIndexExpression { array, index }),
        }
    }

    pub fn type_annotation(base: TokenInfo, generics: Vec<TokenInfo>) -> Self {
        ASTExpression {
            kind: ASTExpressionKind::TypeAnnotation(ASTTypeAnnotationExpression { base, generics }),
        }
    }
}

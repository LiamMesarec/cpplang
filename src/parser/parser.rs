use crate::tokenizer::{Token, TokenInfo};
use ptree::{Style, TreeItem};

use crate::parser::{
    ASTBinaryOperator, ASTBinaryOperatorKind, ASTElseStatement, ASTExpression, ASTStatement,
    ASTUnaryExpression, ASTUnaryOperator, ASTUnaryOperatorKind, FuncDeclParameter,
};
use std::borrow::Cow;
use std::cell::Cell;
use std::io::Write;

pub struct Counter {
    value: Cell<usize>,
}

impl Counter {
    pub fn new() -> Self {
        Self {
            value: Cell::new(0),
        }
    }
    pub fn increment(&self) {
        let current_value = self.value.get();
        self.value.set(current_value + 1);
    }
    pub fn get_value(&self) -> usize {
        self.value.get()
    }
}

pub struct Node {
    tokens: Vec<TokenInfo>,
    current: Counter,
}

impl Node {
    pub fn new(tokens: Vec<TokenInfo>) -> Self {
        Self {
            tokens: tokens.iter().map(|token| token.clone()).collect(),
            current: Counter::new(),
        }
    }

    pub fn next_statement(&mut self) -> Option<ASTStatement> {
        if self.is_at_end() {
            return None;
        }
        Some(self.parse_statement())
    }

    fn is_at_end(&self) -> bool {
        self.current().token == Token::EOF
    }

    pub fn parse_statement(&mut self) -> ASTStatement {
        match self.current().token {
            Token::Let => self.parse_let_statement(),
            Token::If => self.parse_if_statement(),
            Token::LeftBraces => self.parse_block_statement(),
            Token::While => self.parse_while_statement(),
            Token::Fn => self.parse_function_declaration(),
            _ => self.parse_expression_statement(),
        }
    }

    fn parse_function_declaration(&mut self) -> ASTStatement {
        self.consume_and_check(Token::Fn);
        let identifier = self.consume_and_check(Token::Identifier).clone();
        let parameters = self.parse_optional_parameter_list();
        let body = self.parse_statement();
        ASTStatement::func_decl_statement(identifier, parameters, body)
    }

    fn parse_optional_parameter_list(&mut self) -> Vec<FuncDeclParameter> {
        if self.current().token != Token::LeftParantheses {
            return Vec::new();
        }
        self.consume_and_check(Token::LeftParantheses);
        let mut parameters = Vec::new();
        while self.current().token != Token::RightParantheses && !self.is_at_end() {
            parameters.push(FuncDeclParameter {
                identifier: self.consume_and_check(Token::Identifier).clone(),
            });
            if self.current().token == Token::Comma {
                self.consume_and_check(Token::Comma);
            }
        }
        self.consume_and_check(Token::RightParantheses);
        parameters
    }

    fn parse_while_statement(&mut self) -> ASTStatement {
        let while_keyword = self.consume_and_check(Token::While).clone();
        let condition_expr = self.parse_expression();
        let body = self.parse_statement();
        ASTStatement::while_statement(while_keyword, condition_expr, body)
    }

    fn parse_block_statement(&mut self) -> ASTStatement {
        self.consume_and_check(Token::LeftBraces);
        let mut statements = Vec::new();
        while self.current().token != Token::RightBraces && !self.is_at_end() {
            statements.push(self.parse_statement());
        }
        self.consume_and_check(Token::RightBraces);
        ASTStatement::block_statement(statements)
    }

    fn parse_if_statement(&mut self) -> ASTStatement {
        let if_keyword = self.consume_and_check(Token::If).clone();
        let condition_expr = self.parse_expression();
        let then = self.parse_statement();
        let else_statement = self.parse_optional_else_statement();
        ASTStatement::if_statement(if_keyword, condition_expr, then, else_statement)
    }

    fn parse_optional_else_statement(&mut self) -> Option<ASTElseStatement> {
        if self.current().token == Token::Else {
            let else_keyword = self.consume_and_check(Token::Else).clone();
            let else_statement = self.parse_statement();
            return Some(ASTElseStatement::new(else_keyword, else_statement));
        }
        return None;
    }

    fn parse_let_statement(&mut self) -> ASTStatement {
        self.consume_and_check(Token::Let);
        let identifier = self.consume_and_check(Token::Identifier).clone();
        self.consume_and_check(Token::Equals);
        let expr = self.parse_expression();
        return ASTStatement::let_statement(identifier, expr);
    }
    fn parse_expression_statement(&mut self) -> ASTStatement {
        let expr = self.parse_expression();
        return ASTStatement::expression(expr);
    }

    fn parse_expression(&mut self) -> ASTExpression {
        self.parse_assignment_expression()
    }

    fn parse_assignment_expression(&mut self) -> ASTExpression {
        if self.current().token == Token::Identifier {
            if self.peek(1).token == Token::Equals {
                let identifier = self.consume_and_check(Token::Identifier).clone();
                self.consume_and_check(Token::Equals);
                let expr = self.parse_expression();
                return ASTExpression::assignment(identifier, expr);
            }
        }
        return self.parse_binary_expression(0);
    }

    fn parse_binary_expression(&mut self, precedence: u8) -> ASTExpression {
        let mut left = self.parse_unary_expression();
        while let Some(operator) = self.parse_binary_operator() {
            let operator_precedence = operator.precedence();
            if operator_precedence < precedence {
                break;
            }
            self.consume();
            let right = self.parse_binary_expression(operator_precedence);
            left = ASTExpression::binary(operator, left, right);
        }
        return left;
    }

    fn parse_unary_expression(&mut self) -> ASTExpression {
        if let Some(operator) = self.parse_unary_operator() {
            self.consume();
            let operand = self.parse_unary_expression();
            return ASTExpression::unary(operator, operand);
        }
        return self.parse_primary_expression();
    }
    fn parse_unary_operator(&mut self) -> Option<ASTUnaryOperator> {
        let token = self.current();
        let kind = match token.token {
            Token::Subtraction => Some(ASTUnaryOperatorKind::Subtraction),
            _ => None,
        };
        return kind.map(|kind| ASTUnaryOperator::new(kind, token.clone()));
    }

    fn parse_binary_operator(&mut self) -> Option<ASTBinaryOperator> {
        let token = self.current();
        let kind = match token.token {
            Token::Addition => Some(ASTBinaryOperatorKind::Addition),
            Token::Subtraction => Some(ASTBinaryOperatorKind::Subtraction),
            Token::Star => Some(ASTBinaryOperatorKind::Star),
            Token::Division => Some(ASTBinaryOperatorKind::Division),
            Token::BwAnd => Some(ASTBinaryOperatorKind::BwAnd),
            Token::BwOr => Some(ASTBinaryOperatorKind::BwOr),
            Token::BwXor => Some(ASTBinaryOperatorKind::BwXor),
            Token::Equals => Some(ASTBinaryOperatorKind::Equals),
            Token::Inequal => Some(ASTBinaryOperatorKind::Inequal),
            Token::LowerThan => Some(ASTBinaryOperatorKind::LowerThan),

            Token::GreaterThan => Some(ASTBinaryOperatorKind::GreaterThan),

            _ => None,
        };
        return kind.map(|kind| ASTBinaryOperator::new(kind, token.clone()));
    }
    fn parse_primary_expression(&mut self) -> ASTExpression {
        let token = self.consume();
        return match token.token {
            Token::Number => ASTExpression::number(token.clone()),
            Token::LeftParantheses => {
                let expr = self.parse_expression();
                self.consume_and_check(Token::RightParantheses);
                ASTExpression::parenthesized(expr)
            }
            Token::Identifier => {
                if self.current().token == Token::LeftParantheses {
                    self.parse_call_expression(token.clone())
                } else {
                    ASTExpression::identifier(token.clone())
                }
            }
            _ => panic!("Unexpected token: {:?}", token),
        };
    }
    fn parse_call_expression(&mut self, identifier: TokenInfo) -> ASTExpression {
        self.consume_and_check(Token::LeftParantheses);
        let mut arguments = Vec::new();
        while self.current().token != Token::RightParantheses && !self.is_at_end() {
            arguments.push(self.parse_expression());
            if self.current().token != Token::RightParantheses {
                self.consume_and_check(Token::Comma);
            }
        }
        self.consume_and_check(Token::RightParantheses);
        return ASTExpression::call(identifier.clone(), arguments);
    }

    fn peek(&self, offset: isize) -> &TokenInfo {
        let mut index = (self.current.get_value() as isize + offset) as usize;
        if index >= self.tokens.len() {
            index = self.tokens.len() - 1;
        }
        self.tokens.get(index).unwrap()
    }
    fn current(&self) -> &TokenInfo {
        self.peek(0)
    }

    fn consume(&self) -> &TokenInfo {
        self.current.increment();
        self.peek(-1)
    }

    fn consume_and_check(&self, kind: Token) -> &TokenInfo {
        let token = self.consume();
        if token.token != kind {
            println!("Error");
        }
        token
    }
}

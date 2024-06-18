use crate::tokenizer::{Token, TokenInfo};
use ptree::{Style, TreeItem};

use crate::parser::{
    ASTArrayExpression, ASTArrayIndexExpression, ASTBinaryOperator, ASTBinaryOperatorKind,
    ASTElseStatement, ASTExpression, ASTReturnStatement, ASTStatement, ASTUnaryExpression,
    ASTUnaryOperator, ASTUnaryOperatorKind, Ast, FuncDeclParameter,
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

    pub fn is_at_end(&self) -> bool {
        self.current().token == Token::EOF
    }

    pub fn parse_statement(&mut self) -> ASTStatement {
        match self.current().token {
            Token::Let => self.parse_let_statement(),
            Token::If => self.parse_if_statement(),
            Token::LeftBraces => self.parse_block_statement(),
            Token::While => self.parse_while_statement(),
            Token::Fn => self.parse_function_declaration(),
            Token::For => self.parse_for_statement(),
            Token::Return => self.parse_return_statement(),
            _ => self.parse_expression_statement(),
        }
    }

    fn parse_return_statement(&mut self) -> ASTStatement {
        let keyword = self.consume_and_check(Token::Return).clone();
        let value = self.parse_expression();
        ASTStatement::return_statement(keyword, Some(value))
    }

    fn parse_function_declaration(&mut self) -> ASTStatement {
        self.consume_and_check(Token::Fn);
        let identifier = self.consume_and_check(Token::Identifier).clone();
        let parameters = self.parse_optional_parameter_list();
        let return_type = if self.peek(0).token == Token::Colon {
            self.consume_and_check(Token::Colon);
            Some(self.consume_and_check(Token::Identifier).clone())
        } else {
            None
        };
        let body = self.parse_statement();
        ASTStatement::func_decl_statement(identifier, parameters, return_type, body)
    }

    fn parse_optional_parameter_list(&mut self) -> Vec<FuncDeclParameter> {
        if self.current().token != Token::LeftParantheses {
            return Vec::new();
        }
        self.consume_and_check(Token::LeftParantheses);
        let mut parameters = Vec::new();
        while self.current().token != Token::RightParantheses && !self.is_at_end() {
            let identifier = self.consume_and_check(Token::Identifier).clone();
            let type_annotation = if self.peek(0).token == Token::Colon {
                self.consume_and_check(Token::Colon);
                Some(self.consume_and_check(Token::Identifier).clone())
            } else {
                None
            };
            parameters.push(FuncDeclParameter {
                identifier,
                type_annotation,
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

    fn parse_for_statement(&mut self) -> ASTStatement {
        let for_keyword = self.consume_and_check(Token::For).clone();
        let identifier = self.consume_and_check(Token::Identifier).clone();
        let type_annotation = if self.peek(0).token == Token::Colon {
            self.consume_and_check(Token::Colon);
            Some(self.consume_and_check(Token::Identifier).clone())
        } else {
            None
        };

        self.consume_and_check(Token::In);

        let iterable = self.parse_expression();

        let body = self.parse_statement();
        ASTStatement::for_statement(for_keyword, identifier, type_annotation, iterable, body)
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

        let is_mut = if self.current().token == Token::Mut {
            self.consume_and_check(Token::Mut);
            true
        } else {
            false
        };

        let identifier = self.consume_and_check(Token::Identifier).clone();

        let type_annotation = if self.peek(0).token == Token::Colon {
            self.consume_and_check(Token::Colon);
            Some(self.consume_and_check(Token::Identifier).clone())
        } else {
            None
        };

        self.consume_and_check(Token::Assignment);

        let expr = self.parse_expression();

        return ASTStatement::let_statement(identifier, type_annotation, is_mut, expr);
    }

    fn parse_expression_statement(&mut self) -> ASTStatement {
        let expr = self.parse_expression();
        return ASTStatement::expression(expr);
    }

    fn parse_expression(&mut self) -> ASTExpression {
        if let Some(range_expr) = self.lookahead_for_range() {
            return range_expr;
        }
        if self.current().token == Token::Std {
            let keyword = self.current().clone();
            self.consume_and_check(Token::Std);
            let double_colon = self.current().clone();
            self.consume_and_check(Token::DoubleColon);
            let identifier = self.current().clone();
            self.consume_and_check(Token::Identifier);

            return self.parse_std_call_expression(keyword, double_colon, identifier);
        }
        self.parse_assignment_expression()
    }


    fn lookahead_for_range(&mut self) -> Option<ASTExpression> {
        let start_position = self.current.get_value();
        let start_expr = self.parse_assignment_expression();

        if self.current().token == Token::Range {
            self.consume_and_check(Token::Range); // Consume the range token
                                                println!("HERE");
            let end_expr = self.parse_assignment_expression();
            return Some(ASTExpression::range(Box::new(start_expr), Box::new(end_expr)));
        } else {
            self.current.value.set(start_position);
                                                println!("HERE2");
            return None;
        }
    }
    fn parse_assignment_expression(&mut self) -> ASTExpression {
        if self.current().token == Token::Identifier {
            if self.peek(1).token == Token::LeftSquareBracket {
                let identifier = self.consume_and_check(Token::Identifier).clone();
                let array = ASTExpression::identifier(identifier.clone());
                let array_index = self.parse_array_index_expression(array);
                self.consume_and_check(Token::Assignment);
                let expr = self.parse_expression();
                return ASTExpression::assignment(identifier, array_index);
            } else if self.peek(1).token == Token::Assignment {
                let identifier = self.consume_and_check(Token::Identifier).clone();
                self.consume_and_check(Token::Assignment);
                let expr = self.parse_expression();
                return ASTExpression::assignment(identifier, expr);
            }
        }
        self.parse_binary_expression(0)
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

    fn parse_array_expression(&mut self) -> ASTExpression {
        self.consume_and_check(Token::LeftSquareBracket);
        let mut elements = Vec::new();
        while self.current().token != Token::RightSquareBracket && !self.is_at_end() {
            elements.push(self.parse_expression());
            if self.current().token != Token::RightSquareBracket {
                self.consume_and_check(Token::Comma);
            }
        }
        self.consume_and_check(Token::RightSquareBracket);
        ASTExpression::array(elements)
    }

    fn parse_array_index_expression(&mut self, array: ASTExpression) -> ASTExpression {
        self.consume_and_check(Token::LeftSquareBracket);
        let index = self.parse_expression();
        self.consume_and_check(Token::RightSquareBracket);
        ASTExpression::array_index(Box::new(array), Box::new(index))
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
        let token = self.consume().clone();
        match token.token {
            Token::String => ASTExpression::string(token),
            Token::Number => ASTExpression::number(token),
            Token::LeftParantheses => {
                let expr = self.parse_expression();
                self.consume_and_check(Token::RightParantheses);
                ASTExpression::parenthesized(expr)
            }
            Token::LeftSquareBracket => {
                let array_expr = ASTExpression::identifier(token.clone());
                self.parse_array_index_expression(array_expr)
            }
            Token::Identifier => {
                if self.current().token == Token::LeftParantheses {
                    self.parse_call_expression(token)
                } else if self.current().token == Token::LeftSquareBracket {
                    let array_expr = ASTExpression::identifier(token);
                    self.parse_array_index_expression(array_expr)
                } else {
                    ASTExpression::identifier(token)
                }
            }
            Token::Std => {
            let double_colon = self.current().clone();
            self.consume_and_check(Token::DoubleColon);
            let identifier = self.current().clone();
            self.consume_and_check(Token::Identifier);

             self.parse_std_call_expression(token, double_colon, identifier)
            }
            _ => panic!("Unexpected token: {:?}", token),
        }
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

    fn parse_std_call_expression(
        &mut self,
        std_keyword: TokenInfo,
        double_colon: TokenInfo,
        identifier: TokenInfo,
    ) -> ASTExpression {
        self.consume_and_check(Token::LeftParantheses);
        let mut arguments = Vec::new();
        while self.current().token != Token::RightParantheses && !self.is_at_end() {
            arguments.push(self.parse_expression());
            if self.current().token != Token::RightParantheses {
                self.consume_and_check(Token::Comma);
            }
        }
        self.consume_and_check(Token::RightParantheses);
        return ASTExpression::std_call(std_keyword, double_colon, identifier, arguments);
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
            panic!("Expected token: {:?}, found: {:?}", kind, token.token);
        }
        token
    }
}

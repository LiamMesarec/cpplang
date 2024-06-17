use crate::parser::visitor::ASTVisitor;
use crate::parser::*;
use std::collections::HashMap;

pub struct ASTInterpreter {
    pub last_value: Option<i64>,
    pub variables: HashMap<String, i64>,
}

impl ASTInterpreter {
    pub fn new() -> Self {
        Self {
            last_value: None,
            variables: HashMap::new(),
        }
    }
}

impl ASTVisitor<'_> for ASTInterpreter {
    fn visit_statement(&mut self, statement: &ASTStatement) {
        match &statement.kind {
            ASTStatementKind::Let(let_statement) => self.visit_let_statement(let_statement),
            ASTStatementKind::If(if_statement) => self.visit_if_statement(if_statement),
            ASTStatementKind::Block(block_statement) => self.visit_block_statement(block_statement),
            ASTStatementKind::FuncDecl(func_decl_statement) => {
                self.visit_func_decl_statement(func_decl_statement)
            }
            ASTStatementKind::Return(return_statement) => {
                self.visit_return_statement(return_statement)
            }
            ASTStatementKind::For(for_statement) => self.visit_for_statement(for_statement),
            // Add other statement types here
            _ => unimplemented!("Unsupported statement type {:?}", &statement.kind),
        }
    }

    fn visit_expression(&mut self, expression: &ASTExpression) {
        match &expression.kind {
            ASTExpressionKind::Variable(variable_expr) => {
                self.visit_variable_expression(variable_expr)
            }
            ASTExpressionKind::Number(number_expr) => self.visit_number_expression(number_expr),
            ASTExpressionKind::String(string_expr) => self.visit_string_expression(string_expr),
            ASTExpressionKind::Unary(unary_expr) => self.visit_unary_expression(unary_expr),
            ASTExpressionKind::Binary(binary_expr) => self.visit_binary_expression(binary_expr),
            ASTExpressionKind::Parenthesized(paren_expr) => {
                self.visit_parenthesized_expression(paren_expr)
            }
            // Add other expression types here
            _ => unimplemented!("Unsupported expression type"),
        }
    }

    fn visit_boolean_expression(&mut self, boolean: &ASTBooleanExpression) {
        // Implementation for visiting boolean expressions
    }

    fn visit_let_statement(&mut self, let_statement: &ASTLetStatement) {
        self.visit_expression(&let_statement.initializer);
        self.variables.insert(
            let_statement.identifier.lexeme.clone(),
            self.last_value.unwrap(),
        );
    }

    fn visit_variable_expression(&mut self, variable_expression: &ASTVariableExpression) {
        self.last_value = Some(
            *self
                .variables
                .get(&variable_expression.identifier.lexeme)
                .unwrap(),
        );
    }

    fn visit_number_expression(&mut self, number_expr: &ASTNumberExpression) {
        match number_expr.num.lexeme.parse::<i64>() {
            Ok(n) => self.last_value = Some(n),
            Err(e) => println!("Error: {}", e),
        }
    }

    //string ne dela, to je placeholder
    fn visit_string_expression(&mut self, string_expr: &ASTStringExpression) {
        match string_expr.token.lexeme.parse::<i64>() {
            Ok(n) => self.last_value = Some(n),
            Err(e) => println!("Error: {}", e),
        }
    }

    fn visit_unary_expression(&mut self, unary_expr: &ASTUnaryExpression) {
        self.visit_expression(&unary_expr.operand);
        let operand = self.last_value.unwrap();
        self.last_value = Some(match unary_expr.operator.kind {
            ASTUnaryOperatorKind::Subtraction => -operand,
            ASTUnaryOperatorKind::BwNot => !operand,
            // Add other unary operators as needed
        });
    }

    fn visit_binary_expression(&mut self, binary_expr: &ASTBinaryExpression) {
        self.visit_expression(&binary_expr.left);
        let left = self.last_value.unwrap();
        self.visit_expression(&binary_expr.right);
        let right = self.last_value.unwrap();
        self.last_value = Some(match binary_expr.operator.kind {
            ASTBinaryOperatorKind::Addition => left + right,
            ASTBinaryOperatorKind::Subtraction => left - right,
            ASTBinaryOperatorKind::Star => left * right,
            ASTBinaryOperatorKind::Division => left / right,
            // Add other binary operators as needed
            ASTBinaryOperatorKind::BwAnd => left & right,
            ASTBinaryOperatorKind::BwOr => left | right,
            ASTBinaryOperatorKind::BwXor => left ^ right,
            ASTBinaryOperatorKind::Equals => (left == right) as i64,
            ASTBinaryOperatorKind::Inequal => (left != right) as i64,
            ASTBinaryOperatorKind::LowerThan => (left < right) as i64,
            ASTBinaryOperatorKind::GreaterThan => (left > right) as i64,
        });
    }

    fn visit_parenthesized_expression(&mut self, paren_expr: &ASTParenthesizedExpression) {
        self.visit_expression(&paren_expr.expression);
    }

    fn visit_if_statement(&mut self, if_statement: &ASTIfStatement) {
        self.visit_expression(&if_statement.condition);
        if self.last_value.unwrap() != 0 {
            self.visit_statement(&if_statement.then_branch);
        }
    }

    fn visit_block_statement(&mut self, block_statement: &ASTBlockStatement) {
        for statement in &block_statement.statements {
            self.visit_statement(statement);
        }
    }

    fn visit_func_decl_statement(&mut self, func_decl_statement: &ASTFuncDeclStatement) {
        // For now, let's just print the function declaration
        println!("Function declaration:");
        println!("Identifier: {:?}", func_decl_statement.identifier);
        println!("Parameters: {:?}", func_decl_statement.parameters);
        if let Some(type_annotation) = &func_decl_statement.type_annotation {
            println!("Return Type: {:?}", type_annotation);
        } else {
            println!("Return Type: None");
        }
        println!("Body:");
        self.visit_statement(&func_decl_statement.body);
    }

    fn visit_return_statement(&mut self, return_statement: &ASTReturnStatement) {
        // For now, let's just print the return statement
        println!("Return statement:");
        if let Some(expr) = &return_statement.return_value {
            println!("Expression:");
            self.visit_expression(expr);
        } else {
            println!("No return value");
        }
    }

    fn visit_for_statement(&mut self, for_statement: &ASTForStatement) {
        // For now, let's just print the for statement
        println!("For statement:");
        println!("Identifier: {:?}", for_statement.identifier);
        println!("Iterable:");
        self.visit_expression(&for_statement.iterable);
        println!("Body:");
        self.visit_statement(&for_statement.body);
    }

    fn finalize(&mut self) {}
}

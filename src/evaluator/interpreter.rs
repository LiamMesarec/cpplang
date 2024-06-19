use crate::parser::visitor::ASTVisitor;
use crate::parser::*;
use std::collections::HashMap;
use std::fmt;

pub struct ASTInterpreter {
    pub last_value: Option<VariableType>,
    pub variables: HashMap<String, VariableType>,
}

#[derive(Debug, Clone)]
pub enum VariableType {
    Number(i64),
    String(String),
    Array(Vec<VariableType>),
}

impl fmt::Display for VariableType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            VariableType::Number(n) => write!(f, "{}", n),
            VariableType::String(s) => write!(f, "{}", s),
            VariableType::Array(arr) => {
                let formatted_elements: Vec<String> =
                    arr.iter().map(|elem| elem.to_string()).collect();
                write!(f, "[{}]", formatted_elements.join(", "))
            }
        }
    }
}

impl ASTInterpreter {
    pub fn new() -> Self {
        Self {
            last_value: None,
            variables: HashMap::new(),
        }
    }

    // Helper function to evaluate an expression and return its value
    fn evaluate_expression(&mut self, expression: &ASTExpression) -> VariableType {
        self.visit_expression(expression);
        self.last_value.clone().unwrap()
    }

    fn evaluate_assignment_expression(
        &mut self,
        assignment_expr: &ASTAssignmentExpression,
    ) -> VariableType {
        let value = self.evaluate_expression(&assignment_expr.expression);
        self.variables
            .insert(assignment_expr.identifier.lexeme.clone(), value.clone());
        value
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
            ASTStatementKind::Expression(expr_statement) => self.visit_expression(expr_statement),
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
            ASTExpressionKind::StdCall(std_call_expr) => {
                self.visit_std_call_expression(std_call_expr)
            }
            ASTExpressionKind::Assignment(assignment_expr) => {
                self.visit_assignment_expression(assignment_expr)
            }
            // Add other expression types here
            _ => unimplemented!("Unsupported expression type"),
        }
    }

    fn visit_assignment_expression(&mut self, assignment_expr: &ASTAssignmentExpression) {
        let value = self.evaluate_assignment_expression(assignment_expr);
        self.last_value = Some(value);
    }

    fn visit_boolean_expression(&mut self, boolean: &ASTBooleanExpression) {
        // Implementation for visiting boolean expressions
    }

    fn visit_let_statement(&mut self, let_statement: &ASTLetStatement) {
        let value = self.evaluate_expression(&let_statement.initializer);
        self.variables
            .insert(let_statement.identifier.lexeme.clone(), value);
    }

    fn visit_variable_expression(&mut self, variable_expression: &ASTVariableExpression) {
        if let Some(value) = self.variables.get(&variable_expression.identifier.lexeme) {
            self.last_value = Some(value.clone());
        } else {
            panic!(
                "Undefined variable: {}",
                &variable_expression.identifier.lexeme
            );
        }
    }

    fn visit_number_expression(&mut self, number_expr: &ASTNumberExpression) {
        match number_expr.num.lexeme.parse::<i64>() {
            Ok(n) => self.last_value = Some(VariableType::Number(n)),
            Err(e) => println!("Error: {}", e),
        }
    }

    fn visit_string_expression(&mut self, string_expr: &ASTStringExpression) {
        self.last_value = Some(VariableType::String(string_expr.token.lexeme.clone()));
    }

    fn visit_unary_expression(&mut self, unary_expr: &ASTUnaryExpression) {
        self.visit_expression(&unary_expr.operand);
        let operand = if let VariableType::Number(n) = self.last_value.as_ref().unwrap() {
            n
        } else {
            panic!("Unary operator applied to non-number")
        };
        self.last_value = Some(VariableType::Number(match unary_expr.operator.kind {
            ASTUnaryOperatorKind::Subtraction => -operand,
            ASTUnaryOperatorKind::BwNot => !operand,
        }));
    }

    fn visit_binary_expression(&mut self, binary_expr: &ASTBinaryExpression) {
        let left = if let VariableType::Number(n) = self.evaluate_expression(&binary_expr.left) {
            n
        } else {
            panic!("Binary operator applied to non-number on left side")
        };
        let right = if let VariableType::Number(n) = self.evaluate_expression(&binary_expr.right) {
            n
        } else {
            panic!("Binary operator applied to non-number on right side")
        };
        self.last_value = Some(VariableType::Number(match binary_expr.operator.kind {
            ASTBinaryOperatorKind::Addition => left + right,
            ASTBinaryOperatorKind::Subtraction => left - right,
            ASTBinaryOperatorKind::Star => left * right,
            ASTBinaryOperatorKind::Division => left / right,
            ASTBinaryOperatorKind::BwAnd => left & right,
            ASTBinaryOperatorKind::BwOr => left | right,
            ASTBinaryOperatorKind::BwXor => left ^ right,
            ASTBinaryOperatorKind::Equals => (left == right) as i64,
            ASTBinaryOperatorKind::Inequal => (left != right) as i64,
            ASTBinaryOperatorKind::LowerThan => (left < right) as i64,
            ASTBinaryOperatorKind::GreaterThan => (left > right) as i64,
        }));
    }

    fn visit_parenthesized_expression(&mut self, paren_expr: &ASTParenthesizedExpression) {
        self.visit_expression(&paren_expr.expression);
    }

    fn visit_if_statement(&mut self, if_statement: &ASTIfStatement) {
        let condition =
            if let VariableType::Number(n) = self.evaluate_expression(&if_statement.condition) {
                n
            } else {
                panic!("If condition is not a number")
            };
        if condition != 0 {
            self.visit_statement(&if_statement.then_branch);
        } else if let Some(else_branch) = &if_statement.else_branch {
            self.visit_statement(&else_branch.else_statement);
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

    fn visit_std_call_expression(&mut self, std_call_expr: &ASTStdCallExpression) {
        if std_call_expr.identifier.lexeme == "println" {
            let args: Vec<String> = std_call_expr
                .arguments
                .iter()
                .map(|arg| {
                    self.evaluate_expression(arg);
                    self.last_value.as_ref().unwrap().to_string()
                })
                .collect();
            println!("{}", args.join(" "));
        } else {
            unimplemented!("Unsupported std call type");
        }
    }

    fn finalize(&mut self) {}
}

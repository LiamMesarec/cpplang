use crate::parser::{
    ASTArrayAssignmentExpression, ASTArrayExpression, ASTArrayIndexExpression,
    ASTAssignmentExpression, ASTBinaryExpression, ASTBlockStatement, ASTBooleanExpression,
    ASTCallExpression, ASTExpression, ASTExpressionKind, ASTForStatement, ASTFuncDeclStatement,
    ASTIfStatement, ASTLetStatement, ASTNumberExpression, ASTParenthesizedExpression,
    ASTRangeExpression, ASTReturnStatement, ASTStatement, ASTStatementKind, ASTStdCallExpression,
    ASTStringExpression, ASTTypeAnnotationExpression, ASTUnaryExpression, ASTVariableExpression,
    ASTWhileStatement,
};

pub trait ASTVisitor<'a> {
    fn do_visit_statement(&mut self, statement: &ASTStatement) {
        match &statement.kind {
            ASTStatementKind::Expression(expr) => {
                self.visit_expression(expr);
            }
            ASTStatementKind::Let(expr) => {
                self.visit_let_statement(expr);
            }
            ASTStatementKind::If(stmt) => {
                self.visit_if_statement(stmt);
            }
            ASTStatementKind::Block(stmt) => {
                self.visit_block_statement(stmt);
            }
            ASTStatementKind::While(stmt) => {
                self.visit_while_statement(stmt);
            }
            ASTStatementKind::FuncDecl(stmt) => {
                self.visit_func_decl_statement(stmt);
            }
            ASTStatementKind::Return(stmt) => {
                self.visit_return_statement(stmt);
            }
            ASTStatementKind::For(stmt) => {
                self.visit_for_statement(stmt);
            }
        }
    }

    fn visit_func_decl_statement(&mut self, func_decl_statement: &ASTFuncDeclStatement);

    fn visit_return_statement(&mut self, return_statement: &ASTReturnStatement) {
        if let Some(expr) = &return_statement.return_value {
            self.visit_expression(expr);
        }
    }

    fn visit_while_statement(&mut self, while_statement: &ASTWhileStatement) {
        self.visit_expression(&while_statement.condition);
        self.visit_statement(&while_statement.body);
    }

    fn visit_for_statement(&mut self, for_statement: &ASTForStatement) {
        self.visit_expression(&for_statement.iterable);
        self.visit_statement(&for_statement.body);
    }

    fn visit_block_statement(&mut self, block_statement: &ASTBlockStatement) {
        for statement in &block_statement.statements {
            self.visit_statement(statement);
        }
    }

    fn visit_if_statement(&mut self, if_statement: &ASTIfStatement) {
        self.visit_expression(&if_statement.condition);
        self.visit_statement(&if_statement.then_branch);
        if let Some(else_branch) = &if_statement.else_branch {
            self.visit_statement(&else_branch.else_statement);
        }
    }

    fn visit_let_statement(&mut self, let_statement: &ASTLetStatement);
    fn visit_statement(&mut self, statement: &ASTStatement) {
        self.do_visit_statement(statement);
    }
    fn do_visit_expression(&mut self, expression: &ASTExpression) {
        match &expression.kind {
            ASTExpressionKind::Number(number) => {
                self.visit_number_expression(number);
            }
            ASTExpressionKind::String(string) => {
                self.visit_string_expression(string);
            }
            ASTExpressionKind::Binary(expr) => {
                self.visit_binary_expression(expr);
            }
            ASTExpressionKind::Parenthesized(expr) => {
                self.visit_parenthesized_expression(expr);
            }
            /* ASTExpressionKind::Error(span) => {
                self.visit_error(span);
            }*/
            ASTExpressionKind::Variable(expr) => {
                self.visit_variable_expression(expr);
            }
            ASTExpressionKind::Unary(expr) => {
                self.visit_unary_expression(expr);
            }
            ASTExpressionKind::Assignment(expr) => {
                self.visit_assignment_expression(expr);
            }
            ASTExpressionKind::ArrayAssignment(expr) => {
                self.visit_array_assignment_expression(expr);
            }
            ASTExpressionKind::Boolean(expr) => {
                self.visit_boolean_expression(expr);
            }
            ASTExpressionKind::Call(expr) => {
                self.visit_call_expression(expr);
            }
            ASTExpressionKind::StdCall(expr) => {
                self.visit_std_call_expression(expr);
            }
            ASTExpressionKind::Range(expr) => {
                self.visit_range_expression(expr);
            }
            ASTExpressionKind::Array(expr) => {
                self.visit_array_expression(expr);
            }
            ASTExpressionKind::ArrayIndex(expr) => {
                self.visit_array_index_expression(expr);
            }
            ASTExpressionKind::TypeAnnotation(expr) => {
                self.visit_type_annotation_expression(expr);
            }
        }
    }
    fn visit_call_expression(&mut self, call_expression: &ASTCallExpression) {
        for argument in &call_expression.arguments {
            self.visit_expression(argument);
        }
    }

    fn visit_std_call_expression(&mut self, std_call_expression: &ASTStdCallExpression) {
        for argument in &std_call_expression.arguments {
            self.visit_expression(argument);
        }
    }
    fn visit_expression(&mut self, expression: &ASTExpression) {
        self.do_visit_expression(expression);
    }

    fn visit_assignment_expression(&mut self, assignment_expression: &ASTAssignmentExpression) {
        self.visit_expression(&assignment_expression.expression);
    }

    fn visit_array_assignment_expression(
        &mut self,
        array_assignment_expression: &ASTArrayAssignmentExpression,
    ) {
        self.visit_expression(&array_assignment_expression.expression);
    }

    fn visit_range_expression(&mut self, range_expression: &ASTRangeExpression) {
        self.visit_expression(&range_expression.start);
        self.visit_expression(&range_expression.end);
    }

    fn visit_variable_expression(&mut self, variable_expression: &ASTVariableExpression);

    fn visit_number_expression(&mut self, number: &ASTNumberExpression);

    fn visit_string_expression(&mut self, string: &ASTStringExpression);

    fn visit_boolean_expression(&mut self, boolean: &ASTBooleanExpression);

    fn visit_unary_expression(&mut self, unary_expression: &ASTUnaryExpression);

    fn visit_type_annotation_expression(
        &mut self,
        type_annotation_expression: &ASTTypeAnnotationExpression,
    );

    fn finalize(&mut self);

    fn visit_binary_expression(&mut self, binary_expression: &ASTBinaryExpression) {
        self.visit_expression(&binary_expression.left);
        self.visit_expression(&binary_expression.right);
    }
    fn visit_parenthesized_expression(
        &mut self,
        parenthesized_expression: &ASTParenthesizedExpression,
    ) {
        self.visit_expression(&parenthesized_expression.expression);
    }

    fn visit_array_expression(&mut self, array_expression: &ASTArrayExpression) {
        for element in &array_expression.elements {
            self.visit_expression(element);
        }
    }

    fn visit_array_index_expression(&mut self, array_index_expression: &ASTArrayIndexExpression) {
        self.visit_expression(&array_index_expression.array);
        self.visit_expression(&array_index_expression.index);
    }
}

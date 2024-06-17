use crate::evaluator::to_cpp;
use crate::parser::visitor::ASTVisitor;
use crate::parser::*;
use std::collections::HashMap;

pub struct ASTCppTranspiler {
    pub result: String,
    types: HashMap<String, to_cpp::TypeInfo>,
    std_names: HashMap<String, String>,
    includes: Vec<String>,
}

impl ASTCppTranspiler {
    fn add_whitespace(&mut self) {
        self.result.push_str(" ");
    }

    fn add_keyword(&mut self, keyword: &str) {
        self.result.push_str(&keyword);
    }

    fn add_text(&mut self, text: &str) {
        self.result.push_str(&text);
    }

    fn add_variable(&mut self, variable: &str) {
        self.result.push_str(&variable);
    }

    fn add_boolean(&mut self, boolean: bool) {
        self.result.push_str(&format!("{}", boolean));
    }

    pub fn new() -> Self {
        Self {
            result: String::new(),
            types: to_cpp::init_types().unwrap(),
            std_names: to_cpp::init_std_names().unwrap(),
            includes: Vec::new(),
        }
    }
}

impl ASTVisitor<'_> for ASTCppTranspiler {
    fn visit_func_decl_statement(&mut self, func_decl_statement: &ASTFuncDeclStatement) {
        if let Some(t) = &func_decl_statement.type_annotation {
            if let Some(cpp_t) = to_cpp::translate_type(&t, &self.types) {
                self.add_text(&cpp_t.name);

                if !self.includes.contains(&cpp_t.library) {
                    self.includes.push(cpp_t.library);
                }
            } else {
                self.add_text(&t.lexeme);
            }
        } else {
            self.add_text("auto");
        }
        self.add_whitespace();
        self.add_text(&func_decl_statement.identifier.lexeme);
        let are_parameters_empty = func_decl_statement.parameters.is_empty();
        if !are_parameters_empty {
            self.add_text("(");
        } else {
            self.add_text("()");
            self.add_whitespace();
        }
        for (i, parameter) in func_decl_statement.parameters.iter().enumerate() {
            if i != 0 {
                self.add_text(",");
                self.add_whitespace();
            }
            self.add_text(&parameter.identifier.lexeme);
        }
        if !are_parameters_empty {
            self.add_text(")");
            self.add_whitespace();
        }
        self.visit_statement(&func_decl_statement.body);
    }

    fn visit_for_statement(&mut self, for_statement: &ASTForStatement) {
        self.add_keyword("for");
        self.add_text("(");
        self.add_text("auto");
        self.add_whitespace();
        self.add_text(&for_statement.identifier.lexeme);
        self.add_whitespace();
        self.add_text(":");
        //tu je array
        self.visit_statement(&for_statement.body);
    }
    fn visit_return_statement(&mut self, return_statement: &ASTReturnStatement) {
        self.add_keyword("return");
        if let Some(expression) = &return_statement.return_value {
            self.add_whitespace();
            self.visit_expression(expression);
        }
        self.add_text(";");
    }
    fn visit_while_statement(&mut self, while_statement: &ASTWhileStatement) {
        self.add_keyword("while");
        self.add_whitespace();
        self.visit_expression(&while_statement.condition);
        self.add_whitespace();
        self.visit_statement(&while_statement.body);
    }
    fn visit_block_statement(&mut self, block_statement: &ASTBlockStatement) {
        self.add_text("{");
        self.add_whitespace();
        for statement in &block_statement.statements {
            self.visit_statement(statement);
            self.add_whitespace();
        }
        self.add_text("}");
    }

    fn visit_if_statement(&mut self, if_statement: &ASTIfStatement) {
        self.add_keyword("if");
        self.add_whitespace();
        self.add_text("(");
        self.add_whitespace();
        self.visit_expression(&if_statement.condition);
        self.add_whitespace();
        self.add_text(")");
        self.add_whitespace();
        self.visit_statement(&if_statement.then_branch);
        self.add_whitespace();

        if let Some(else_branch) = &if_statement.else_branch {
            self.add_keyword("else");
            self.add_whitespace();
            self.visit_statement(&else_branch.else_statement);
        }
    }

    fn visit_let_statement(&mut self, let_statement: &ASTLetStatement) {
        if !let_statement.is_mut {
            self.add_text("const");
            self.add_whitespace();
        }

        if let Some(t) = &let_statement.type_annotation {
            if let Some(cpp_t) = to_cpp::translate_type(&t, &self.types) {
                self.add_text(&cpp_t.name);

                if !self.includes.contains(&cpp_t.library) {
                    self.includes.push(cpp_t.library);
                }
            } else {
                self.add_text(&t.lexeme);
            }
        } else {
            self.add_text("auto");
        }
        self.add_whitespace();

        self.add_text(let_statement.identifier.lexeme.as_str());
        self.add_whitespace();
        self.add_text("=");
        self.add_whitespace();
        self.visit_expression(&let_statement.initializer);
        self.add_text(";");
    }

    fn visit_statement(&mut self, statement: &ASTStatement) {
        Self::do_visit_statement(self, statement);
    }

    fn visit_std_call_expression(&mut self, std_call_expression: &ASTStdCallExpression) {
        let fn_name = format!(
            "{}{}{}",
            &std_call_expression.std_keyword.lexeme,
            &std_call_expression.double_colon.lexeme,
            &std_call_expression.identifier.lexeme
        );

        if let Some(library) = to_cpp::get_library(&fn_name, &self.std_names) {
            self.add_text(&fn_name);

            if !self.includes.contains(&library) {
                self.includes.push(library);
            }
        } else {
            self.add_text(&fn_name); //unknown std function error
        }

        self.add_text("(");
        for (i, argument) in std_call_expression.arguments.iter().enumerate() {
            if i != 0 {
                self.add_text(",");
                self.add_whitespace();
            }
            self.visit_expression(argument);
        }
        self.add_text(")");
        self.add_text(";");
    }

    fn visit_call_expression(&mut self, call_expression: &ASTCallExpression) {
        self.add_text(&call_expression.identifier.lexeme);
        self.add_text("(");
        for (i, argument) in call_expression.arguments.iter().enumerate() {
            if i != 0 {
                self.add_text(",");
                self.add_whitespace();
            }
            self.visit_expression(argument);
        }
        self.add_text(")");
        self.add_text(";");
    }

    fn visit_assignment_expression(&mut self, assignment_expression: &ASTAssignmentExpression) {
        self.add_variable(assignment_expression.identifier.lexeme.as_str());
        self.add_whitespace();
        self.add_text("=");
        self.add_whitespace();
        self.visit_expression(&assignment_expression.expression);
        self.add_text(";");
    }

    fn visit_variable_expression(&mut self, variable_expression: &ASTVariableExpression) {
        self.result
            .push_str(&format!("{}", variable_expression.identifier.lexeme,));
    }

    fn visit_number_expression(&mut self, number: &ASTNumberExpression) {
        self.result.push_str(&format!("{}", number.num.lexeme,));
    }

    fn visit_string_expression(&mut self, string: &ASTStringExpression) {
        self.result.push_str(&format!("{}", string.token.lexeme,));
    }

    fn visit_boolean_expression(&mut self, boolean: &ASTBooleanExpression) {
        self.add_boolean(boolean.value);
    }

    fn visit_unary_expression(&mut self, unary_expression: &ASTUnaryExpression) {
        self.result
            .push_str(&format!("{}", unary_expression.operator.token.lexeme,));
        self.visit_expression(&unary_expression.operand);
    }

    fn visit_binary_expression(&mut self, binary_expression: &ASTBinaryExpression) {
        self.visit_expression(&binary_expression.left);
        self.add_whitespace();
        self.result
            .push_str(&format!("{}", binary_expression.operator.token.lexeme,));
        self.add_whitespace();
        self.visit_expression(&binary_expression.right);
    }

    fn visit_parenthesized_expression(
        &mut self,
        parenthesized_expression: &ASTParenthesizedExpression,
    ) {
        self.result.push_str(&format!("{}", "(",));
        self.visit_expression(&parenthesized_expression.expression);
        self.result.push_str(&format!("{}", ")",));
    }

    fn finalize(&mut self) {
        let formatted_includes: Vec<String> = self
            .includes
            .iter()
            .map(|s| format!("#include <{}>\n", s))
            .collect();

        self.result.insert_str(0, &formatted_includes.join("\n"));
    }
}

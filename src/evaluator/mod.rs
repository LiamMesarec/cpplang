mod evaluator;
mod evaluatorLang;
mod to_cpp;
use crate::evaluator::evaluator::ASTEvaluator;
use crate::evaluator::evaluatorLang::ASTEvaluatorLang;
use crate::parser::Ast;

pub fn evaluate(ast: &Ast) -> String {
    let mut eval = ASTEvaluator::new();
    ast.visit(&mut eval);
    return eval.result;
}

pub fn interpret(ast: &Ast) -> String {
    let mut eval = ASTEvaluatorLang::new();
    ast.visit(&mut eval);
    return format!("{}", eval.last_value.unwrap());
}

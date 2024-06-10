mod evaluator;
use crate::evaluator::evaluator::ASTEvaluator;
use crate::parser::Ast;

pub fn evaluate(ast: &Ast) -> String {
    let mut eval = ASTEvaluator::new();
    ast.visit(&mut eval);
    return eval.result;
}

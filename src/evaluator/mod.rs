mod evaluator;
use crate::parser::Ast;
use crate::evaluator::evaluator::ASTEvaluator;

pub fn evaluate(ast: &Ast) -> String {
    let mut eval = ASTEvaluator::new();
    ast.visit(&mut eval);
    return eval.result;
}

mod interpreter;
mod cpptranspiler;
mod to_cpp;
use crate::evaluator::cpptranspiler::ASTCppTranspiler;
use crate::evaluator::interpreter::ASTInterpreter;
use crate::parser::Ast;

pub fn cpptranspile(ast: &Ast) -> String {
    let mut eval = ASTCppTranspiler::new();
    ast.visit(&mut eval);
    return eval.result;
}

pub fn interpret(ast: &Ast) -> String {
    let mut eval = ASTInterpreter::new();
    ast.visit(&mut eval);
    return format!("{}", eval.last_value.unwrap());
}

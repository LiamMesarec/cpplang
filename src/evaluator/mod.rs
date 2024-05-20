use crate::parser::Node;
use crate::tokenizer::{Token, TokenInfo};
use std::collections::LinkedList;
//TODO make function for creating linked list than evaluate linked list to c++
#[derive(Debug)]
pub enum Error {
    Generic(TokenInfo, String),
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Generic(token_info, string) => write!(
                f,
                "Syntax error: unexpected token '{}' after {} on line {}",
                token_info.lexeme, string, token_info.start_position.row
            ),
        }
    }
}

struct EvaluatorInfo {
    ast: Box<Node>,
}

pub fn evaluate(ast: Box<Node>) -> Result<String, Error> {
    let evaluator_info = EvaluatorInfo { ast };
    Ok(evaluate_recursive(&evaluator_info.ast))
}

fn evaluate_recursive(node: &Node) -> String {
    let mut result = String::new();

    result.push_str(&evaluate_node(&node));

    for child in &node.children {
        result.push_str(&evaluate_recursive(child));
    }

    result
}

fn evaluate_node(node: &Node) -> String {
    match node.token_info.token {
        Token::Let => let_definition(&node),
        Token::Fn => function_definition(&node),
        Token::Equals | Token::Identifier | Token::Number => operator(&node),
        _ => String::new(),
    }
}

fn function_definition(node: &Node) -> String {
    let mut it = node.children.iter();

    let mut out: String = String::from("");

    if let Some(it_name) = it.next() {
        let mut parameter_list: String = String::from("");

        it.next();
        parameter_list.push_str("(");
        // while it.next is Identifier -> var_definition()
        parameter_list.push_str(")");

        if let Some(it_colon) = it.next() {
            let colon = &(*it_colon).token_info.token;
            if *colon == Token::Colon {
                out.push_str(&(*it_colon).children[0].token_info.lexeme);
                // match our type to c++ type
                out.push_str(&(*it_name).token_info.lexeme);
                out.push_str(&parameter_list);
            } else {
                out.push_str(&(*it_name).token_info.lexeme);
                out.push_str(&parameter_list);
                out.push_str(&(*it_colon).token_info.lexeme);
            }
        }
    }

    println!("{}", out);
    out
}

fn let_definition(node: &Node) -> String {
    let mut output = String::new();

    let mut it = node.children.iter();
    if let Some(it_name) = it.next() {
        let name = &(*it_name).token_info.lexeme;

        if let Some(_) = it.next() {
            if let Some(it_type) = it.next() {
                let _type = &(*it_type).token_info.lexeme;
                if let Some(it_assignment_operator) = it.next() {
                    let assignment_operator = &(*it_assignment_operator).token_info.lexeme;
                    if let Some(it_right_side) = it.next() {
                        let right_side = evaluate_node(&(*it_right_side));

                        output.push_str(&format!(
                            "const {} {} {} {};",
                            _type, name, assignment_operator, right_side
                        ));
                    }
                }
            }
        }
    }

    output
}

fn operator(node: &Node) -> String {
    let mut output = node.token_info.lexeme.clone();
    output.push_str(
        &node
            .children
            .iter()
            .map(|child| evaluate_node(&child))
            .collect::<Vec<String>>()
            .join(""),
    );
    output
}

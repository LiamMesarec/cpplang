use crate::parser::Node;
use crate::tokenizer::{Token, TokenInfo};
use std::collections::{HashMap, LinkedList};

mod to_cpp;

#[derive(Debug)]
pub enum Error {
    Generic(TokenInfo, String),
    Csv(String),
    MissingFunctionBody(),
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
            Error::Csv(file_path) => write!(f, "CSV error: {}", file_path),
            Error::MissingFunctionBody() => write!(f, "Evaluator error: Missing function body"),
        }
    }
}

struct EvaluatorInfo {
    ast: Box<Node>,
    types: HashMap<String, to_cpp::TypeInfo>,
}

pub fn it_or_err(it: Option<&Box<Node>>, error: Error) -> Result<&Box<Node>, Error> {
    if let Some(it_) = it {
        return Ok(it_);
    } else {
        return Err(error);
    }
}

pub fn evaluate(ast: Box<Node>) -> Result<String, Error> {
    match to_cpp::init_types() {
        Ok(types) => {
            let evaluator_info = EvaluatorInfo { ast, types };
            Ok(evaluate_recursive(&evaluator_info.ast, &evaluator_info)?)
        }
        Err(error) => Err(Error::Csv(error.to_string())),
    }
}

fn evaluate_recursive(node: &Node, evaluator_info: &EvaluatorInfo) -> Result<String, Error> {
    let mut result = evaluate_node(&node, &evaluator_info)?;

    for child in &node.children {
        result.push_str(&evaluate_node(child, &evaluator_info)?);
    }

    Ok(result)
}

fn evaluate_node(node: &Node, evaluator_info: &EvaluatorInfo) -> Result<String, Error> {
    match node.token_info.token {
        Token::Let => let_definition(&node, &evaluator_info),
        Token::Fn => function_definition(&node, &evaluator_info),
        Token::Equals | Token::Identifier | Token::Number => operator(&node, &evaluator_info),
        _ => Ok(String::new()), // Changed to return Ok()
    }
}

fn function_definition(node: &Node, evaluator_info: &EvaluatorInfo) -> Result<String, Error> {
    let mut it = node.children.iter();

    let mut out: String = String::from("");
    //println!("test1");
    if let Some(it_name) = it.next() {
        let mut parameter_list: String = String::from("");

        it.next();
        parameter_list.push_str("(");
        // while it.next is Identifier -> var_definition()
        parameter_list.push_str(") ");

        if let Some(it_colon) = it.next() {
            let colon = &(*it_colon).token_info.token;
            if *colon == Token::Colon {
                let type_token = &(*it_colon).children[0].token_info;
                if let Some(cpp_type) = to_cpp::translate_type(type_token, &evaluator_info.types) {
                    out.push_str(&cpp_type);
                } else {
                    out.push_str(&type_token.lexeme);
                }
                out.push_str(" ");
                out.push_str(&(*it_name).token_info.lexeme);
                out.push_str(&parameter_list);
            } else {
                out.push_str("void ");
                out.push_str(&(*it_name).token_info.lexeme);
                out.push_str(&parameter_list);
            }
        }
        let it_body = it_or_err(it.next(), Error::MissingFunctionBody())?;
        out.push_str("{ ");
        out.push_str(&evaluate_recursive(it_body, &evaluator_info)?);
        out.push_str(" }");
    }

    Ok(out)
}

fn let_definition(node: &Node, evaluator_info: &EvaluatorInfo) -> Result<String, Error> {
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
                        let right_side = evaluate_node(&(*it_right_side), &evaluator_info)?;

                        output.push_str(&format!(
                            "const {} {} {} {};",
                            _type, name, assignment_operator, right_side
                        ));
                    }
                }
            }
        }
    }

    Ok(output)
}

fn operator(node: &Node, evaluator_info: &EvaluatorInfo) -> Result<String, Error> {
    let mut output = node.token_info.lexeme.clone();
    output.push_str(
        &node
            .children
            .iter()
            .map(|child| evaluate_node(&child, &evaluator_info))
            .collect::<Result<Vec<String>, Error>>()?
            .join(""),
    );
    Ok(output)
}

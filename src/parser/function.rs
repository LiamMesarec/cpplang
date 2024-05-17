use crate::parser::body;
use crate::parser::{Error, Node, ParseResult, ParserInfo};
use crate::tokenizer::Token;
pub fn function(parser_info: &mut ParserInfo) -> ParseResult {
    let mut node = Node::new_box(&parser_info.current_token_info); //fn
    if parser_info.match_token(Token::Identifier) {
        let mut node_func = Node::new_box(&parser_info.current_token_info);
        //fn-main

        if !parser_info.match_token(Token::LeftParantheses) {
            return Err(Error::ExpectedStartingParantheses(
                parser_info.current_token_info.clone(),
            ));
        }

        let parameters = parameter_list(parser_info)?;

        node_func.children.push(parameters);

        if !parser_info.match_token(Token::RightParantheses) {
            return Err(Error::MissingClosingParantheses(
                parser_info.current_token_info.clone(),
            ));
        }

        node_func
            .children
            .push(Node::new_box(&parser_info.current_token_info));

        if parser_info.match_token(Token::Colon) {
            let mut node_colon = Node::new_box(&parser_info.current_token_info);

            if parser_info.match_token(Token::Identifier) {
                let mut node_return_type = Node::new_box(&parser_info.current_token_info);

                node_return_type.children.push(body::body(parser_info)?);

                node_colon.children.push(node_return_type);
                node_func.children.push(node_colon);
                node.children.push(node_func);

                return Ok(node);
            }
        } else {
            node_func.children.push(body::body(parser_info)?);
            node.children.push(node_func);
        }
    }

    Err(Error::InvalidAssignment(
        parser_info.current_token_info.clone(),
        parser_info.last_n_token_lexemes(3),
    ))
}

pub fn parameter_list(parser_info: &mut ParserInfo) -> ParseResult {
    let mut node = Node::new_box(&parser_info.current_token_info);

    while parser_info.match_token(Token::Identifier) {
        let mut node_identifier = Node::new_box(&parser_info.current_token_info);
        if !parser_info.match_token(Token::Colon) {
            return Err(Error::MissingType(
                parser_info.current_token_info.clone(),
                parser_info.last_n_token_lexemes(3),
            ));
        }

        let mut node_colon = Node::new_box(&parser_info.current_token_info);

        if !parser_info.match_token(Token::Identifier) {
            return Err(Error::MissingType(
                parser_info.current_token_info.clone(),
                parser_info.last_n_token_lexemes(3),
            ));
        }

        let node_type = Node::new_box(&parser_info.current_token_info);

        if !parser_info.match_token(Token::Comma) {
            node_colon.children.push(node_type);
            node_identifier.children.push(node_colon);
            node.children.push(node_identifier);
            break;
        } else {
            node_colon.children.push(node_type);
            node_identifier.children.push(node_colon);
            node.children.push(node_identifier);
        }

        node.children
            .push(Node::new_box(&parser_info.current_token_info));
    }

    Ok(node)
}

use crate::parser::operator;
use crate::parser::{Error, Node, ParseResult, ParserInfo};
use crate::tokenizer::Token;
use crate::parser::body;
pub fn function(parser_info: &mut ParserInfo) -> ParseResult {
    let mut node = Node::new_box(&parser_info.current_token_info);
    if parser_info.match_token(Token::Identifier) {
        node.children
            .push(Node::new_box(&parser_info.current_token_info));

        if !parser_info.match_token(Token::LeftParantheses) {
            return Err(Error::ExpectedStartingParantheses(
                parser_info.current_token_info.clone(),
            ));
        }

        let parameters = parameter_list(parser_info)?;

        node.children.push(parameters);

        if !parser_info.match_token(Token::RightParantheses) {
            return Err(Error::MissingClosingParantheses(
                parser_info.current_token_info.clone(),
            ));
        }

        node.children
            .push(Node::new_box(&parser_info.current_token_info));

        if parser_info.match_token(Token::Colon) {
            node.children
                .push(Node::new_box(&parser_info.current_token_info));

            if parser_info.match_token(Token::Identifier) {
                node.children
                    .push(Node::new_box(&parser_info.current_token_info));

                node.children.push(body::body(parser_info)?);
                return Ok(node);
            }
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
        node.children
            .push(Node::new_box(&parser_info.current_token_info));

        if !parser_info.match_token(Token::Colon) {
            return Err(Error::MissingType(
                parser_info.current_token_info.clone(),
                parser_info.last_n_token_lexemes(3),
            ));
        }

        node.children
            .push(Node::new_box(&parser_info.current_token_info));

        if !parser_info.match_token(Token::Identifier) {
            return Err(Error::MissingType(
                parser_info.current_token_info.clone(),
                parser_info.last_n_token_lexemes(3),
            ));
        }

        node.children
            .push(Node::new_box(&parser_info.current_token_info));

        if !parser_info.match_token(Token::Comma) {
            break;
        }

        node.children
            .push(Node::new_box(&parser_info.current_token_info));
    }

    Ok(node)
}

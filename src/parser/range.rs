use crate::parser::{Node, ParserInfo, ParseResult, Error};
use crate::tokenizer::Token;

pub fn declaration(parser_info: &mut ParserInfo, mut parent: Box<Node>) -> ParseResult {
    if !parser_info.match_token(Token::Identifier) {
        return Err(Error::InvalidFor(parser_info.current_token_info.clone(), String::from("Expected variable declaration")));
    }

    parent.children.push(Node::new_box(&parser_info.current_token_info));

    Ok(Box::new(*parent))
}

pub fn expression(parser_info: &mut ParserInfo, mut parent: Box<Node>) -> ParseResult {
    if parser_info.match_token(Token::Identifier) {
        parent.children.push(Node::new_box(&parser_info.current_token_info));
        return Ok(parent);
    }

    if parser_info.match_token(Token::Number) {
        parent.children.push(Node::new_box(&parser_info.current_token_info));

        if parser_info.match_token(Token::Range) {
            parent.children.push(Node::new_box(&parser_info.current_token_info));

            if parser_info.match_token(Token::Number) {
                parent.children.push(Node::new_box(&parser_info.current_token_info));
            }

        }

    }

    Ok(parent)
}

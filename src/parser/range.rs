use crate::parser::{Error, Node, ParseResult, ParserInfo};
use crate::tokenizer::Token;

pub fn declaration(parser_info: &mut ParserInfo) -> ParseResult {
    if !parser_info.match_token(Token::Identifier) {
        return Err(Error::InvalidFor(
            parser_info.current_token_info.clone(),
            String::from("Expected variable declaration"),
        ));
    }

    let node = Node::new_box(&parser_info.current_token_info);

    Ok(node)
}

pub fn expression(parser_info: &mut ParserInfo) -> ParseResult {
    if parser_info.match_token(Token::Identifier) {
        return Ok(Node::new_box(&parser_info.current_token_info));
    }

    if parser_info.match_token(Token::Number) {
        let mut node = Node::new_box(&parser_info.current_token_info);

        if parser_info.match_token(Token::Range) {
            node
                .children
                .push(Node::new_box(&parser_info.current_token_info));

            if parser_info.match_token(Token::Number) {
                node
                    .children
                    .push(Node::new_box(&parser_info.current_token_info));
                return Ok(node);
            }
        }
    }

    return Err(Error::InvalidFor(
            parser_info.current_token_info.clone(),
            String::from("Invalid expression"),
        ));
}

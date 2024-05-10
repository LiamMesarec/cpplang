use crate::parser::body;
use crate::parser::operator;

use crate::parser::{Node, ParseResult, ParserInfo, Error};
use crate::tokenizer::Token;

///struct {
///    name: type
///    name: type
///    *...*
///}
pub fn struct_(parser_info: &mut ParserInfo) -> ParseResult {
    let mut node = Node::new_box(&parser_info.current_token_info);

    node.children.push(operator(parser_info)?);

    if !parser_info.match_token(Token::LeftBraces) {
        return Err(Error::ExpectedStartingBrackets(
            parser_info.current_token_info.clone(),
        ));
    }

    let mut struct_body = Node::new_box(&parser_info.current_token_info);

    while parser_info.match_token(Token::Identifier) {
        struct_body.children.push(Node::new_box(&parser_info.current_token_info));

        if !parser_info.match_token(Token::Colon) {
            //TODO ERROR
            return Err(Error::MissingClosingBrackets(
                parser_info.current_token_info.clone(),
            ));
        }

        struct_body.children.push(Node::new_box(&parser_info.current_token_info));

        if !parser_info.match_token(Token::Identifier) {
            //TODO ERROR
            return Err(Error::MissingClosingBrackets(
                parser_info.current_token_info.clone(),
            ));
        }

        struct_body.children.push(Node::new_box(&parser_info.current_token_info));
    }

    if !parser_info.match_token(Token::RightBraces) {
        return Err(Error::MissingClosingBrackets(
            parser_info.current_token_info.clone(),
        ));
    }

    struct_body.children
        .push(Node::new_box(&parser_info.current_token_info));

    node.children.push(struct_body);
    Ok(node)
}

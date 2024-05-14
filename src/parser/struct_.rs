use crate::parser::operator;

use crate::parser::{Error, Node, ParseResult, ParserInfo};
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

    struct_body
        .children
        .push(var_definition_list(parser_info, struct_body.clone())?);

    if !parser_info.match_token(Token::RightBraces) {
        return Err(Error::MissingClosingBrackets(
            parser_info.current_token_info.clone(),
        ));
    }

    struct_body
        .children
        .push(Node::new_box(&parser_info.current_token_info));

    node.children.push(struct_body);
    Ok(node)
}

pub fn var_definition_list(parser_info: &mut ParserInfo, mut parent: Box<Node>) -> ParseResult {
    while parser_info.match_token(Token::Identifier) {
        let mut definition = Node::new_box(&parser_info.current_token_info);

        if !parser_info.match_token(Token::Colon) {
            return Err(Error::MissingType(
                parser_info.current_token_info.clone(),
                parser_info.last_n_token_lexemes(3),
            ));
        }

        definition
            .children
            .push(Node::new_box(&parser_info.current_token_info));

        if !parser_info.match_token(Token::Identifier) {
            return Err(Error::MissingType(
                parser_info.current_token_info.clone(),
                parser_info.last_n_token_lexemes(3),
            ));
        }

        definition
            .children
            .push(Node::new_box(&parser_info.current_token_info));

        parent.children.push(definition)
    }

    Ok(parent)
}

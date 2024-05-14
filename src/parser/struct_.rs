

use crate::parser::{Error, Node, ParseResult, ParserInfo};
use crate::tokenizer::Token;

///struct Name {
///    name: type
///    name: type
///    *...*
///}
pub fn struct_(parser_info: &mut ParserInfo) -> ParseResult {
    let mut node = Node::new_box(&parser_info.current_token_info);

    if !parser_info.match_token(Token::Identifier) {
        return Err(Error::Generic(
            parser_info.current_token_info.clone(),
            String::from("Expected struct name"),
        ));
    }

    node.children
        .push(Node::new_box(&parser_info.current_token_info));

    if !parser_info.match_token(Token::LeftBraces) {
        return Err(Error::ExpectedStartingBrackets(
            parser_info.current_token_info.clone(),
        ));
    }

    let mut body = var_definition_list(parser_info)?;

    if !parser_info.match_token(Token::RightBraces) {
        return Err(Error::MissingClosingBrackets(
            parser_info.current_token_info.clone(),
        ));
    }

    body
        .children
        .push(Node::new_box(&parser_info.current_token_info));

    node.children.push(body);
    Ok(node)
}

pub fn var_definition_list(parser_info: &mut ParserInfo) -> ParseResult {
    let mut node = Node::new_box(&parser_info.current_token_info);
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

        node.children.push(definition)
    }

    Ok(node)
}

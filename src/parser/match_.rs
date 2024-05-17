use crate::parser::operator;

use crate::parser::{Error, Node, ParseResult, ParserInfo};
use crate::tokenizer::Token;

///match statement { case => statement case => statement *...* _ => statement }
pub fn match_(parser_info: &mut ParserInfo) -> ParseResult {
    let mut node = Node::new_box(&parser_info.current_token_info);

    node.children.push(operator(parser_info)?);

    if !parser_info.match_token(Token::LeftBraces) {
        return Err(Error::ExpectedStartingBrackets(
            parser_info.current_token_info.clone(),
        ));
    }

    let mut node_opening_braces = Node::new_box(&parser_info.current_token_info);

    while parser_info.match_any_of(&[Token::Number, Token::Identifier]) {
        let mut node_match = Node::new_box(&parser_info.current_token_info);

        if !parser_info.match_token(Token::Arrow) {
            return Err(Error::ExpectedStartingBrackets(
                // make error for arrow
                parser_info.current_token_info.clone(),
            ));
        }

        let mut node_arrow = Node::new_box(&parser_info.current_token_info);

        node_arrow.children.push(operator(parser_info)?);

        node_match.children.push(node_arrow);

        node_opening_braces.children.push(node_match);
    }
    // TODO last case in match must be _
    if !parser_info.match_token(Token::RightBraces) {
        return Err(Error::MissingClosingBrackets(
            parser_info.current_token_info.clone(),
        ));
    }

    node_opening_braces
        .children
        .push(Node::new_box(&parser_info.current_token_info));

    node.children.push(node_opening_braces);

    Ok(node)
}

use crate::parser::bitwise;

use crate::parser::{Error, Node, ParseResult, ParserInfo};
use crate::tokenizer::Token;

pub fn body(parser_info: &mut ParserInfo) -> ParseResult {
    if !parser_info.match_token(Token::LeftBraces) {
        return Err(Error::ExpectedStartingBrackets(
            parser_info.current_token_info.clone(),
        ));
    }

    let mut node = Node::new_box(&parser_info.current_token_info);

    node.children.push(bitwise(parser_info)?);

    if !parser_info.match_token(Token::RightBraces) {
        return Err(Error::MissingClosingBrackets(
            parser_info.current_token_info.clone(),
        ));
    }

    Ok(node)
}

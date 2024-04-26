use crate::parser::body;
use crate::parser::operator;

use crate::parser::{Node, ParseResult, ParserInfo};
use crate::tokenizer::Token;

///if statement *body* else(optional) *body*
pub fn if_(parser_info: &mut ParserInfo) -> ParseResult {
    let mut node = Node::new_box(&parser_info.current_token_info);

    node.children.push(operator(parser_info)?);

    node.children.push(body::body(parser_info)?);

    if !parser_info.match_token(Token::Else) {
        return Ok(node);
    }

    node
        .children
        .push(Node::new_box(&parser_info.current_token_info));

    node.children.push(body::body(parser_info)?);
    Ok(node)
}

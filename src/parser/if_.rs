use crate::parser::body;
use crate::parser::operator;
use crate::parser::range;
use crate::parser::{Error, Node, ParseResult, ParserInfo};
use crate::tokenizer::Token;

///if statement *body* else(optional) *body*
pub fn if_(parser_info: &mut ParserInfo, mut parent: Box<Node>) -> ParseResult {
    parent.children.push(operator(parser_info)?);

    parent = body::body(parser_info, parent)?;

    if !parser_info.match_token(Token::Else) {
        return Ok(parent);
    }

    parent
        .children
        .push(Node::new_box(&parser_info.current_token_info));

    return Ok(body::body(parser_info, parent)?);
}

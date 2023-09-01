use std::collections::HashMap;

use crate::kparser::{KParserError, KParserTracer};
use crate::kproc_macros::KTokenStream;
use crate::proc_macro::TokenTree;
use crate::rust::ast_nodes::{AttrToken, AttributeToken, CondAttributeToken};
use crate::{build_error, kparser};
use crate::{check, trace};

use super::ast_nodes::{Attr, AttributeV2Token};

pub mod macros {
    #[macro_export]
    macro_rules! parse_attributes {
        ($stream:expr, $tracer:expr ) => {
            check_and_parser_attributes_v2($stream, $tracer)
        };
    }

    pub use parse_attributes;
}

pub mod prelude {
    pub use super::check_and_parser_attributes_v2;
    pub use super::macros::*;
}

#[deprecated(note = "Pleas use the check_and_parse_cond_attribute! macro")]
pub fn check_and_parse_cond_attribute(
    ast: &mut KTokenStream,
    tracer: &dyn KParserTracer,
) -> HashMap<String, AttrToken> {
    tracer.log("check and parse an attribute");
    tracer.log(format!("{:?}", ast.peek()).as_str());
    let mut attrs = HashMap::new();
    if ast.match_tok("#") {
        let _ = ast.advance();
        tracer.log(format!("{:?}", ast.peek()).as_str());

        if ast.match_tok("!") {
            let _ = ast.advance();
            // check []
        } else if ast.is_group() {
            // check (
            if let TokenTree::Group(_) = ast.lookup(2) {
                let name = ast.advance();
                let _ = ast.advance();
                // keep parsing the conditional attribute
                // FIXME: parse a sequence of attribute
                let attr = check_and_parse_attribute(ast).unwrap();
                let conf_attr = CondAttributeToken {
                    name: name.to_owned(),
                    value: attr,
                };
                attrs.insert(name.to_string(), AttrToken::CondAttr(conf_attr));
            } else {
                // parsing the normal attribute
                // FIXME: parse a sequence of attribute
                if let Some(attr) = check_and_parse_attribute(&mut ast.to_ktoken_stream()) {
                    // consume group token
                    let _ = ast.advance();
                    attrs.insert(attr.name.to_string(), AttrToken::Attr(attr));
                    return attrs;
                }
            }
        }
    }
    attrs
}

#[deprecated(note = "Pleas use the check_and_parse_cond_attribute! macro")]
pub fn check_and_parse_attribute(ast: &mut KTokenStream) -> Option<AttributeToken> {
    let name = ast.advance();
    // FIXME: check if it is a valid name
    if !ast.is_end() && ast.match_tok("=") {
        let _ = ast.advance();
        let value = ast.advance();
        return Some(AttributeToken {
            name,
            value: Some(value),
        });
    }
    Some(AttributeToken { name, value: None })
}

pub fn check_and_parser_attributes_v2<T: KParserTracer + ?Sized>(
    stream: &mut KTokenStream,
    tracer: &T,
) -> kparser::Result<HashMap<String, AttributeV2Token>> {
    trace!(tracer, "checking and parsing attributes");

    let mut attrs = HashMap::new();
    // Parsing case where there are multiple attributes on one fields
    while stream.match_tok("#") {
        check!("#", stream.advance())?;
        let inner_attr = stream.match_tok("!").then(|| stream.next());
        let (identifier, attr) = check_and_parse_attribute_v2(stream, tracer)?;
        let attr = if inner_attr.is_some() {
            AttributeV2Token::InnerAttribute(attr)
        } else {
            AttributeV2Token::OuterAttribute(attr)
        };
        attrs.insert(identifier, attr);
    }
    Ok(attrs)
}

pub fn check_and_parse_attribute_v2<T: KParserTracer + ?Sized>(
    stream: &mut KTokenStream,
    tracer: &T,
) -> kparser::Result<(String, Attr)> {
    // unwrap the group given by the `#[]`
    let mut inner_stream = stream.to_ktoken_stream();
    trace!(
        tracer,
        "Attribute parsing: inner stream `{:?}`",
        inner_stream
    );
    let raw_attr = stream.advance();
    let identifier = inner_stream.advance();
    if inner_stream.is_end() {
        return Ok((
            identifier.to_string(),
            Attr {
                path: Vec::new(),
                identifier,
                value: None,
                raw_attr,
            },
        ));
    }
    let (_, value) = match inner_stream.peek() {
        TokenTree::Group(_) => check_and_parse_attribute_v2(&mut inner_stream, tracer)?,
        TokenTree::Punct(punt) => {
            trace!(tracer, "Attribute parsing: separator `{punt}`");
            inner_stream.next();
            let value = inner_stream.advance();
            (
                String::new(),
                Attr {
                    path: Vec::new(),
                    identifier: value.clone(),
                    value: None,
                    raw_attr: value,
                },
            )
        }
        // FIXME in case of multiple attribute this will panic
        _ => {
            return Err(build_error!(
                inner_stream.peek().clone(),
                "Error while parsing an attribute, token `{}` not expected",
                inner_stream.peek()
            ))
        }
    };
    Ok((
        identifier.to_string(),
        Attr {
            path: Vec::new(),
            identifier,
            value: Some(value.into()),
            raw_attr,
        },
    ))
}

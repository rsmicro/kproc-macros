use std::collections::HashMap;

use crate::kparser::KParserTracer;
use crate::kproc_macros::KTokenStream;
use crate::proc_macro::TokenTree;
use crate::rust::ast_nodes::{AttrToken, AttributeToken, CondAttributeToken};

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

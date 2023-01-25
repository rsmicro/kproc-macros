use crate::kparser::KParserTracer;
use crate::kproc_macros::KTokenStream;
use crate::proc_macro::TokenTree;
use crate::rust::ast_nodes::{AttrToken, AttributeToken, CondAttributeToken};

pub fn check_and_parse_cond_attribute<'c>(
    ast: &'c mut KTokenStream,
    tracer: &dyn KParserTracer,
) -> Option<AttrToken> {
    tracer.log("check and parse an attribute");
    tracer.log(format!("{:?}", ast.peek()).as_str());
    if ast.match_tok("#") {
        let _ = ast.advance();
        tracer.log(format!("{:?}", ast.peek()).as_str());

        if ast.match_tok("!") {
            let _ = ast.advance();
            // check []
        } else if ast.is_group() {
            // check (
            if let TokenTree::Group(_) = ast.lookup(2) {
                let name = ast.advance().to_owned();
                let _ = ast.advance();
                // keep parsing the conditional attribute
                // FIXME: parse a sequence of attribute
                let attr = check_and_parse_attribute(ast).unwrap();
                let conf_attr = CondAttributeToken {
                    name: name.to_owned(),
                    value: attr,
                };
                return Some(AttrToken::CondAttr(conf_attr));
            } else {
                // parsing the normal attribute
                // FIXME: parse a sequence of attribute
                if let Some(attr) = check_and_parse_attribute(&mut ast.to_ktoken_stream()) {
                    // consume group token
                    let _ = ast.advance();
                    return Some(AttrToken::Attr(attr));
                }
            }
        }
    }
    None
}

pub fn check_and_parse_attribute<'c>(ast: &'c mut KTokenStream) -> Option<AttributeToken> {
    let name = ast.advance().to_owned();
    // FIXME: check if it is a valid name
    if !ast.is_end() && ast.match_tok("=") {
        let _ = ast.advance();
        let value = ast.advance().to_owned();
        return Some(AttributeToken {
            name: name.to_owned(),
            value: Some(value.to_owned()),
        });
    }
    Some(AttributeToken {
        name: name.to_owned(),
        value: None,
    })
}

//! API to parse the rust struct provided as
//! TokenStream.
use std::collections::HashMap;

use crate::diagnostic::{KDiagnInfo, KDiagnostic};
use crate::eassert_eq;
use crate::kparser::KParserTracer;
use crate::kproc_macros::KTokenStream;
use crate::proc_macro::TokenTree;
use crate::rust::ast::RustAST;
use crate::rust::ast_nodes::{FieldToken, FieldTyToken, StructToken};

use super::ast_nodes::{AttrToken, AttributeToken, CondAttributeToken};
use super::core::*;

/// parsing a rust data structure inside a AST that will be easy to
/// manipulate and use by a compiler
pub fn parse_struct<'c>(ast: &'c mut KTokenStream, tracer: &dyn KParserTracer) -> RustAST {
    let visibility = if let Some(vs) = parse_visibility_identifier(ast) {
        let res = Some(vs.clone());
        ast.next();
        res
    } else {
        None
    };
    let tok = ast.advance().to_owned();
    eassert_eq!(
        "struct",
        tok.to_string(),
        tok,
        format!("expected struct keyword found {}", tok)
    );
    let name = ast.advance().to_owned();
    let generics = parse_decl_generics_and_lifetime(ast, tracer);
    tracer.log(format!("Struct generics ty: {:?}", generics).as_str());

    let mut group = ast.to_ktoken_stream();
    let fields = parse_struct_fields(&mut group, tracer);

    //FIXME: store informatio about attribute inside the
    // struct
    let stru = StructToken {
        visibility: visibility.to_owned(),
        name,
        fields,
        generics,
    };
    RustAST::Struct(stru)
}

pub fn parse_struct_fields(ast: &mut KTokenStream, tracer: &dyn KParserTracer) -> Vec<FieldToken> {
    let mut fields = vec![];
    while !ast.is_end() {
        let attr = if let Some(attr) = check_and_parse_cond_attribute(ast, tracer) {
            tracer.log(format!("attribute found: {:?}", attr).as_str());
            Some(attr)
        } else {
            None
        };
        tracer.log(format!("after token {:?}", ast.peek()).as_str());
        let mut field = parse_struct_field(ast, tracer);
        if let Some(attr) = attr {
            // FIXME: improve this solution, I want to search in O(1)
            // the attribute field and had the field as well
            field.attrs.insert(attr.name(), attr);
        }
        //FIXME: LOG me thanks!
        fields.push(field);
    }
    return fields;
}

pub fn parse_struct_field(ast: &mut KTokenStream, tracer: &dyn KParserTracer) -> FieldToken {
    // name filed
    let visibility = if let Some(vs) = parse_visibility_identifier(ast) {
        let res = Some(vs.clone());
        ast.next();
        res
    } else {
        None
    };
    let field_name = ast.advance().to_owned();
    // : separator
    let separator = ast.advance().clone();
    eassert_eq!(
        ":",
        separator.to_string(),
        separator,
        format!("expected `:` but found {}", separator)
    );

    let ty = parse_field_ty(ast, tracer);
    tracer.log(format!("top type field: {}", ty).as_str());

    FieldToken {
        visibility: visibility.to_owned(),
        name: field_name.to_owned(),
        ty,
        attrs: HashMap::new(),
    }
}

/// parse the field type as an AST element.
///
/// FIXME: support no reference and mutable field for the moment!
/// please feel free to contribute
pub fn parse_field_ty(ast: &mut KTokenStream, tracer: &dyn KParserTracer) -> FieldTyToken {
    tracer.log("parsing field ty");
    let ty_ref = check_and_parse_ref(ast);
    let lifetime = check_and_parse_lifetime(ast);
    let ty_mutability = check_and_parse_mut(ast);
    let dyn_tok = check_and_parse_dyn(ast);

    let field_ty = ast.advance().clone();
    tracer.log(format!("Type: {field_ty}").as_str());
    let mut generics = vec![];

    if ast.match_tok("<") {
        let _ = ast.advance();
        while !ast.match_tok(">") {
            let ty = parse_field_ty(ast, tracer);
            tracer.log(format!("{:?}", ty).as_str());
            generics.push(ty);
            tracer.log("exit from generics while");
        }
        let tok = ast.advance();
        eassert_eq!(
            ">",
            tok.to_string(),
            tok,
            format!("expected > but found {}", tok.to_string())
        );
    }

    if ast.match_tok(",") {
        let tok = ast.advance().to_owned();
        eassert_eq!(
            ",",
            tok.to_string().as_str(),
            tok,
            format!("terminator `,` not found but found `{}`", tok.to_string())
        );
    }
    tracer.log("finish decoding");

    FieldTyToken {
        reference: ty_ref,
        mutable: ty_mutability,
        lifetime: lifetime.to_owned(),
        dyn_tok,
        generics: generics.to_owned(),
        name: field_ty.to_owned(),
    }
}

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

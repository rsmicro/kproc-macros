//! API to parse the rust struct provided as
//! TokenStream.
use super::core::*;
use super::kattr::check_and_parse_cond_attribute;
use crate::diagnostic::{KDiagnInfo, KDiagnostic};
use crate::eassert_eq;
use crate::kparser::KParserTracer;
use crate::kproc_macros::KTokenStream;
use crate::rust::ast_nodes::{FieldToken, StructToken};
use crate::rust::ty::parse_ty;
use std::collections::HashMap;

/// parsing a rust data structure inside a AST that will be easy to
/// manipulate and use by a compiler
pub fn parse_struct<'c>(ast: &'c mut KTokenStream, tracer: &dyn KParserTracer) -> StructToken {
    let visibility = if let Some(vs) = check_and_parse_visibility(ast) {
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
    let generics = check_and_parse_generics_params(ast, tracer);
    tracer.log(format!("Struct generics ty: {:?}", generics).as_str());

    let mut group = ast.to_ktoken_stream();
    let fields = parse_struct_fields(&mut group, tracer);

    let struct_tok = StructToken {
        visibility: visibility.to_owned(),
        name,
        fields,
        generics,
    };
    tracer.log(format!("`parse_struct` result {:#?}", struct_tok).as_str());
    struct_tok
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
        let mut field = parse_struct_ty(ast, tracer);
        if let Some(attr) = attr {
            // FIXME: improve this solution, I want to search in O(1)
            // the attribute field and had the field as well
            field.attrs.insert(attr.name(), attr);
        }
        fields.push(field);
    }
    return fields;
}

pub fn parse_struct_ty(ast: &mut KTokenStream, tracer: &dyn KParserTracer) -> FieldToken {
    // name filed
    let visibility = if let Some(vs) = check_and_parse_visibility(ast) {
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

    let ty = parse_ty(ast, tracer);
    tracer.log(format!("top type field: {}", ty).as_str());

    FieldToken {
        visibility: visibility.to_owned(),
        identifier: field_name.to_owned(),
        ty,
        attrs: HashMap::new(),
    }
}

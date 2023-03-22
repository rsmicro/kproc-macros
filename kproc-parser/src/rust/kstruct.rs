//! API to parse the rust struct provided as
//! TokenStream.
use std::collections::HashMap;

use crate::kparser::{KParserError, KParserTracer};
use crate::kproc_macros::KTokenStream;
use crate::rust::ast_nodes::{FieldToken, StructToken};
use crate::rust::ty::parse_ty;
use crate::{check, parse_visibility, trace};

use super::core::*;
use super::kattr::check_and_parse_cond_attribute;

/// parsing a rust data structure inside a AST that will be easy to
/// manipulate and use by a compiler
pub fn parse_struct<'c>(
    ast: &'c mut KTokenStream,
    tracer: &dyn KParserTracer,
) -> Result<StructToken, KParserError> {
    let visibility = parse_visibility!(ast);
    let tok = ast.advance();
    check!("struct", tok)?;

    let name = ast.advance();
    let generics = check_and_parse_generics_params(ast, tracer);
    trace!(tracer, "Struct generics ty: {:?}", generics);

    let mut group = ast.to_ktoken_stream();
    let fields = parse_struct_fields(&mut group, tracer)?;

    let struct_tok = StructToken {
        visibility,
        name,
        fields,
        generics,
    };
    trace!(tracer, "`parse_struct` result {:#?}", struct_tok);
    Ok(struct_tok)
}

pub fn parse_struct_fields(
    ast: &mut KTokenStream,
    tracer: &dyn KParserTracer,
) -> Result<Vec<FieldToken>, KParserError> {
    let mut fields = vec![];
    while !ast.is_end() {
        let attr = check_and_parse_cond_attribute(ast, tracer);
        trace!(tracer, "after token {:?}", ast.peek());
        let mut field = parse_struct_ty(ast, tracer)?;
        field.attrs.extend(attr);
        fields.push(field);
    }
    Ok(fields)
}

pub fn parse_struct_ty(
    ast: &mut KTokenStream,
    tracer: &dyn KParserTracer,
) -> Result<FieldToken, KParserError> {
    // name filed
    let visibility = parse_visibility!(ast);
    let field_name = ast.advance();
    let separator = ast.advance();
    check!(":", separator)?;

    let ty = parse_ty(ast, tracer);
    trace!(tracer, "top type field: {ty}");

    let field = FieldToken {
        visibility,
        identifier: field_name,
        ty,
        attrs: HashMap::new(),
    };
    Ok(field)
}

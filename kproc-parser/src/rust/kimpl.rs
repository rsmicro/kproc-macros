//! API to parse a rust `impl`
use crate::kparser::{KParserError, KParserTracer};
use crate::kproc_macros::KTokenStream;
use crate::rust::ast_nodes::ImplToken;
use crate::rust::core::check_and_parse_generics_params;
use crate::rust::kattr::check_and_parse_cond_attribute;
use crate::{check, trace};

/// helper function that allow to parse an impl block
pub fn parse_impl<'c>(
    ast: &'c mut KTokenStream,
    tracer: &dyn KParserTracer,
) -> Result<ImplToken, KParserError> {
    let attr = check_and_parse_cond_attribute(ast, tracer);
    let impl_tok = ast.advance();
    check!("impl", impl_tok)?;
    let generics = check_and_parse_generics_params(ast, tracer);
    let name = ast.advance();
    let _for_ty = if ast.match_tok("for") {
        // FIXME: parsing the generic and lifetime usage
        let for_tok = ast.advance();
        check!("for", for_tok)?;
        Some(ast.advance())
    } else {
        None
    };

    // FIXME: parse the where clause!

    // store the raw content of the block because there
    // if the user want parse it,
    // it has all the necessary tools for parse it.
    let impl_block = ast.unwrap_group_as_stream();
    trace!(tracer, "{:#?}", impl_block);
    let impl_tok = ImplToken {
        attributes: attr,
        generics,
        name,
        // FIXME: make an abstraction for this kind of type
        for_ty: None,
        raw_block: impl_block.into(),
    };
    Ok(impl_tok)
}

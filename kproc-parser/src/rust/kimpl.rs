//! API to parse a rust `impl`
use crate::kparser::{self, KParserError, KParserTracer};
use crate::kproc_macros::KTokenStream;
use crate::rust::ast_nodes::ImplToken;
use crate::rust::core::check_and_parse_bounds;
use crate::rust::kattr::check_and_parse_cond_attribute;
use crate::rust::kfunc::parse_fn;
use crate::{check, trace};

/// helper function that allow to parse an impl block
pub fn parse_impl<'c>(
    toks: &'c mut KTokenStream,
    tracer: &dyn KParserTracer,
) -> kparser::Result<ImplToken> {
    let attr = check_and_parse_cond_attribute(toks, tracer);
    let impl_tok = toks.advance();
    check!("impl", impl_tok)?;
    let generics = check_and_parse_bounds(toks, tracer)?;
    let name = toks.advance();
    let _for_ty = if toks.match_tok("for") {
        // FIXME: parsing the generic and lifetime usage
        let for_tok = toks.advance();
        check!("for", for_tok)?;
        Some(toks.advance())
    } else {
        None
    };

    // FIXME: parse the where clause!

    // store the raw content of the block because there
    // if the user want parse it,
    // it has all the necessary tools for parse it.
    let raw_impl_block = toks.unwrap_group_as_stream();
    let mut impl_block = toks.to_ktoken_stream();

    let mut funs = Vec::new();
    while !impl_block.is_end() {
        // FIXME we Suppose to have all the function and no
        // extra stuff
        let fn_tok = parse_fn(&mut impl_block, tracer)?;
        funs.push(fn_tok);
    }

    let impl_tok = ImplToken {
        attributes: attr,
        generics,
        name,
        // FIXME: make an abstraction for this kind of type
        for_ty: None,
        raw_block: raw_impl_block.into(),
        functions: funs,
    };

    trace!(tracer, "impl tok parserd: {:#?}", impl_tok);
    Ok(impl_tok)
}

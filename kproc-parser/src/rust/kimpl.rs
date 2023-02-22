//! API to parse a rust `impl`
use crate::eassert_eq;
use crate::kparser::KParserTracer;
use crate::kproc_macros::KTokenStream;
use crate::rust::ast_nodes::ImplToken;
use crate::rust::core::check_and_parse_generics_params;
use crate::rust::kattr::{check_and_parse_attribute, check_and_parse_cond_attribute};

/// helper function that allow to parse an impl block
pub fn parse_impl<'c>(ast: &'c mut KTokenStream, tracer: &dyn KParserTracer) -> ImplToken {
    let attr = check_and_parse_cond_attribute(ast, tracer);
    let impl_tok = ast.advance().to_owned();
    eassert_eq!(
        "impl",
        impl_tok.to_string(),
        impl_tok,
        format!("expected `impl` found `{}`", impl_tok.to_string())
    );
    let generics = check_and_parse_generics_params(ast, tracer);
    let name = ast.advance().to_owned();
    let _for_ty = if ast.match_tok("for") {
        // FIXME: parsing the generic and lifetime usage
        let for_tok = ast.advance().to_owned();
        eassert_eq!(
            "for",
            for_tok.to_string(),
            for_tok,
            format!("expected `for` found `{}`", for_tok.to_owned())
        );
        Some(ast.advance())
    } else {
        None
    };

    // FIXME: parse the where clause!

    // store the raw content of the block because there
    // if the user want parse it,
    // it has all the necessary tools for parse it.
    let impl_block = ast.advance().to_owned();
    tracer.log(&format!("{:#?}", impl_block));
    ImplToken {
        attributes: attr,
        generics,
        name,
        // FIXME: make an abstraction for this kind of type
        for_ty: None,
        impl_block,
    }
}

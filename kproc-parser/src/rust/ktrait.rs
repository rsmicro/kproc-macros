use crate::kparser::{KParserError, KParserTracer};
use crate::kproc_macros::KTokenStream;
use crate::rust::core::*;
use crate::rust::kattr::check_and_parse_cond_attribute;
use crate::rust::kfunc::parse_fn;
use crate::{check, parse_visibility, trace};

use super::ast_nodes::TraitToken;

/// helper function that allow to parse an trait definition
pub fn parse_trait(
    ast: &mut KTokenStream,
    tracer: &dyn KParserTracer,
) -> Result<TraitToken, KParserError> {
    trace!(tracer, "start parning the trait");
    let attrs = check_and_parse_cond_attribute(ast, tracer);

    let vist = parse_visibility!(ast);
    let trait_tok = ast.advance();
    check!("trait", trait_tok)?;
    let name = ast.advance();
    trace!(tracer, "`{name}` checking bounds on `{:?}`", ast.peek());
    let generics = check_and_parse_bounds(ast, tracer)?;
    trace!(tracer, "checking the trait block");
    let raw_block = ast.unwrap_group_as_stream();
    let mut block = ast.to_ktoken_stream();

    let mut funs = Vec::new();
    while !block.is_end() {
        trace!(tracer, "checking body");
        let fn_tok = parse_fn(&mut block, tracer)?;
        funs.push(fn_tok);
    }

    let trait_tok = TraitToken {
        attrs,
        visibility: vist,
        ident: name,
        generics,
        inn_attrs: None, // FIXME: parse this
        associated_items: vec![],
        raw_block,
        functions: funs,
    };
    trace!(tracer, "trait token result: {:#?}", trait_tok);
    Ok(trait_tok)
}

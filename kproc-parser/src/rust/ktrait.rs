use crate::kparser::{KParserError, KParserTracer};
use crate::kproc_macros::KTokenStream;
use crate::rust::core::*;
use crate::{check, parse_visibility, trace};

use super::ast_nodes::TraitToken;

/// helper function that allow to parse an trait definition
pub fn parse_trait<'c>(
    ast: &'c mut KTokenStream,
    tracer: &dyn KParserTracer,
) -> Result<TraitToken, KParserError> {
    trace!(tracer, "start parning the trait");
    let vist = parse_visibility!(ast);
    let trait_tok = ast.advance();
    check!("trait", trait_tok)?;
    let name = ast.advance();
    let generics = check_and_parse_generics_params(ast, tracer);

    let raw_block = ast.unwrap_group();

    let trait_tok = TraitToken {
        visibility: vist,
        ident: name,
        generics,
        inn_attrs: None, // FIXME: parse this
        associated_items: vec![],
        raw_block,
    };
    trace!(tracer, "trait token result: {:#?}", trait_tok);
    Ok(trait_tok)
}

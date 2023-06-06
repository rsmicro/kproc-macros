//! Type parser to allow a more flexible
//! parser.
use super::ast_nodes::TyToken;
use crate::build_error;
use crate::kparser;
use crate::kparser::{KParserError, KParserTracer};
use crate::kproc_macros::KTokenStream;
use crate::kproc_macros::MatchTok;
use crate::rust::ast_nodes::LifetimeParam;
use crate::rust::ast_nodes::TyKind;
use crate::rust::core::check_and_parse_dyn;
use crate::rust::core::check_and_parse_lifetime;
use crate::rust::core::check_and_parse_mut;
use crate::rust::core::check_and_parse_ref;
use crate::trace;

/// parse the field type as an AST element, and return the type field,
/// if found, otherwise if the type is a Trait bound return None.
pub fn parse_ty(
    stream: &mut KTokenStream,
    tracer: &dyn KParserTracer,
) -> kparser::Result<Option<TyToken>> {
    let ref_tok = check_and_parse_ref(stream);
    let lifetime = check_and_parse_lifetime(stream).and_then(|lifetime| {
        Some(LifetimeParam {
            lifetime_or_label: lifetime,
            bounds: None,
        })
    });
    let dyn_tok = check_and_parse_dyn(stream);
    let mut_tok = check_and_parse_mut(stream);
    let identifier = stream.advance();
    trace!(tracer, "type identifier {identifier}");
    trace!(tracer, "is at the EOF {}", stream.is_end());
    trace!(tracer, "next item {}", stream.peek());
    // in the case of the function parameters here  we ca be
    // at the end of the stream
    //
    // In addition the basics types do not need
    // the generics check, and in the case of EOF
    // checking the generic will panic the parser.
    let mut generics: Option<Vec<TyToken>> = None;
    if !stream.is_end() && (stream.has(2) && !stream.lookup(2).match_tok(":")) {
        let subtypes = parse_recursive_ty(stream, tracer)?;
        if !subtypes.is_empty() {
            generics = Some(subtypes);
        }
        let sep = stream.peek().to_owned();

        // token allowed as stop words for the type parser
        if ![",", ">", ";", ":"].contains(&sep.to_string().as_str()) && !stream.is_group() {
            assert!(false, "seprator found {:?}", sep);
        }
        // token to consume, but in this case
        // we do not consume the `>`
        // because we are in a recursive call,
        // and the token is a stop word for the
        // root recursive call.
        if [","].contains(&sep.to_string().as_str()) {
            stream.next();
        }
    } else if !stream.is_end() && (stream.has(2) && stream.match_tok(":")) {
        return Ok(None);
    }

    Ok(Some(TyToken {
        identifier,
        dyn_tok,
        ref_tok,
        mut_tok,
        lifetime,
        generics,
        bounds: vec![],
        // FIXME: try to understnad how to parse the `TyKind` or if we
        // really need it.
        kind: TyKind::NeverType,
    }))
}

pub fn parse_recursive_ty(
    ast: &mut KTokenStream,
    tracer: &dyn KParserTracer,
) -> kparser::Result<Vec<TyToken>> {
    let mut types: Vec<TyToken> = vec![];
    if ast.match_tok("<") {
        ast.next(); // consume `<``
        while !ast.match_tok(">") {
            let ty = parse_ty(ast, tracer)?.ok_or(build_error!(
                ast.peek().clone(),
                "failing to parse the type, this is a bug, please report it"
            ))?;
            types.push(ty);
        }
        ast.next(); // consume the `>` toks
    }
    Ok(types)
}

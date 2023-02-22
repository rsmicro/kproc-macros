//! Type parser to allow a more flexible
//! parser.
use super::ast_nodes::TyToken;
use crate::kparser::KParserTracer;
use crate::kproc_macros::KTokenStream;
use crate::rust::ast_nodes::LifetimeParam;
use crate::rust::ast_nodes::TyKind;
use crate::rust::core::check_and_parse_dyn;
use crate::rust::core::check_and_parse_lifetime;
use crate::rust::core::check_and_parse_ref;

/// parse the field type as an AST element.
pub fn parse_ty(ast: &mut KTokenStream, tracer: &dyn KParserTracer) -> TyToken {
    tracer.log(format!("parsing field ty {:?}", ast.peek()).as_str());
    // FIXME: a possible type can start with `(....)``

    let ref_tok = check_and_parse_ref(ast);
    // FIXME: parsing the lifetime
    // FIXME: check the kind of the type
    let lifetime = if let Some(lifetime) = check_and_parse_lifetime(ast) {
        Some(LifetimeParam {
            lifetime_or_label: lifetime,
            bounds: None,
        })
    } else {
        None
    };

    let dyn_tok = check_and_parse_dyn(ast);
    let identifier = ast.advance().to_owned();
    let generics = parse_recursive_ty(ast, tracer);
    let sep = ast.peek().to_owned();

    // token allowed as stop words for the type parser
    if ![",", ">"].contains(&sep.to_string().as_str()) {
        assert!(false, "found {sep}");
    }

    // token to consume, but in this case
    // we do not consume the `>`
    // because we are in a recursive call,
    // and the token is a stop word for the
    // root recursive call.
    if [","].contains(&sep.to_string().as_str()) {
        ast.next();
    }

    let generics = if generics.is_empty() {
        None
    } else {
        Some(generics)
    };

    TyToken {
        identifier,
        dyn_tok,
        ref_tok,
        lifetime,
        generics,
        // FIXME: try to understnad how to parse the `TyKind`
        kind: TyKind::NeverType,
    }
}

pub fn parse_recursive_ty(ast: &mut KTokenStream, tracer: &dyn KParserTracer) -> Vec<TyToken> {
    let mut types = vec![];
    if ast.match_tok("<") {
        ast.next(); // consume `<``
        while !ast.match_tok(">") {
            let ty = parse_ty(ast, tracer);
            types.push(ty);
        }
        ast.next(); // consume the `>` toks
    }
    types
}

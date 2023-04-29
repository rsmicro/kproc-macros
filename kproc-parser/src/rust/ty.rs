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
use crate::trace;

// FIXME return a Result type here
/// parse the field type as an AST element.
pub fn parse_ty(stream: &mut KTokenStream, tracer: &dyn KParserTracer) -> TyToken {
    trace!(tracer, "parsing field ty {:?}", stream.peek());

    let ref_tok = check_and_parse_ref(stream);
    let lifetime = if let Some(lifetime) = check_and_parse_lifetime(stream) {
        Some(LifetimeParam {
            lifetime_or_label: lifetime,
            bounds: None,
        })
    } else {
        None
    };

    let dyn_tok = check_and_parse_dyn(stream);
    let identifier = stream.advance();
    trace!(tracer, "type identifier {identifier}");
    trace!(tracer, "is at the EOF {}", stream.is_end());
    // in the case of the function parameters here  we ca be
    // at the end of the stream
    //
    // In addition the basics types do not need
    // the generics check, and in the case of EOF
    // checking the generic will panic the parser.
    let mut generics: Option<Vec<TyToken>> = None;
    if !stream.is_end() {
        generics = Some(parse_recursive_ty(stream, tracer));
        let sep = stream.peek().to_owned();

        // token allowed as stop words for the type parser
        if ![",", ">", ";"].contains(&sep.to_string().as_str()) && !stream.is_group() {
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
    }

    trace!(tracer, "end of the parse_ty");
    TyToken {
        identifier,
        dyn_tok,
        ref_tok,
        lifetime,
        generics,
        // FIXME: try to understnad how to parse the `TyKind` or if we
        // really need it.
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

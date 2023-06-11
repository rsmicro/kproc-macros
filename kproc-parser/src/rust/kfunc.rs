//! kfunc is the module that it is used to
//! parse the function like rust syntax.
use crate::kparser::{KParserError, KParserTracer};
use crate::kproc_macros::KTokenStream;
use crate::proc_macro::TokenTree;
use crate::rust::core::{check_and_parse_bounds, check_and_parse_return_type};
use crate::rust::kattr::check_and_parse_cond_attribute;
use crate::rust::ty::parse_ty;
use crate::{build_error, check, parse_visibility, trace};

use super::ast_nodes::{MethodDeclToken, TyToken};
use super::core::{check_and_parse_fn_qualifier, check_is_fun_with_visibility};

/// helper function to parse the method/function declaration
///
/// ```norun
/// fn answer_to_life_the_universe_and_everything() -> i32 {
///     return 42;
/// }
///
/// fn foo<A, B>(x: A, y: B);
///
/// fn foo<T>(x: &[T]) where T: Debug {
/// // details elided
/// }
///
/// async fn regular_example() { }
///
/// async unsafe fn unsafe_example() { }
/// ```
pub fn parse_fn(
    toks: &mut KTokenStream,
    tracer: &dyn KParserTracer,
) -> Result<MethodDeclToken, KParserError> {
    trace!(tracer, "Start parsing fn");

    let attrs = check_and_parse_cond_attribute(toks, tracer);
    let visibility = check_is_fun_with_visibility(toks).then(|| parse_visibility!(toks).unwrap());
    let qualifier = check_and_parse_fn_qualifier(toks);
    let fn_tok = toks.advance();
    check!("fn", fn_tok)?;

    let ident = toks.advance();
    trace!(
        tracer,
        "function name {ident} and next tok: {:?}",
        toks.peek()
    );
    let generics = check_and_parse_bounds(toks, tracer)?;
    trace!(tracer, "starting parsing fn params");
    let raw_params = toks.unwrap_group_as_stream();
    let mut params_stream: KTokenStream = raw_params.clone().into();
    let params = parse_fn_params(&mut params_stream, tracer)?;
    trace!(tracer, "fn parametes {:?}", params);
    toks.next();

    // FIXME: parse where clouse
    let rt_ty = check_and_parse_return_type(toks, tracer)?;
    trace!(
        tracer,
        "return type {:?} next should be the body function: {:?}",
        rt_ty,
        toks.peek()
    );

    // The trait has a function declaration without
    // body.
    let body = if toks.is_group() {
        let body = toks.unwrap_group_as_stream();
        toks.next();
        Some(body)
    } else {
        let toks = toks.advance();
        check!(";", toks)?;
        None
    };

    let method = MethodDeclToken {
        attrs,
        visibility,
        qualifier,
        ident,
        generics,
        raw_params,
        params,
        raw_body: body,
        return_ty: rt_ty,
    };
    Ok(method)
}

pub fn parse_fn_params(
    raw_params: &mut KTokenStream,
    tracer: &dyn KParserTracer,
) -> Result<Vec<(TokenTree, TyToken)>, KParserError> {
    trace!(
        tracer,
        "parsing fn params from the following source: {:?}",
        raw_params
    );
    let mut params = Vec::new();
    // the stream of token that we get in
    // are the token that are inside a `(...)`
    // when in rust is a `TokenTree::Group` token
    while !raw_params.is_end() {
        // FIXME here we need to check the special case of
        // `&mut self`
        if raw_params.match_tok("&") || raw_params.match_tok("self") | raw_params.match_tok("mut") {
            while !raw_params.is_end() && !raw_params.match_tok(",") {
                trace!(tracer, "`self` found `{:?}`", raw_params.advance());
            }
            if raw_params.is_end() {
                trace!(tracer, "end of the params stream.");
                break;
            }
            if raw_params.match_tok(",") {
                check!(",", raw_params.advance())?;
            }
        }
        let ident = raw_params.advance();
        trace!(tracer, "parameters name `{ident}`");
        check!(":", raw_params.advance())?;
        let ty = parse_ty(raw_params, tracer)?.ok_or(build_error!(
            ident.clone(),
            "fails to parse the rust type, this is a bug, please open a issue"
        ))?;
        trace!(tracer, "param found `{ident}: {ty}`");
        params.push((ident, ty));
        // keep going, or there are more token, or we finish the stream
        // but the while will check the last case.
    }
    Ok(params)
}

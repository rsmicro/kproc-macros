//! kfunc is the module that it is used to
//! parse the function like rust syntax.
use crate::kparser::{KParserError, KParserTracer};
use crate::kproc_macros::KTokenStream;
use crate::rust::core::check_and_parse_return_type;
use crate::{check, parse_visibility, trace};

use super::ast_nodes::MethodDeclToken;
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
pub fn parse_fn<'c>(
    toks: &'c mut KTokenStream,
    tracer: &dyn KParserTracer,
) -> Result<MethodDeclToken, KParserError> {
    trace!(tracer, "Start parsing fn");

    let visibility = if check_is_fun_with_visibility(toks) {
        parse_visibility!(toks)
    } else {
        None
    };

    let qualifier = check_and_parse_fn_qualifier(toks);
    let fn_tok = toks.advance();
    check!("fn", fn_tok)?;

    let ident = toks.advance();
    trace!(
        tracer,
        "function name {ident} and next tok: {:#?}",
        toks.peek()
    );
    // FIXME: check and parse generics
    let params = toks.unwrap_group_as_stream();
    toks.next();
    // FIXME: parse where clouse
    let rt_ty = check_and_parse_return_type(toks, tracer);
    trace!(
        tracer,
        "return type {:?} next should be the body function: {:#?}",
        rt_ty,
        toks.peek()
    );
    let body = toks.unwrap_group_as_stream();
    toks.next();

    let method = MethodDeclToken {
        visibility,
        qualifier,
        ident,
        raw_params: params,
        raw_body: body,
    };
    Ok(method)
}

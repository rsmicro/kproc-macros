//! kfunc is the module that it is used to
//! parse the function like rust syntax.
use crate::kparser::{KParserError, KParserTracer};
use crate::kproc_macros::KTokenStream;
use crate::parse_visibility;

use super::ast_nodes::MethodDeclToken;

/// helper function to parse the method/function declaration
///
/// ```rust
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
    let visibility = parse_visibility!(toks);

    todo!()
}

//! API to parse a rust `impl`
use super::ast::RustAST;
use crate::diagnostic::{KDiagnInfo, KDiagnostic};
use crate::eassert_eq;
use crate::kparser::KParserTracer;
use crate::kproc_macros::KTokenStream;

pub fn parse_impl<'c>(ast: &'c mut KTokenStream, _: &dyn KParserTracer) -> RustAST {
    let impl_tok = ast.advance().to_owned();
    eassert_eq!(
        "impl",
        impl_tok.to_string(),
        impl_tok,
        format!("expected `impl` found `{}`", impl_tok.to_string())
    );

    todo!()
}

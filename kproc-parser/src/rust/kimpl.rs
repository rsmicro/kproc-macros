//! API to parse a rust `impl`
use super::ast::RustAST;
use crate::diagnostic::{KDiagnInfo, KDiagnostic};
use crate::eassert_eq;
use crate::kparser::KParserTracer;
use crate::kproc_macros::KTokenStream;
use crate::rust::core::parse_decl_generics_and_lifetime;

pub fn parse_impl<'c>(ast: &'c mut KTokenStream, tracer: &dyn KParserTracer) -> RustAST {
    let impl_tok = ast.advance().to_owned();
    eassert_eq!(
        "impl",
        impl_tok.to_string(),
        impl_tok,
        format!("expected `impl` found `{}`", impl_tok.to_string())
    );
    let _ = parse_decl_generics_and_lifetime(ast, tracer);
    let for_tok = ast.advance().to_owned();
    eassert_eq!(
        "for",
        for_tok.to_string(),
        for_tok,
        format!("expected `for` found `{}`", for_tok.to_owned())
    );
    todo!()
}

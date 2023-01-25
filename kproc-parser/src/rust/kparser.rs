use super::ast_nodes::ImplToken;
use super::kimpl::parse_impl;
use super::{ast_nodes::StructToken, kstruct::parse_struct};
use crate::proc_macro::TokenStream;
use crate::{
    kparser::{DummyTracer, KParserTracer},
    kproc_macros::KTokenStream,
};

/// generic struct to implement the rust parser
pub struct RustParser<'tcx> {
    pub tracer: &'tcx dyn KParserTracer,
}

impl<'tcx> RustParser<'tcx> {
    pub fn new() -> Self {
        RustParser {
            tracer: &DummyTracer {},
        }
    }

    pub fn with_tracer(tracer: &'tcx dyn KParserTracer) -> Self {
        RustParser { tracer }
    }

    pub fn parse_struct(&self, stream: &TokenStream) -> StructToken {
        let mut stream = KTokenStream::from(stream);
        parse_struct(&mut stream, self.tracer)
    }

    pub fn parse_impl(&self, stream: &TokenStream) -> ImplToken {
        let mut stream = KTokenStream::from(stream);
        parse_impl(&mut stream, self.tracer)
    }
}

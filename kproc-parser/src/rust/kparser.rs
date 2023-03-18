use super::ast_nodes::{ImplToken, TraitToken};
use super::kimpl::parse_impl;
use super::ktrait::parse_trait;
use super::{ast_nodes::StructToken, kstruct::parse_struct};
use crate::proc_macro::TokenStream;
use crate::{
    kparser::{DummyTracer, KParserTracer},
    kproc_macros::KTokenStream,
};

macro_rules! unwrap {
    ($res: expr, $def: expr) => {
        $res.unwrap_or_else(|err| {
            err.emit();
            $def
        })
    };
}

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
        let result = parse_struct(&mut stream, self.tracer);
        unwrap!(result, StructToken::default())
    }

    pub fn parse_impl(&self, stream: &TokenStream) -> ImplToken {
        let mut stream = KTokenStream::from(stream);
        let result = parse_impl(&mut stream, self.tracer);
        unwrap!(result, ImplToken::default())
    }

    pub fn parser_trait(&self, stream: &TokenStream) -> TraitToken {
        let mut stram = KTokenStream::from(stream);
        let result = parse_trait(&mut stram, self.tracer);
        unwrap!(result, TraitToken::default())
    }
}

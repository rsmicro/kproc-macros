use super::ast_nodes::{ImplToken, MethodDeclToken, TopLevelNode, TraitToken};
use super::kenum::{self, EnumToken};
use super::kfunc::parse_fn;
use super::kimpl::parse_impl;
use super::ktrait::parse_trait;
use super::{ast_nodes::StructToken, kstruct::parse_struct};
use crate::kparser::KParserError;
use crate::proc_macro::TokenStream;
use crate::{build_error, trace};
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

    pub fn parse(&self, stream: &TokenStream) -> Result<TopLevelNode, KParserError> {
        let mut ast = KTokenStream::new(stream);
        let first = ast.peek().clone();
        if let Ok(tok) = parse_struct(&mut ast, self.tracer) {
            return Ok(tok.into());
        } else {
            trace!(self.tracer, "error fom `parse_struct`");
        }

        let mut ast = KTokenStream::new(stream);
        if let Ok(tok) = parse_impl(&mut ast, self.tracer) {
            return Ok(tok.into());
        } else {
            trace!(self.tracer, "error fro `parse_impl`");
        }

        let mut ast = KTokenStream::new(stream);
        if let Ok(tok) = parse_trait(&mut ast, self.tracer) {
            return Ok(tok.into());
        }

        let mut ast = KTokenStream::new(stream);
        if let Ok(tok) = parse_fn(&mut ast, self.tracer) {
            return Ok(tok.into());
        }
        let err = build_error!(first, "Token Stream sequence not known");
        Err(err)
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

    pub fn parse_trait(&self, stream: &TokenStream) -> TraitToken {
        let mut stram = KTokenStream::from(stream);
        let result = parse_trait(&mut stram, self.tracer);
        unwrap!(result, TraitToken::default())
    }

    pub fn parse_fn(&self, stream: &TokenStream) -> MethodDeclToken {
        let mut stream = KTokenStream::from(stream);
        let result = parse_fn(&mut stream, self.tracer);
        unwrap!(result, MethodDeclToken::default())
    }

    pub fn parse_enum(&self, stream: &TokenStream) -> EnumToken {
        let mut stream = KTokenStream::from(stream);
        let result = kenum::parse(&mut stream, self.tracer);
        unwrap!(result, EnumToken::default())
    }
}

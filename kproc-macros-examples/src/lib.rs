use kproc_parser::kparser::{DummyTracer, KParserTracer};
use kproc_parser::rust::kparser::RustParser;
use kproc_parser::{error, trace};
use proc_macro::TokenStream;

mod gen;
use crate::gen::*;

struct Tracer;

impl KParserTracer for Tracer {
    fn log(&self, msg: &str) {
        eprintln!("\x1b[93mkproc-tracing\x1b[1;97m {msg}");
    }
}

/// Mock this will be some parse macros
#[proc_macro_derive(RustBuilder, attributes(build))]
pub fn derive_rust(input: TokenStream) -> TokenStream {
    let tracer = DummyTracer {};
    let parser = RustParser::with_tracer(&tracer);
    let ast = parser.parse_struct(&input);
    let toks = generate_impl(&ast);
    trace!(tracer, "{}", toks);
    toks
}

#[proc_macro_attribute]
pub fn derive_impl(_: TokenStream, input: TokenStream) -> TokenStream {
    let tracer = DummyTracer {};
    let parser = RustParser::with_tracer(&tracer);

    let ast = parser.parse_impl(&input);
    trace!(tracer, "{}", ast);
    ast.to_string().parse().unwrap()
}

#[proc_macro_attribute]
pub fn default_impl(_: TokenStream, input: TokenStream) -> TokenStream {
    let tracer = Tracer {};
    let parsr = RustParser::with_tracer(&tracer);

    let _ = parsr.parse_trait(&input);
    input
}

#[proc_macro_attribute]
pub fn derive_fn(_: TokenStream, input: TokenStream) -> TokenStream {
    let tracer = Tracer {};
    let parser = RustParser::with_tracer(&tracer);
    let ast = parser.parse_fn(&input);
    trace!(tracer, "function AST: {:#?}", ast);
    input
}

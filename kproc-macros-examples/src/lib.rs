use kproc_parser::kparser::KParserTracer;
use kproc_parser::rust::kparser::RustParser;
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
    let tracer = Tracer {};
    let parser = RustParser::with_tracer(&tracer);
    let ast = parser.parse_struct(&input);

    let toks = generate_impl(&ast);
    tracer.log(format!("{}", toks).as_str());
    toks
}

#[proc_macro_attribute]
pub fn derive_impl(_: TokenStream, input: TokenStream) -> TokenStream {
    let tracer = Tracer {};
    let parser = RustParser::with_tracer(&tracer);

    let ast = parser.parse_impl(&input);
    tracer.log(format!("{}", ast).as_str());
    ast.to_string().parse().unwrap()
}

#[proc_macro_attribute]
pub fn default_impl(_: TokenStream, input: TokenStream) -> TokenStream {
    let tracer = Tracer {};
    let parsr = RustParser::with_tracer(&tracer);

    let _ = parsr.parse_trait(&input);
    input
}

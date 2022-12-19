use kproc_parser::kparser::KParserTracer;
use kproc_parser::kproc_macros::KTokenStream;
use kproc_parser::proc_macro::TokenStream as TokenStreamV2;
use kproc_parser::rust::kimpl::parse_impl;
use kproc_parser::rust::kstruct::parse_struct;
use proc_macro::TokenStream;

mod gen;
use crate::gen::*;

struct Tracer;

impl KParserTracer for Tracer {
    fn log(&self, msg: &str) {
        eprintln!("{msg}");
    }
}

/// Mock this will be some parse macros
#[proc_macro_derive(RustBuilder, attributes(build))]
pub fn derive_rust(input: TokenStream) -> TokenStream {
    let tracer = Tracer {};
    let inputv2 = TokenStreamV2::from(input);
    let mut stream = KTokenStream::new(&inputv2);

    let ast = parse_struct(&mut stream, &tracer);

    let toks = generate_impl(&ast);
    tracer.log(format!("{}", toks).as_str());
    toks
}

#[proc_macro_attribute]
pub fn derive_impl(_: TokenStream, input: TokenStream) -> TokenStream {
    let tracer = Tracer {};
    let inputv2 = TokenStreamV2::from(input);
    let mut stream = KTokenStream::new(&inputv2);

    let ast = parse_impl(&mut stream, &tracer);
    tracer.log(format!("{}", ast).as_str());
    ast.to_token_stream()
}

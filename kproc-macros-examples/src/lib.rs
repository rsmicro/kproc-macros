use kproc_parser::kparser::{DummyTracer, KParserTracer};
use kproc_parser::kproc_macros::KTokenStream;
use kproc_parser::proc_macro::TokenStream as TokenStreamV2;
use kproc_parser::rust::ast::RustAST;
use kproc_parser::rust::ast_nodes::StructToken;
use kproc_parser::rust::rust_struct::parse_struct;
use proc_macro::TokenStream;

struct Tracer;

impl KParserTracer for Tracer {
    fn log(&self, msg: &str) {
        eprintln!("{msg}");
    }
}

/// Mock this will be some parse macros
#[proc_macro_derive(RustBuilder)]
pub fn derive_rust(input: TokenStream) -> TokenStream {
    let tracer = Tracer {};
    let inputv2 = TokenStreamV2::from(input);
    let mut stream = KTokenStream::new(&inputv2);

    let ast = parse_struct(&mut stream, &tracer);

    let toks = generate_impl(&ast);
    tracer.log(format!("{}", toks).as_str());
    toks
}

fn generate_impl(ast: &RustAST) -> TokenStream {
    let RustAST::Struct(StructToken {
        name,
        attributes,
        generics,
        ..
    }) = ast;
    let gen = if let Some(str_gen) = generics {
        format!("{}", str_gen)
    } else {
        "".to_owned()
    };
    let code = format!(
        "impl{} {}{} {{ \
                    fn get_{}(&self) -> {} {{ \
                       return self.{}.clone()\
                    }}\
                }}",
        gen, name, gen, attributes[0].name, attributes[0].ty, attributes[0].name,
    );
    code.parse().unwrap()
}

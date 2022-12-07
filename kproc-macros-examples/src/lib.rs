use kproc_parser::kparser::KParserTracer;
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

// FIXME: use the filed attribute to generate the get method when the attribute
// is specified!
fn generate_impl(ast: &RustAST) -> TokenStream {
    let RustAST::Struct(StructToken {
        name,
        fields: attributes,
        generics,
        ..
    }) = ast;
    let gen = if let Some(str_gen) = generics {
        format!("{}", str_gen)
    } else {
        "".to_owned()
    };
    let name_attr = attributes[0].name.to_string();
    let ty = attributes[0].ty.to_string();
    let code = format!(
        "impl{} {}{} {{ \
                    fn get_{name_attr}(&self) -> {ty} {{ \
                       return self.{name_attr}.clone()\
                    }} \
                       \
                    fn set_{name_attr}(&self, inner: {ty}) {{ }}
                }}",
        gen, name, gen,
    );
    code.parse().unwrap()
}

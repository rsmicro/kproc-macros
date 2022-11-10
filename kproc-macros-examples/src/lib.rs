use kproc_parser::kproc_macros::KTokenStream;
use kproc_parser::proc_macro::TokenStream as TokenStreamV2;
use kproc_parser::rust::rust_struct::parse_struct;
use proc_macro::TokenStream;

/// Mock this will be some parse macros
#[proc_macro_derive(RustBuilder)]
pub fn derive_rust(input: TokenStream) -> TokenStream {
    println!("{:?}", input);

    let inputv2 = TokenStreamV2::from(input);
    let mut stream = KTokenStream::new(&inputv2);
    let ast = parse_struct(&mut stream);

    ast.to_string().parse().unwrap()
}

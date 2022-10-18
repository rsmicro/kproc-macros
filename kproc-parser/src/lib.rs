//! kproc-macros is a minimal library to parse
//! specific language defined inside the procedural
//! procedural macros. This language can be Rust
//! itself ofcourse
use proc_macro::TokenStream;
mod kproc_macros;
mod rust;

use crate::rust::rust_struct::parse_struct;

/// Mock this will be some parse macros
#[proc_macro_derive(RustBuilder)]
pub fn derive_rust(input: TokenStream) -> TokenStream {
    println!("{:?}", input);
    let ast = parse_struct(&mut input.into_iter());
    ast.to_token_stream()
}

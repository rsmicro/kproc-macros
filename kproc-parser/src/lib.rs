//! kproc-macros is a minimal library to parse
//! specific language defined inside the procedural
//! procedural macros. This language can be Rust
//! itself ofcourse
pub mod kproc_macros;
pub mod rust;

pub mod proc_macros {
    pub use proc_macro2::TokenStream;
}

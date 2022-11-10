//! kproc-macros is a minimal library to parse
//! specific language defined inside the procedural
//! procedural macros. This language can be Rust
//! itself ofcourse
pub mod kproc_macros;
pub mod rust;

pub mod proc_macro {
    #[cfg(proc_macro_wrapper)]
    extern crate proc_macro2 as macros;

    #[cfg(any(not(proc_macro_wrapper), proc_macro))]
    extern crate proc_macro as macros;

    pub use macros::{TokenStream, TokenTree};
}

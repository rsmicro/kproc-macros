//! kproc-parser is a minimal procedural macros parser that
//! produce a convenient AST by including only the
//! necessary code.
#![feature(proc_macro_diagnostic)]
pub mod diagnostic;
pub mod kparser;
pub mod kproc_macros;
pub mod macros;
pub mod rust;

/// proc_macro by exporting just the correct
/// proc_macro API.
///
/// This allow to inject an external `proc_macro` API
/// like `proc_macro2`, and this is convenient for
/// library that do not implement directly a
/// procedural macro or avoid to include the external
/// dependencies just to mimic the `proc_macro` API.
///
/// The last case is particular useful when the parser
/// is injected inside the procedural macro code, such
/// as the linux kernel
pub mod proc_macro {
    #[cfg(proc_macro_wrapper)]
    extern crate proc_macro2 as macros;

    #[cfg(any(not(proc_macro_wrapper), proc_macro))]
    extern crate proc_macro as macros;

    pub use macros::*;
}

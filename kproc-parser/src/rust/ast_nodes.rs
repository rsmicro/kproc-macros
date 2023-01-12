//! Crate that implement an abstraction of all
//! the AST nodes like
//! `struct`, `fields`, `FielsTypes`
//!
//! Each implementation contains information
//! regarding the position in `KDiagnostic`.
use crate::diagnostic::KDiagnostic;
use crate::proc_macro::TokenTree;
use std::collections::{BTreeMap, HashMap};
use std::fmt::Display;

use super::fmt::fmt_lifetimes;

/// Strung token that allow to
/// decode a `struct` block.
///
/// Defined as described in
/// https://doc.rust-lang.org/stable/reference/items/structs.html
///
// FIXME: parse and add where clause
// FIXME: parse the TupleStruct
#[derive(Debug)]
pub struct StructToken {
    pub visibility: Option<TokenTree>,
    pub name: TokenTree,
    pub fields: Vec<FieldToken>,
    pub generics: Option<GenericParams>,
}

/// Generic Parameters token allow to
/// decode stream of token defined as described
/// in https://doc.rust-lang.org/stable/reference/items/generics.html
#[derive(Debug)]
pub struct GenericParams {
    /// part of the impl block where the
    /// lifetimes is stored.
    ///
    /// This is useful if it is needed some lookup
    /// or implementing any smart logic with
    /// it.
    ///
    /// Stored with the following idea
    /// key:value => for the code <'a: 'static, 'b: 'a, 'c: 'a + 'b>
    /// is translated with `a:static, b:a, c:a + b`
    pub lifetimes: BTreeMap<TokenTree, Vec<TokenTree>>,
    /// generics are declared.
    ///
    /// This is similar to the lifetime declaration.
    pub generics: BTreeMap<TokenTree, Vec<TokenTree>>,
}

impl Display for GenericParams {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut gen = String::new();
        gen += fmt_lifetimes(&self.lifetimes).unwrap().as_str();
        // FIXME: missing generics formatting
        write!(f, "<{}>", gen)
    }
}

/// struct filed token allow to decode the
/// struct fields defined as described in
/// https://doc.rust-lang.org/stable/reference/items/structs.html
#[derive(Debug)]
pub struct FieldToken {
    pub visibility: Option<TokenTree>,
    pub name: TokenTree,
    // FIXME: convert the struct in a single
    // type as described in https://doc.rust-lang.org/stable/reference/types.html#type-expressions
    pub ty: FieldTyToken,
    pub attrs: HashMap<String, AttrToken>,
}

impl Display for FieldToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut vis = String::new();
        if let Some(viss) = &self.visibility {
            vis = viss.to_string()
        }
        write!(f, "{} {}: {}", vis, self.name, self.ty)
    }
}

#[derive(Debug, Clone)]
pub struct FieldTyToken {
    pub reference: Option<TokenTree>,
    pub mutable: Option<TokenTree>,
    // FIXME: this is a partial work, it is better
    // to parse the lifetimes and the generics
    // with the struct `GenericParams`.
    pub lifetime: Option<TokenTree>,
    pub generics: Vec<FieldTyToken>,
    pub dyn_tok: Option<TokenTree>,
    pub name: TokenTree,
}

impl Display for FieldTyToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut prefix = String::new();
        if let Some(refer) = &self.reference {
            prefix += refer.to_string().as_str();
        }

        // FIXME: can be more than one!
        if let Some(lifetime) = &self.lifetime {
            prefix += lifetime.to_string().as_str();
        }

        if let Some(mutable) = &self.mutable {
            prefix += format!(" {mutable}").as_str();
        }

        let mut generics = "".to_owned();
        if self.generics.len() > 0 {
            generics += "<";
            for generic in &self.generics {
                generics += format!("{}, ", generic.to_string()).as_str();
            }
            generics = generics.strip_suffix(",").unwrap_or(&generics).to_owned();
            generics += ">";
        }

        write!(f, "{prefix} {}{}", self.name, generics)
    }
}

/// An attribute is a general, free-form metadatum that is
/// interpreted according to name, convention, language,
/// and compiler version. Attributes are modeled
/// on Attributes in ECMA-335, with the syntax coming
/// from ECMA-334 (C#).
///
/// A list of possible attribute syntax is:
///
/// ```rust
/// #![allow(unused)]
/// fn main() {
///     // General metadata applied to the enclosing module or crate.
///     #![crate_type = "lib"]
///     // A function marked as a unit test
///     #[test]
///     fn test_foo() {
///         /* ... */
///     }
///
///     // A conditionally-compiled module
///     #[cfg(target_os = "linux")]
///     mod bar {
///     /* ... */
///     }
///
///     // A lint attribute used to suppress a warning/error
///     #[allow(non_camel_case_types)]
///     type int8_t = i8;
///
///     // Inner attribute applies to the entire function.
///     fn some_unused_variables() {
///         #![allow(unused_variables)]
///             let x = ();
///             let y = ();
///             let z = ();
///      }
/// }
/// ```
#[derive(Debug, Clone)]
pub struct CondAttributeToken {
    /// name of the attribute
    pub name: TokenTree,
    /// value of the attribut contained inside the `()`
    pub value: AttributeToken,
}

/// Inner attribute is the simplest attribute that you can find
/// inside the sintax, in fact this kind of struct contains
/// information regarding the attribute itself.
///
/// ```rust
/// // #[pin_project]
/// // #[key = value]
/// // #![pin_project]
/// // #![key = value]
/// ```
#[derive(Debug, Clone)]
pub struct AttributeToken {
    /// name of the attribute
    pub name: TokenTree,
    /// an optional value of the attribute
    pub value: Option<TokenTree>,
}

#[derive(Debug, Clone)]
pub enum AttrToken {
    Attr(AttributeToken),
    CondAttr(CondAttributeToken),
}

impl AttrToken {
    pub fn name(&self) -> String {
        match self {
            AttrToken::Attr(tok) => tok.name.to_string(),
            AttrToken::CondAttr(tok) => tok.name.to_string(),
        }
    }
}

/// AST Token to store information about an
/// `impl` block.
///
/// Reference: https://doc.rust-lang.org/stable/reference/items/implementations.html
#[derive(Debug)]
pub struct ImplToken {
    pub generics: Option<GenericParams>,
    /// The name of the impl Block
    pub name: TokenTree,
    /// for the type where the impl block is implemented for
    // FIXME: we should abstract this token in a better way?
    // or just rename it?
    pub for_ty: Option<FieldTyToken>,
    /// Content of the impl block
    ///
    /// It is stored the raw block because
    /// the kparser library expose all the primitive
    /// to parse this kind of token tree, and this
    /// will make a slim version of the library.
    pub impl_block: TokenTree,
}

impl KDiagnostic for StructToken {}
impl KDiagnostic for FieldToken {}
impl KDiagnostic for FieldTyToken {}
impl KDiagnostic for AttributeToken {}
impl KDiagnostic for CondAttributeToken {}
impl KDiagnostic for ImplToken {}

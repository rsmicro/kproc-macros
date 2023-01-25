//! Crate that implement an abstraction of all
//! the AST nodes like
//! `struct`, `fields`, `FielsTypes`
//!
//! Each implementation contains information
//! regarding the position in `KDiagnostic`.
use crate::diagnostic::KDiagnostic;
use crate::kproc_macros::OrderedTokenTree;
use crate::proc_macro::TokenTree;
use std::collections::{BTreeMap, HashMap};
use std::fmt::Display;
use std::rc::Rc;

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
#[derive(Debug, Clone)]
pub struct GenericParams {
    pub params: Vec<GenericParam>,
}

#[derive(Debug, Clone)]
pub enum GenericParam {
    LifetimeParam(LifetimeParam),
    TypeParam(),
    // FIXME: support the const params
}

#[derive(Debug, Clone)]
pub struct LifetimeParam {
    pub lifetime_or_label: TokenTree,
    pub bounds: Option<TypeParamBound>,
}

#[derive(Debug, Clone)]
pub struct TypeParam {
    pub identifier: TokenTree,
    pub bounds: Option<TypeParamBound>,
    pub ty: Option<FieldTyToken>,
}

#[derive(Debug, Clone)]
pub enum TypeParamBound {
    Lifetime(Vec<TokenTree>),
    // FIXME: complete this
    TraitBound,
}

impl Display for GenericParams {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let gen = String::new();
        // FIXME: missing formatting
        write!(f, "<{}>", gen)
    }
}

/// struct filed token allow to decode the
/// struct fields defined as described in
/// https://doc.rust-lang.org/stable/reference/items/structs.html
#[derive(Debug)]
pub struct FieldToken {
    pub attrs: HashMap<String, AttrToken>,
    pub visibility: Option<TokenTree>,
    pub name: TokenTree,
    // FIXME: convert the struct in a single
    // type as described in https://doc.rust-lang.org/stable/reference/types.html#type-expressions
    pub ty: FieldTyToken,
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

/// parsing the type of the filed, where this will be
/// defined with the following grammar
/// https://doc.rust-lang.org/stable/reference/types.html#type-expressions
///
// FIXME(vincenzopalazzo): I am not happy with this solution, so
// happy to receive some feedback regarding it. In this case
// would be good an enum or a filed regarding the kind of the type
#[derive(Debug, Clone)]
pub struct FieldTyToken {
    pub reference: Option<TokenTree>,
    pub lifetimes: Option<BTreeMap<OrderedTokenTree, Vec<TokenTree>>>,
    pub name: TokenTree,
    pub dyn_tok: Option<TokenTree>,
    pub generics: Option<Rc<FieldTyToken>>,
}

impl Display for FieldTyToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut prefix = String::new();
        if let Some(refer) = &self.reference {
            prefix += refer.to_string().as_str();
        }

        write!(f, "{prefix} {}{}", self.name, "")
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

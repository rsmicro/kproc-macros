//! Crate that implement an abstraction of all
//! the AST nodes like
//! `struct`, `fields`, `FielsTypes`
//!
//! Each implementation contains information
//! regarding the position in `KDiagnostic`.
use crate::proc_macro::TokenStream;

use crate::{
    kparser::{DummyTracer, KParserError},
    kproc_macros::KTokenStream,
    proc_macro::TokenTree,
};
use std::collections::HashMap;
use std::fmt::Display;

use super::{
    fmt::{fmt_generics, fmt_ty},
    kimpl::parse_impl,
    kstruct::parse_struct,
    ktrait::parse_trait,
};

pub trait TopLevelAST {
    fn span(&self) -> TokenTree;

    fn is_trait(&self) -> bool {
        false
    }

    fn is_struct(&self) -> bool {
        false
    }

    fn is_impl(&self) -> bool {
        false
    }

    fn is_fn(&self) -> bool {
        false
    }
}

pub enum TopLevelNode {
    Struct(StructToken),
    Trait(TraitToken),
    Impl(ImplToken),
    Fn(MethodDeclToken),
}

impl Display for TopLevelNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Impl(node) => write!(f, "{node}"),
            Self::Struct(node) => write!(f, "{node}"),
            Self::Trait(node) => write!(f, "{node}"),
            Self::Fn(node) => write!(f, "{node}"),
        }
    }
}

impl From<StructToken> for TopLevelNode {
    fn from(value: StructToken) -> Self {
        TopLevelNode::Struct(value)
    }
}

impl From<ImplToken> for TopLevelNode {
    fn from(value: ImplToken) -> Self {
        TopLevelNode::Impl(value)
    }
}

impl From<TraitToken> for TopLevelNode {
    fn from(value: TraitToken) -> Self {
        TopLevelNode::Trait(value)
    }
}

impl From<MethodDeclToken> for TopLevelNode {
    fn from(value: MethodDeclToken) -> Self {
        TopLevelNode::Fn(value)
    }
}

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
    pub attrs: HashMap<String, AttrToken>,
    pub visibility: Option<TokenTree>,
    pub name: TokenTree,
    pub fields: Vec<FieldToken>,
    pub generics: Option<GenericParams>,
}

impl TopLevelAST for StructToken {
    fn span(&self) -> TokenTree {
        self.name.clone()
    }

    fn is_struct(&self) -> bool {
        true
    }
}

impl Default for StructToken {
    fn default() -> Self {
        panic!()
    }
}

impl TryFrom<&TokenStream> for StructToken {
    type Error = KParserError;

    fn try_from(value: &TokenStream) -> Result<Self, Self::Error> {
        let mut stream = KTokenStream::new(&value);
        parse_struct(&mut stream, &DummyTracer {})
    }
}

impl Display for StructToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
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
    TypeParam(TyToken),
    Bounds(Bound), // FIXME: support the const params
}

impl GenericParam {
    pub fn add_bound(&mut self, bound: Bound) {
        match self {
            Self::TypeParam(param) => param.bounds.push(bound),
            Self::LifetimeParam(param) => param.bounds.push(bound),
            Self::Bounds(params) => params.add_bound(bound),
        }
    }
}

impl Display for GenericParam {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LifetimeParam(param) => write!(f, "{param}"),
            Self::TypeParam(param) => write!(f, "{param}"),
            Self::Bounds(bounds) => write!(f, "{bounds}"),
        }
    }
}

#[derive(Clone, Debug)]
pub enum Bound {
    Lifetime(LifetimeParam),
    Trait(TypeParam),
}

impl Bound {
    pub fn add_bound(&mut self, bound: Bound) {
        match self {
            Self::Trait(param) => param.bounds.push(bound),
            Self::Lifetime(param) => param.bounds.push(bound),
        }
    }
}

impl std::fmt::Display for Bound {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unimplemented!()
    }
}

/// 'a: 'static
#[derive(Debug, Clone)]
pub struct LifetimeParam {
    pub lifetime_or_label: TokenTree,
    pub bounds: Vec<Bound>,
}

impl std::fmt::Display for LifetimeParam {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut code = format!("'{}", self.lifetime_or_label);
        if !self.bounds.is_empty() {
            code += &format!(
                " {}",
                self.bounds
                    .iter()
                    .map(|bound| format!("{bound} +"))
                    .collect::<String>()
            );
            code = code.strip_suffix("+").unwrap_or(&code).to_owned();
        }
        write!(f, "{code}")
    }
}

#[derive(Debug, Clone)]
pub struct TypeParam {
    pub identifier: TokenTree,
    pub bounds: Vec<Bound>,
}

impl Display for GenericParams {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let gen = fmt_generics(self);
        write!(f, "{gen}")
    }
}

/// struct filed token allow to decode the
/// struct fields defined as described in
/// https://doc.rust-lang.org/stable/reference/items/structs.html
#[derive(Debug)]
pub struct FieldToken {
    pub attrs: HashMap<String, AttrToken>,
    pub visibility: Option<TokenTree>,
    pub identifier: TokenTree,
    pub ty: TyToken,
}

impl Display for FieldToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut vis = String::new();
        if let Some(viss) = &self.visibility {
            vis = viss.to_string()
        }
        write!(f, "{} {}: {}", vis, self.identifier, self.ty)
    }
}

/// TyKidn definition for the Rust language.
///
/// a formal defintion of it is available at
/// https://doc.rust-lang.org/stable/reference/types.html#type-expressions
#[derive(Debug, Clone)]
pub enum TyKind {
    ImplTrait,
    Parenthesized,
    TraitObject,
    TypePath,
    TupleType,
    NeverType,
    RawPointerType,
    ReferenceType,
    ArrayType,
    SliceType,
    InferredType,
    QualifiedPathInType,
    BareFunctionType,
    MacroInvocation,
}

/// parsing the type of the filed, where this will be
/// defined with the following grammar
/// https://doc.rust-lang.org/stable/reference/types.html#type-expressions
///
// FIXME(vincenzopalazzo): I am not happy with this solution, so
// happy to receive some feedback regarding it. In this case
// would be good an enum or a filed regarding the kind of the type
#[derive(Debug, Clone)]
pub struct TyToken {
    pub kind: TyKind,
    pub ref_tok: Option<TokenTree>,
    pub mut_tok: Option<TokenTree>,
    pub identifier: TokenTree,
    pub dyn_tok: Option<TokenTree>,
    pub lifetime: Option<LifetimeParam>,
    pub generics: Option<Vec<TyToken>>,
    pub bounds: Vec<Bound>,
}

impl Display for TyToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let code = fmt_ty(self);
        write!(f, "{code}")
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
    // return the name of the current attribute,
    // to inspect the value please considerer to
    // call the `self.attribute` method.
    pub fn name(&self) -> String {
        match self {
            Self::Attr(tok) => tok.name.to_string(),
            Self::CondAttr(tok) => tok.name.to_string(),
        }
    }

    /// return the attribute value in the case the item is
    /// an attribute, or return the conditional attributes
    /// in case the enum is `CondAttributeToken`.
    pub fn attribute(&self) -> AttributeToken {
        match self {
            Self::Attr(attr) => attr.to_owned(),
            Self::CondAttr(attr) => attr.value.clone(),
        }
    }

    /// check is current element is an conditional item
    pub fn is_conditional(&self) -> bool {
        match self {
            Self::Attr(_) => false,
            Self::CondAttr(_) => true,
        }
    }
}

/// AST Token to store information about an
/// `impl` block.
///
/// Reference: <https://doc.rust-lang.org/stable/reference/items/implementations.html>
#[derive(Debug)]
pub struct ImplToken {
    pub attributes: HashMap<String, AttrToken>,
    pub generics: Option<GenericParams>,
    /// The name of the impl Block
    pub name: TokenTree,
    /// for the type where the impl block is implemented for
    // FIXME: we should abstract this token in a better way?
    // or just rename it?
    pub for_ty: Option<TyToken>,
    /// Content of the impl block
    ///
    /// It is stored the raw block because
    /// the kparser library expose all the primitive
    /// to parse this kind of token tree, and this
    /// will make a slim version of the library.
    pub raw_block: TokenStream,
    pub functions: Vec<MethodDeclToken>,
}

impl TopLevelAST for ImplToken {
    fn span(&self) -> TokenTree {
        self.name.clone()
    }

    fn is_impl(&self) -> bool {
        true
    }
}

impl TryFrom<&TokenStream> for ImplToken {
    type Error = KParserError;

    fn try_from(value: &TokenStream) -> Result<Self, Self::Error> {
        let mut stream = KTokenStream::new(value);
        parse_impl(&mut stream, &DummyTracer {})
    }
}

impl Default for ImplToken {
    fn default() -> Self {
        panic!()
    }
}

impl Display for ImplToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // FIXME: print the attributes
        write!(f, "impl {} {{ {} }}", self.name, self.raw_block)
    }
}

/// AST token to store information about a
/// `Trait`.
///
/// Reference <https://doc.rust-lang.org/stable/reference/items/traits.html>
#[derive(Debug)]
pub struct TraitToken {
    pub attrs: HashMap<String, AttrToken>,
    pub visibility: Option<TokenTree>,
    pub ident: TokenTree,
    pub generics: Option<GenericParams>,
    pub inn_attrs: Option<AttrToken>,
    pub associated_items: Vec<AssociatedItem>,
    pub raw_block: TokenStream,
    pub functions: Vec<MethodDeclToken>,
}

impl TopLevelAST for TraitToken {
    fn span(&self) -> TokenTree {
        self.ident.clone()
    }

    fn is_trait(&self) -> bool {
        true
    }
}

impl TryFrom<&TokenStream> for TraitToken {
    type Error = KParserError;

    fn try_from(value: &TokenStream) -> Result<Self, Self::Error> {
        let mut stream = KTokenStream::new(value);
        parse_trait(&mut stream, &DummyTracer {})
    }
}

impl Default for TraitToken {
    fn default() -> Self {
        panic!()
    }
}

impl Display for TraitToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}

/// Enum that contans all the Associated Items
/// supported by the parser.
///
/// Reference <https://doc.rust-lang.org/stable/reference/items/associated-items.html>
#[derive(Debug)]
pub enum AssociatedItem {
    AssociatedFn(FnDeclTok),
    AssociatedMethod(MethodDeclToken),
    // FIXME: add the other associatedItems missing
}

/// AST token to store the information about the
/// a function or method declaration
///
/// Reference <https://doc.rust-lang.org/stable/reference/items/functions.html>
#[derive(Debug)]
pub struct MethodDeclToken {
    pub attrs: HashMap<String, AttrToken>,
    pub visibility: Option<TokenTree>,
    // FIXME: use a better way to be able to
    // identify what kind of qualifiers is
    // specified.
    pub qualifier: Option<TokenTree>,
    pub ident: TokenTree,
    pub generics: Option<GenericParams>,
    pub raw_params: TokenStream,
    /// method/function parameters parser
    /// from the `raw_params` in a tuple
    /// of `(identifier, Type Token)`
    /// and the position is identified by
    /// vector index.
    pub params: Vec<(TokenTree, TyToken)>,
    pub return_ty: Option<TyToken>,
    pub raw_body: Option<TokenStream>,
}

impl Default for MethodDeclToken {
    fn default() -> Self {
        unimplemented!()
    }
}

impl Display for MethodDeclToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}

impl TopLevelAST for MethodDeclToken {
    fn span(&self) -> TokenTree {
        self.ident.clone()
    }

    fn is_fn(&self) -> bool {
        true
    }
}

/// from a parser point of view this
/// should not change much because it is
/// missing just a self param
pub type FnDeclTok = MethodDeclToken;

//! Crate that implement all the AST nodes like
//! struct, Fields, FielsTypes
//!
//! Each implementation contains information
//! regarding the position in the source code (span).
use std::{fmt::Display, rc::Rc};

use crate::diagnostic::KDiagnostic;
use crate::proc_macro::TokenTree;

#[derive(Debug)]
pub struct StructToken {
    pub visibility: Option<TokenTree>,
    pub name: TokenTree,
    pub attributes: Vec<FieldToken>,
}

#[derive(Debug)]
pub struct FieldToken {
    pub visibility: Option<TokenTree>,
    pub name: TokenTree,
    pub ty: FieldTyToken,
}

impl Display for FieldToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut vis = String::new();
        if let Some(viss) = &self.visibility {
            vis = viss.to_string()
        }
        writeln!(f, "{} {}: {}", vis, self.name, self.ty)
    }
}

#[derive(Debug)]
pub struct FieldTyToken {
    pub reference: Option<TokenTree>,
    pub mutable: Option<TokenTree>,
    pub lifetime: Option<TokenTree>,
    pub generics: Vec<Rc<FieldTyToken>>,
    pub name: TokenTree,
}

impl Display for FieldTyToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut prefix = String::new();
        if let Some(refer) = &self.reference {
            prefix += refer.to_string().as_str();
        }

        if let Some(lifetime) = &self.lifetime {
            prefix += lifetime.to_string().as_str();
        }

        if let Some(mutable) = &self.mutable {
            prefix += format!(" {mutable}").as_str();
        }

        // FIXME: support generics
        writeln!(f, "{prefix} {}", self.name)
    }
}

impl KDiagnostic for StructToken {}
impl KDiagnostic for FieldToken {}
impl KDiagnostic for FieldTyToken {}

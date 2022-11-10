//! API to parse the rust struct provided as
//! TokenStream.
use crate::diagnostic::{KDiagnInfo, KDiagnostic};
use crate::eassert_eq;
use crate::proc_macro::TokenTree;
use crate::{
    kproc_macros::KTokenStream,
    rust::{
        ast::RustAST,
        ast_nodes::{FieldToken, StructToken},
    },
};

use super::ast_nodes::FieldTyToken;

// parsing a rust data structure inside a AST that will be easy to
/// manipulate and use by a compiler
pub fn parse_struct<'c>(ast: &'c mut KTokenStream) -> RustAST {
    let visibility = if let Some(vs) = parse_visibility_identifier(ast) {
        let res = Some(vs.clone());
        ast.next();
        res
    } else {
        None
    };
    let tok = ast.advance().to_owned();
    eassert_eq!(
        "struct",
        tok.to_string(),
        tok,
        format!("expected struct keyword found {}", tok)
    );
    let name = ast.advance().to_owned();
    let mut group = ast.to_ktoken_stream();
    let attributes = parse_struct_fields(&mut group);

    let stru = StructToken {
        visibility: visibility.to_owned(),
        name,
        attributes,
    };
    RustAST::Struct(stru)
}

pub fn parse_struct_fields(ast: &mut KTokenStream) -> Vec<FieldToken> {
    let mut fields = vec![];
    while !ast.is_end() {
        let field = parse_struct_field(ast);
        //FIXME: LOG me thanks!
        fields.push(field);
    }
    return fields;
}

pub fn parse_struct_field(ast: &mut KTokenStream) -> FieldToken {
    // name filed
    let visibility = if let Some(vs) = parse_visibility_identifier(ast) {
        let res = Some(vs.clone());
        ast.next();
        res
    } else {
        None
    };
    let field_name = ast.advance().to_owned();
    // : separator
    let separator = ast.advance().clone();
    let mut vis = String::new();
    if let Some(viss) = &visibility {
        vis = viss.to_string()
    }
    eassert_eq!(
        ":",
        separator.to_string(),
        separator,
        format!("exoected `:` but found {}", separator)
    );

    let ty = parse_field_ty(ast);

    FieldToken {
        visibility: visibility.to_owned(),
        name: field_name.to_owned(),
        ty,
    }
}

/// parse the field type as an AST element.
///
/// FIXME: support no reference and mutable field for the moment!
/// please feel free to contribute
pub fn parse_field_ty(ast: &mut KTokenStream) -> FieldTyToken {
    eprintln!("parsing field ty");
    let ty_ref = check_and_parse_ref(ast);
    let lifetime = check_and_parse_lifetime(ast);
    let ty_mutability = check_and_parse_mut(ast);
    // FIXME: ignore recursion type, contribution welcome :)
    // Suggestion: Think recursively
    let field_ty = ast.advance().clone();
    eprintln!("Type: {field_ty}");
    assert_eq!(",", ast.advance().to_string().as_str());
    eprintln!("with comma");
    FieldTyToken {
        reference: ty_ref,
        mutable: ty_mutability,
        lifetime: lifetime.to_owned(),
        generics: vec![],
        name: field_ty.to_owned(),
    }
}

pub fn check_and_parse_ref<'c>(ast: &'c mut KTokenStream) -> Option<TokenTree> {
    let token = ast.peek();
    match token.to_string().as_str() {
        "&" => Some(ast.advance().to_owned()),
        _ => None,
    }
}

pub fn check_and_parse_lifetime<'c>(ast: &'c mut KTokenStream) -> Option<TokenTree> {
    let token = ast.peek().to_string();
    match token.as_str() {
        "'" => {
            // FIXME: add advance by steps API
            ast.next();
            Some(ast.advance().to_owned())
        }
        _ => None,
    }
}

pub fn check_and_parse_mut<'c>(ast: &'c mut KTokenStream) -> Option<TokenTree> {
    let token = ast.peek().to_string();
    match token.as_str() {
        "mut" => Some(ast.advance().to_owned()),
        _ => None,
    }
}

/// parse visibility identifier like pub(crate) and return an option
/// value in case it is not defined.
///
/// FIXME: Return a AST type with a default value on private
/// to make the code cleaner.
pub fn parse_visibility_identifier<'c>(ast: &'c mut KTokenStream) -> Option<TokenTree> {
    let visibility = ast.peek();
    if let TokenTree::Ident(val) = visibility {
        if val.to_string().contains("pub") {
            return Some(ast.peek().to_owned());
        }
    }
    None
}

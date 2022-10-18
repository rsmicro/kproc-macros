//! API to parse the rust struct provided as
//! TokenStream.
use crate::rust::ast::RustAST;
use proc_macro::TokenTree;
use std::rc::Rc;

// parsing a rust data structure inside a AST that will be easy to
/// manipulate and use by a compiler
pub fn parse_struct<'c>(ast: &'c mut dyn Iterator<Item = TokenTree>) -> RustAST {
    let visibility = parse_visibility_identifier(ast);
    assert_eq!("struct", advance(ast).to_string());
    let name = advance(ast);

    let attributes = parse_struct_fields(ast);

    let new_name = format!("Gen{}", name);
    RustAST::Struct(visibility, new_name.to_string(), vec![])
}

fn parse_struct_fields<'c>(ast: &'c mut dyn Iterator<Item = TokenTree>) -> Vec<RustAST> {
    let mut fields = vec![];

    while !is_end(ast) {
        let field = parse_struct_field(ast);
        //FIXME: LOG me thanks!
        fields.push(field);
    }

    return fields;
}

fn parse_struct_field<'c>(ast: &'c mut dyn Iterator<Item = TokenTree>) -> RustAST {
    // name filed
    let visibility = parse_visibility_identifier(ast);
    let field_name = advance(ast);
    //FIXME: log me thanks!
    // : separator
    let separator = advance(ast);
    assert_eq!(":", separator.to_string());

    let ty = parse_field_ty(ast);

    RustAST::Field(visibility, field_name.to_string(), Rc::new(ty))
}

/// parse the field type as an AST element.
///
/// FIXME: support no reference and mutable field for the moment!
/// please feel free to contribute
fn parse_field_ty<'c>(ast: &'c mut dyn Iterator<Item = TokenTree>) -> RustAST {
    println!("parsing field");
    let ty_ref = check_and_parse_ref(ast);
    let lifetime = check_and_parse_lifetime(ast);
    let ty_mutability = check_and_parse_mut(ast);
    // FIXME: ignore recursion type, contribution welcome :)
    // Suggestion: Think recursively
    let field_name = advance(ast);
    RustAST::FieldType(
        ty_ref,
        ty_mutability,
        lifetime,
        None,
        field_name.to_string(),
    )
}

fn check_and_parse_ref<'c>(ast: &'c mut dyn Iterator<Item = TokenTree>) -> bool {
    let token = peek(ast).to_string();
    match token.as_str() {
        "&" => {
            let _ = advance(ast);
            true
        }
        _ => false,
    }
}

fn check_and_parse_lifetime<'c>(ast: &'c mut dyn Iterator<Item = TokenTree>) -> Option<String> {
    let token = peek(ast).to_string();
    match token.as_str() {
        "'" => {
            let _ = advance(ast);
            let lifetime_name = advance(ast).to_string();
            Some(lifetime_name)
        }
        _ => None,
    }
}

fn check_and_parse_mut<'c>(ast: &'c mut dyn Iterator<Item = TokenTree>) -> bool {
    let token = peek(ast).to_string();
    match token.as_str() {
        "mut" => {
            let _ = advance(ast);
            true
        }
        _ => false,
    }
}

/// parse visibility identifier like pub(crate) and return an option
/// value in case it is not defined.
///
/// FIXME: Return a AST type with a default value on private
/// to make the code cleaner.
fn parse_visibility_identifier<'c>(ast: &'c mut dyn Iterator<Item = TokenTree>) -> String {
    let visibility = peek(ast);
    match visibility.to_string().as_str() {
        "pub" => advance(ast).to_string(),
        _ => "".to_string(),
    }
}

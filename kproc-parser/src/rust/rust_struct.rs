//! API to parse the rust struct provided as
//! TokenStream.
use crate::{kproc_macros::KTokenStream, rust::ast::RustAST};
use std::rc::Rc;

// parsing a rust data structure inside a AST that will be easy to
/// manipulate and use by a compiler
pub fn parse_struct<'c>(ast: &'c mut KTokenStream) -> RustAST {
    let visibility = if let Some(vs) = parse_visibility_identifier(ast) {
        ast.next();
        Some(vs)
    } else {
        None
    };
    assert_eq!("struct", ast.advance().to_string());
    let name = ast.advance().to_owned();
    eprintln!("{name}");
    let mut group = ast.to_ktoken_stream();
    let attributes = parse_struct_fields(&mut group);

    let new_name = format!("Gen{}", name);
    let stru = RustAST::Struct(visibility, new_name.to_string(), attributes);
    eprintln!("{:?}", stru);
    stru
}

fn parse_struct_fields(ast: &mut KTokenStream) -> Vec<RustAST> {
    let mut fields = vec![];
    while !ast.is_end() {
        let field = parse_struct_field(ast);
        //FIXME: LOG me thanks!
        fields.push(field);
    }
    return fields;
}

fn parse_struct_field(ast: &mut KTokenStream) -> RustAST {
    // name filed
    let visibility = if let Some(vs) = parse_visibility_identifier(ast) {
        ast.next();
        Some(vs)
    } else {
        None
    };
    let field_name = ast.advance().to_string();
    // : separator
    let separator = ast.advance().clone();
    assert_eq!(
        ":",
        separator.to_string(),
        "after: {} {}",
        visibility.unwrap_or("".to_owned()),
        field_name
    );

    let ty = parse_field_ty(ast);

    RustAST::Field(visibility, field_name.to_string(), Rc::new(ty))
}

/// parse the field type as an AST element.
///
/// FIXME: support no reference and mutable field for the moment!
/// please feel free to contribute
fn parse_field_ty<'c>(ast: &'c mut KTokenStream) -> RustAST {
    eprintln!("parsing field ty");
    let ty_ref = check_and_parse_ref(ast);
    let lifetime = check_and_parse_lifetime(ast);
    let ty_mutability = check_and_parse_mut(ast);
    // FIXME: ignore recursion type, contribution welcome :)
    // Suggestion: Think recursively
    let field_ty = ast.advance().to_string();
    eprintln!("Type: {field_ty}");
    assert_eq!(",", ast.advance().to_string().as_str());
    eprintln!("with comma");
    RustAST::FieldType(ty_ref, ty_mutability, lifetime, None, field_ty)
}

fn check_and_parse_ref<'c>(ast: &'c mut KTokenStream) -> bool {
    let token = ast.peek().to_string();
    match token.as_str() {
        "&" => {
            ast.next();
            true
        }
        _ => false,
    }
}

fn check_and_parse_lifetime<'c>(ast: &'c mut KTokenStream) -> Option<String> {
    let token = ast.peek().to_string();
    match token.as_str() {
        "'" => {
            // FIXME: add advance by steps API
            ast.next();
            let lifetime_name = ast.advance().to_string();
            Some(lifetime_name)
        }
        _ => None,
    }
}

fn check_and_parse_mut<'c>(ast: &'c mut KTokenStream) -> bool {
    let token = ast.peek().to_string();
    match token.as_str() {
        "mut" => {
            ast.next();
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
fn parse_visibility_identifier<'c>(ast: &'c mut KTokenStream) -> Option<String> {
    let visibility = ast.peek().to_string();
    if visibility.contains("pub") && !visibility.contains("_") {
        return Some(visibility);
    }
    None
}

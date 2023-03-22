//! Contains all the core function that are common across
//! different modules.
use super::ast_nodes::{GenericParams, LifetimeParam};
use super::ty::parse_ty;
use crate::kparser::KParserTracer;
use crate::kproc_macros::KTokenStream;
use crate::proc_macro::TokenTree;
use crate::rust::ast_nodes::GenericParam;

/// parsing the declaration of the lifetimes and generics for a
/// declaration of a impl block or struct.
pub fn check_and_parse_generics_params(
    ast: &mut KTokenStream,
    trace: &dyn KParserTracer,
) -> Option<GenericParams> {
    trace.log("parsing generics params");
    // compliant to this https://doc.rust-lang.org/stable/reference/items/generics.html
    if ast.match_tok("<") {
        ast.next(); // consume `<``
        let mut generics = vec![];
        while !ast.match_tok(">") {
            trace.log(&format!("iterate over geeneric, stuck on {:?}", ast.peek()));
            if let Some(lifetime) = check_and_parse_lifetime(ast) {
                if ast.match_tok("+") {
                    trace.log("bouds parsing not supported");
                } else {
                    let param = LifetimeParam {
                        lifetime_or_label: lifetime,
                        bounds: None,
                    };
                    generics.push(GenericParam::LifetimeParam(param));
                }
            } else {
                let ty = parse_ty(ast, trace);
                generics.push(GenericParam::TypeParam(ty));
            }
        }
        ast.next(); // consume the `>` toks
        return Some(GenericParams { params: generics });
    }
    None
}

/// helper function that check and parse the reference token `&`, if
/// is not present return `None`.
pub fn check_and_parse_ref<'c>(ast: &'c mut KTokenStream) -> Option<TokenTree> {
    let token = ast.peek();
    match token.to_string().as_str() {
        "&" => Some(ast.advance().to_owned()),
        _ => None,
    }
}

/// helper function that check and parse the lifetime symbol `'`, if
/// is not present return `None`.
pub fn check_and_parse_lifetime<'c>(ast: &'c mut KTokenStream) -> Option<TokenTree> {
    let token = ast.peek().to_string();
    match token.as_str() {
        "'" => {
            ast.next();
            Some(ast.advance().to_owned())
        }
        _ => None,
    }
}

/// helper function that check and parse the `mut` token, if is not
/// present return `None`.
pub fn check_and_parse_mut<'c>(ast: &'c mut KTokenStream) -> Option<TokenTree> {
    let token = ast.peek().to_string();
    match token.as_str() {
        "mut" => Some(ast.advance().to_owned()),
        _ => None,
    }
}

/// helper function that check and parser the `dyn` token, if is not
/// present return `None`.
pub fn check_and_parse_dyn<'c>(ast: &'c mut KTokenStream) -> Option<TokenTree> {
    let token = ast.peek().to_string();
    match token.as_str() {
        "dyn" => Some(ast.advance().to_owned()),
        _ => None,
    }
}

#[macro_export]
macro_rules! parse_visibility {
    ($ast:expr) => {{
        $crate::rust::core::check_and_parse_visibility($ast)
    }};
}

/// parse visibility identifier like `pub(crate)`` and return an option
/// value in case it is not defined.
pub fn check_and_parse_visibility<'c>(toks: &'c mut KTokenStream) -> Option<TokenTree> {
    if check_identifier(toks, "pub", 0) {
        return Some(toks.advance());
    }
    None
}

pub fn check_is_funct_with_visibility(toks: &mut KTokenStream) -> bool {
    if check_identifier(toks, "pub", 0) {
        if check_identifiers(toks, &["async", "const", "unsafe"], 1) {
            return true;
        }
        if check_identifier(toks, "fn", 2) {
            return true;
        }
    }
    return false;
}

pub fn check_identifier(toks: &KTokenStream, ident: &str, step: usize) -> bool {
    let tok = toks.lookup(step);
    if let TokenTree::Ident(val) = tok {
        if val.to_string().contains(ident) {
            return true;
        }
    }
    false
}

pub fn check_identifiers(toks: &KTokenStream, ident: &[&str], step: usize) -> bool {
    let tok = toks.lookup(step);
    if let TokenTree::Ident(val) = tok {
        if ident.contains(&val.to_string().as_str()) {
            return true;
        }
    }
    false
}

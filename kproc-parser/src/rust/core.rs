//! Contains all the core function that are common across
//! different modules.

use crate::kparser::KParserError;
use crate::kparser::{self, KParserTracer};
use crate::kproc_macros::KTokenStream;
use crate::proc_macro::TokenTree;
use crate::rust::ast_nodes::TypeParam;
use crate::{build_error, check, trace};

use super::ast_nodes::{GenericParam, GenericParams, LifetimeParam, TyToken};
use super::ty::parse_ty;

/// parsing the declaration of the lifetimes and generics for a
/// declaration of a impl block or struct.
pub fn check_and_parse_generics_params(
    ast: &mut KTokenStream,
    trace: &dyn KParserTracer,
) -> kparser::Result<Option<GenericParams>> {
    trace.log("parsing generics params");
    // compliant to this https://doc.rust-lang.org/stable/reference/items/generics.html
    if ast.match_tok("<") {
        ast.next(); // consume `<``

        let mut generics = vec![];
        while !ast.match_tok(">") {
            trace.log(&format!("iterate over geeneric, stuck on {:?}", ast.peek()));
            if let Some(lifetime) = check_and_parse_lifetime(ast) {
                // FIXME: this works? or we need a better parsing for the bounds?
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
                if let Some(ty) = parse_ty(ast, trace)? {
                    generics.push(GenericParam::TypeParam(ty));
                }
            }
        }
        ast.next(); // consume the `>` toks
        return Ok(Some(GenericParams { params: generics }));
    }
    Ok(None)
}

pub fn check_and_parse_bounds(
    stream: &mut KTokenStream,
    tracer: &dyn KParserTracer,
) -> kparser::Result<Option<GenericParams>> {
    trace!(tracer, "check and parsing the type bounds");
    // compliant to this https://doc.rust-lang.org/stable/reference/items/generics.html
    if stream.match_tok("<") {
        stream.next(); // consume `<``

        let mut generics = vec![];
        while !stream.match_tok(">") {
            trace!(
                tracer,
                "iterate over tokens, current token is: `{:?}`",
                stream.peek()
            );
            let mut generic: Option<GenericParam> = None;
            while !stream.match_tok(",") && !stream.match_tok(">") {
                match stream.peek().to_string().as_str() {
                    "'" => {
                        let lifetime = check_and_parse_lifetime(stream).ok_or(build_error!(
                            stream.peek().clone(),
                            "this is a bug while parsing a lifetime, please report a bug"
                        ))?;
                        // FIXME: check the type bound
                        generic = Some(GenericParam::LifetimeParam(LifetimeParam {
                            lifetime_or_label: lifetime,
                            bounds: None,
                        }));
                    }
                    "+" => unimplemented!(
                        "unimplemented contatentation of trait bound + {:?}",
                        generics
                    ),
                    _ => {
                        trace!(tracer, "checking type");
                        if let Some(ty) = check_and_parse_ty(stream, tracer)? {
                            trace!(tracer, "is type");
                            generic = Some(GenericParam::TypeParam(ty));
                        } else {
                            trace!(tracer, "is trait bound");
                            check!(":", stream.lookup(1).clone())?;
                            let identifier = stream.advance();
                            check!(":", stream.advance())?;
                            let trait_bound = stream.advance();
                            // FIXME: fix the trait bound here
                            generic = Some(GenericParam::Bounds(
                                crate::rust::ast_nodes::Bounds::Trait(TypeParam {
                                    identifier,
                                    bounds: None,
                                }),
                            ))
                        }
                    }
                }
            }
            generics.push(generic.unwrap());
        }
        stream.next(); // consume the `>` toks
        return Ok(Some(GenericParams { params: generics }));
    }
    Ok(None)
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

/// helper function that check and parse the lifetime symbol `'`, if
/// is not present return `None`.
pub fn check_and_parse_ty(
    stream: &mut KTokenStream,
    tracer: &dyn KParserTracer,
) -> kparser::Result<Option<TyToken>> {
    let token = stream.lookup(1).to_string();
    trace!(tracer, "checking type with token: {}", token.to_string());
    match token.as_str() {
        ":" => Ok(None),
        _ => parse_ty(stream, tracer),
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

pub fn check_and_parse_fn_qualifier(toks: &mut KTokenStream) -> Option<TokenTree> {
    if check_identifiers(toks, &["async", "const", "unsafe"], 0) {
        return Some(toks.advance());
    }
    None
}

pub fn check_and_parse_fn_tok(toks: &mut KTokenStream) -> Option<TokenTree> {
    if check_identifier(toks, "fn", 0) {
        return Some(toks.advance());
    }
    None
}

pub fn check_is_fun_with_visibility(toks: &mut KTokenStream) -> bool {
    if check_identifier(toks, "pub", 0) {
        if check_identifiers(toks, &["async", "const", "unsafe"], 1) {
            return check_identifier(&toks, "fn", 2);
        } else if check_identifier(toks, "fn", 1) {
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

pub fn check_tok(toks: &KTokenStream, ident: &str, step: usize) -> bool {
    let tok = toks.lookup(step);
    tok.to_string().contains(ident)
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

pub fn check_raw_toks(toks: &KTokenStream, ident: &[&str], step: usize) -> bool {
    let tok = toks.lookup(step);
    ident.contains(&tok.to_string().as_str())
}

pub fn check_and_parse_return_type(
    toks: &mut KTokenStream,
    tracer: &dyn KParserTracer,
) -> kparser::Result<Option<TyToken>> {
    if check_tok(toks, "-", 0) {
        toks.next();
        trace!(tracer, "ok parsed the `-`, now the next is {}", toks.peek());
        if check_tok(toks, ">", 0) {
            toks.next();
            trace!(tracer, "found the `>` no the next is {:?}", toks.peek());
            // FIXME: add a method to consube by steps
            let ty = parse_ty(toks, tracer)?;
            return Ok(ty);
        }
    }
    return Ok(None);
}

//! Contains all the core function that are common across
//! different modules.

use crate::kparser::KParserError;
use crate::kparser::{self, KParserTracer};
use crate::kproc_macros::{KTokenStream, MatchTok};
use crate::proc_macro::TokenTree;
use crate::rust::ast_nodes::{self, TypeParam};
use crate::{build_error, check, trace};

use super::ast_nodes::{GenericParam, GenericParams, LifetimeParam, TyToken};
use super::ty::parse_ty;

/// parsing the declaration of the lifetimes and generics for a
/// declaration of a impl block or struct.
pub fn check_and_parse_generics_params(
    ast: &mut KTokenStream,
    tracer: &dyn KParserTracer,
) -> kparser::Result<Option<GenericParams>> {
    trace!(tracer, "parsing generics params");
    // compliant to this https://doc.rust-lang.org/stable/reference/items/generics.html
    if ast.match_tok("<") {
        ast.next(); // consume `<``
        let mut generics = vec![];
        while !ast.match_tok(">") {
            trace!(tracer, "iterate over geeneric, stuck on {:?}", ast.peek());
            if let Some(lifetime) = check_and_parse_lifetime(ast) {
                let param = LifetimeParam {
                    lifetime_or_label: lifetime,
                    bounds: Vec::new(),
                };
                generics.push(GenericParam::LifetimeParam(param));
            } else if let Some(ty) = parse_ty(ast, tracer)? {
                generics.push(GenericParam::TypeParam(ty));
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
    let is_starting_tok = stream.match_tok("<");
    if !is_starting_tok && !stream.is_group() {
        trace!(
            tracer,
            "while checking the parameter we do not find a token group"
        );
        return Ok(None);
    }
    if !is_starting_tok && !stream.peek().clone().to_token_stream().match_tok("<") {
        trace!(tracer, "not a `<...>` token group`");
        return Ok(None);
    }

    let inner_stream = stream;
    if !is_starting_tok {
        trace!(tracer, "in a `<...>` token group, uwrapping it ...");
        *inner_stream = inner_stream.advance().to_token_stream();
    }

    if inner_stream.match_tok("<") {
        trace!(tracer, "check and parsing the type bounds");
        inner_stream.next(); // consume `<``

        let mut generics = vec![];
        while !inner_stream.is_end() && !inner_stream.match_tok(">") {
            trace!(
                tracer,
                "iterate over tokens, current token is: `{:?}`",
                inner_stream.peek()
            );
            let mut generic: Option<GenericParam> = None;
            while !inner_stream.match_tok(",") && !inner_stream.match_tok(">") {
                trace!(tracer, "checking bound");
                match inner_stream.peek().to_string().as_str() {
                    "+" | ":" => {
                        let tok = inner_stream.advance();
                        assert!(
                            ["+", ":"].contains(&tok.to_string().as_str()),
                            "unexpected token {:?}",
                            tok.to_string()
                        );
                        trace!(tracer, "new bound for the current trait");
                        let bound = if let Some(lifetime) = check_and_parse_lifetime(inner_stream) {
                            ast_nodes::Bound::Lifetime(LifetimeParam {
                                lifetime_or_label: lifetime,
                                bounds: Vec::new(),
                            })
                        } else {
                            let trait_bound = inner_stream.advance();
                            ast_nodes::Bound::Trait(TypeParam {
                                identifier: trait_bound,
                                bounds: Vec::new(),
                            })
                        };
                        assert!(
                            generic.is_some(),
                            "concatenation bound `+` on generic used in the wrong way"
                        );
                        trace!(tracer, "bound found `{:?}`", bound);
                        let Some(generic) = generic.as_mut() else {
                            return Err(build_error!(inner_stream.peek().clone(), "concatenation bound `+` on generic used in the wrong way"));
                         };
                        generic.add_bound(bound);
                    }
                    _ => {
                        assert!(
                            generic.is_none(),
                            "declaration bound with `:` used in the wrong way"
                        );
                        trace!(tracer, "parising token {:?}", inner_stream.peek());
                        if let Some(lifetime) = check_and_parse_lifetime(inner_stream) {
                            trace!(tracer, "life bound found {:?}", lifetime);
                            generic = Some(GenericParam::LifetimeParam(LifetimeParam {
                                lifetime_or_label: lifetime,
                                bounds: vec![],
                            }));
                            continue;
                        }
                        let identifier = inner_stream.advance();
                        trace!(tracer, "Trait `{identifier}`");
                        generic = Some(GenericParam::Bounds(ast_nodes::Bound::Trait(TypeParam {
                            identifier,
                            bounds: vec![],
                        })))
                    }
                }
                trace!(tracer, "next token `{:?}`", inner_stream.peek());
            }
            trace!(
                tracer,
                "conclude to parse the generic bound `{:?}`",
                generic
            );
            generics.push(generic.unwrap());
            if inner_stream.match_tok(",") {
                check!(",", inner_stream.advance())?;
            }
        }
        trace!(
            tracer,
            "finish to parse the generics bounds `{:?}`",
            generics
        );
        if !inner_stream.is_end() {
            check!(">", inner_stream.advance())?; // consume the `>` toks
        }
        return Ok(Some(GenericParams { params: generics }));
    }
    Ok(None)
}

/// helper function that check and parse the reference token `&`, if
/// is not present return `None`.
pub fn check_and_parse_ref(ast: &mut KTokenStream) -> Option<TokenTree> {
    let token = ast.peek();
    match token.to_string().as_str() {
        "&" => Some(ast.advance()),
        _ => None,
    }
}

/// helper function that check and parse the lifetime symbol `'`, if
/// is not present return `None`.
pub fn check_and_parse_lifetime(ast: &mut KTokenStream) -> Option<TokenTree> {
    let token = ast.peek().to_string();
    match token.as_str() {
        "'" => {
            ast.next();
            Some(ast.advance())
        }
        _ => None,
    }
}

/// helper function that check and parse the `mut` token, if is not
/// present return `None`.
pub fn check_and_parse_mut(ast: &mut KTokenStream) -> Option<TokenTree> {
    let token = ast.peek().to_string();
    match token.as_str() {
        "mut" => Some(ast.advance()),
        _ => None,
    }
}

/// helper function that check and parser the `dyn` token, if is not
/// present return `None`.
pub fn check_and_parse_dyn(ast: &mut KTokenStream) -> Option<TokenTree> {
    let token = ast.peek().to_string();
    match token.as_str() {
        "dyn" => Some(ast.advance()),
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
pub fn check_and_parse_visibility(toks: &mut KTokenStream) -> Option<TokenTree> {
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
            return check_identifier(toks, "fn", 2);
        } else if check_identifier(toks, "fn", 1) {
            return true;
        }
    }
    false
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
    if toks.is_end() {
        return Ok(None);
    }
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
    Ok(None)
}

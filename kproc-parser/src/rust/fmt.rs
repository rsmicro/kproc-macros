//! formatting module that contains the basic
//! fmt function that convert in a string
//! part of the rust syntax.
use crate::proc_macro::TokenTree;
use crate::rust::errors::SyntaxError;
use std::collections::BTreeMap;

/// format the lifetime in a string
pub(crate) fn fmt_lifetimes(
    lifetimes: &BTreeMap<TokenTree, Vec<TokenTree>>,
) -> Result<String, SyntaxError> {
    let mut code = String::new();
    for lifetime in lifetimes.into_iter() {
        let mut dec = lifetime.0.to_string();
        let bounds = lifetime.1;
        if !bounds.is_empty() {
            dec += ": ";
            for lifetime in bounds {
                dec += format!("{lifetime} +").as_str();
            }
        }
        dec = dec.strip_suffix("+").unwrap().to_owned();
        code += format!("{dec}, ").as_str();
    }
    code = code.strip_suffix(", ").unwrap().to_owned();
    Ok(code)
}

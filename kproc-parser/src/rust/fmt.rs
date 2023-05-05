//! formatting module that contains the basic
//! fmt function that convert in a string
//! part of the rust syntax.
use super::ast_nodes::{GenericParams, TyToken};
use crate::warn;

pub(crate) fn fmt_generics(generics: &GenericParams) -> String {
    if generics.params.is_empty() {
        return String::new();
    }
    let mut buff = "<".to_owned();
    for generic in &generics.params {
        buff += &format!("{generic}");
    }

    buff += ">";
    buff
}

pub fn fmt_ty(ty: &TyToken) -> String {
    let mut prefix = String::new();
    if let Some(refer) = &ty.ref_tok {
        prefix += &refer.to_string();
    }

    if let Some(mut_tok) = &ty.mut_tok {
        prefix += &mut_tok.to_string();
    }

    if let Some(dyn_tok) = &ty.dyn_tok {
        prefix += &dyn_tok.to_string();
    }

    let mut postfix = String::new();

    // FIXME: the lifetime possible here is only one?
    if let Some(lifetime) = &ty.lifetime {
        postfix += &format!("'{lifetime}, ");
    }

    if let Some(generics) = &ty.generics {
        postfix += "<";
        for generic in generics {
            // FIXME: remove the last comma in
            // the last item
            postfix += &format!("{generic}, ");
        }
        postfix += ">";
    } else {
        let ident = ty.identifier.clone();
        warn!(
            ["Vec"].contains(&ident.to_string().as_str()),
            ident, "the token required generics"
        );
    }

    format!("{prefix} {}{postfix}", ty.identifier)
}

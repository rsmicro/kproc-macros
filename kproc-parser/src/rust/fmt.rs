//! formatting module that contains the basic
//! fmt function that convert in a string
//! part of the rust syntax.
use crate::wassert;

use super::ast_nodes::{GenericParams, TyToken};

pub(crate) fn fmt_generics(generics: &GenericParams) -> String {
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
        prefix += refer.to_string().as_str();
    }

    if let Some(dyn_tok) = &ty.dyn_tok {
        prefix += dyn_tok.to_string().as_str();
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
        wassert!(
            !["Vec"].contains(&ty.identifier.to_string().as_str()),
            ty.identifier,
            "the token required regenerics"
        );
    }

    format!("{prefix} {}{postfix}", ty.identifier)
}

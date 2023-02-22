//! formatting module that contains the basic
//! fmt function that convert in a string
//! part of the rust syntax.
use super::ast_nodes::GenericParams;

pub(crate) fn fmt_generics(generics: &GenericParams) -> String {
    let mut buff = "<".to_owned();
    for generic in &generics.params {
        buff += &format!("{generic}");
    }

    buff += ">";
    buff
}

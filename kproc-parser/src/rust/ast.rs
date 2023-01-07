use crate::proc_macro::TokenStream;
use std::fmt::Display;

use super::{
    ast_nodes::{ImplToken, StructToken},
    fmt::fmt_lifetimes,
};

#[derive(Debug)]
pub enum RustAST {
    Struct(StructToken),
    Impl(ImplToken),
}

impl RustAST {
    pub fn to_token_stream(&self) -> TokenStream {
        let ast_str = self.to_string();
        println!("AST to Tokens \n\n{}", ast_str);
        ast_str.parse().unwrap()
    }

    fn fmt_struct(&self, token: &StructToken) -> String {
        let mut source: String = if let Some(vis) = &token.visibility {
            format!("{} struct {} {{\n", vis.to_string(), token.name.to_string())
        } else {
            format!("struct {} {{\n", token.name)
        };
        for attr in &token.fields {
            // FIXME: add code identation
            source += format!("     {},\n", attr).as_str();
        }
        source += "}\n";
        source
    }

    fn fmt_impl(&self, token: &ImplToken) -> String {
        let lifetimes = if let Some(lifetimes) = &token.decl_lifetims {
            let code = fmt_lifetimes(&lifetimes).unwrap();
            Some(code)
        } else {
            None
        };

        let generics = if let Some(_) = &token.decl_generics {
            Some("".to_owned())
        } else {
            None
        };

        let mut code = if lifetimes.is_some() || generics.is_some() {
            let mut dec_generics = "<".to_owned();
            if let Some(lifetimes) = lifetimes {
                dec_generics += format!("{lifetimes}, ").as_str();
            }
            if let Some(gen) = generics {
                dec_generics += format!("{gen}, ").as_str();
            }
            dec_generics = dec_generics.strip_suffix(", ").unwrap().to_owned();
            dec_generics += ">";
            dec_generics.to_owned()
        } else {
            format!("impl {}", token.name)
        };

        if let Some(for_ty) = &token.for_ty {
            code += format!(" for {for_ty}").as_str();
        }
        code += token.impl_block.to_string().as_str();
        code
    }
}

impl Display for RustAST {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let source = match self {
            RustAST::Struct(token) => self.fmt_struct(&token),
            RustAST::Impl(token) => self.fmt_impl(token),
        };
        write!(f, "{}", source)
    }
}

use crate::proc_macro::TokenStream;
use std::fmt::Display;

use super::ast_nodes::StructToken;

#[derive(Debug)]
pub enum RustAST {
    /// Struct field contain 3 type of information
    /// `Struct(Visibility, Name, Attributes)`
    Struct(StructToken),
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
}

impl Display for RustAST {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let source = match self {
            RustAST::Struct(token) => self.fmt_struct(&token),
        };
        write!(f, "{}", source)
    }
}

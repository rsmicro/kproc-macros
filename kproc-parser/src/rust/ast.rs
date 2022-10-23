use proc_macro2::TokenStream;
use std::fmt::Display;
use std::rc::Rc;
use std::vec::Vec;

#[derive(Debug)]
pub enum RustAST {
    /// Struct field contain 3 type of information
    /// `Struct(Visibility, Name, Attributes)`
    Struct(Option<String>, String, Vec<RustAST>),
    /// An attributed contains the following fields
    /// `Field(Visibility, Name. Type)`
    Field(Option<String>, String, Rc<RustAST>),
    /// Field Type
    /// `FieldType(Reference, Mutable, Lifetime, GenType, TypeName)`
    FieldType(bool, bool, Option<String>, Option<Rc<RustAST>>, String),
}

impl RustAST {
    pub fn to_token_stream(&self) -> TokenStream {
        let ast_str = self.to_string();
        println!("AST to Tokens \n\n{}", ast_str);
        ast_str.parse().unwrap()
    }

    fn fmt_struct(&self, vis: &Option<String>, name: &String, attrs: &Vec<RustAST>) -> String {
        let mut source: String = if let Some(vis) = vis {
            format!("{} struct {} {{\n", vis.as_str(), name.as_str())
        } else {
            format!("struct {} {{\n", name.as_str())
        };
        for attr in attrs {
            // FIXME: add code identation
            source += format!("     {},\n", attr).as_str();
        }
        source += "}\n";
        source
    }

    fn fmt_filed(&self, vis: &Option<String>, name: &str, ty: Rc<RustAST>) -> String {
        let mut code = format!("{name}: {}", ty);
        if let Some(vis) = vis {
            let vis = format!("{vis}");
            code = format!("{vis} {code}");
        }
        code
    }

    fn fmt_ty(
        &self,
        is_ref: bool,
        is_mut: bool,
        lifetime: Option<String>,
        specific_ty: Option<Rc<RustAST>>,
        name: &str,
    ) -> String {
        let mut prefix = String::new();
        if is_ref {
            prefix += "&";
        }

        if let Some(lifetime) = lifetime {
            prefix += lifetime.as_str();
        }

        if is_mut {
            prefix += " mut";
        }

        // FIXME: support generics
        format!("{prefix} {name}").trim().to_owned()
    }
}

impl Display for RustAST {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let source = match self {
            RustAST::Struct(visibility, name, atts) => self.fmt_struct(&visibility, &name, &atts),
            RustAST::Field(vis, name, ty) => self.fmt_filed(vis, name, ty.to_owned()),
            RustAST::FieldType(is_ref, is_mut, lifetime, specific_ty, name) => self.fmt_ty(
                is_ref.to_owned(),
                is_mut.to_owned(),
                lifetime.to_owned(),
                specific_ty.to_owned(),
                name.as_str(),
            ),
        };
        write!(f, "{}", source)
    }
}

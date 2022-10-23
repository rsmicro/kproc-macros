use proc_macro::TokenStream;
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
    pub(crate) fn to_token_stream(&self) -> TokenStream {
        let ast_str = self.to_string();
        println!("AST to Tokens \n\n{}", ast_str);
        ast_str.parse().unwrap()
    }

    fn fmt_struct(&self, vis: &Option<String>, name: &String, attrs: &Vec<RustAST>) -> String {
        let mut source: String = if let Some(vis) = vis {
            format!("{} struct {} {{", vis.as_str(), name.as_str())
        } else {
            format!("struct {} {{", name.as_str())
        };
        for attr in attrs {
            source += format!("{},\n", attr).as_str();
        }
        source += "}\n";
        source
    }
}

impl Display for RustAST {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let source = match self {
            RustAST::Struct(visibility, name, atts) => self.fmt_struct(&visibility, &name, &atts),
            RustAST::Field(_, _, _) => "/* TODO  filed  */".to_string(),
            RustAST::FieldType(_, _, _, _, _) => "/* TODO type */".to_string(),
        };
        write!(f, "{}", source)
    }
}

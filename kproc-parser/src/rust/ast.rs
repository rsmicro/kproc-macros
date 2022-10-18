use log::debug;
use proc_macro::TokenStream;
use std::fmt::Display;
use std::ops::Deref;
use std::rc::Rc;
use std::vec::Vec;

pub enum RustAST {
    /// Struct field contain 3 type of information
    /// `Struct(Visibility, Name, Attributes)`
    Struct(String, String, Vec<RustAST>),
    /// An attributed contains the following fields
    /// `Field(Visibility, Name. Type)`
    Field(String, String, Rc<RustAST>),
    /// Field Type
    /// `FieldType(Reference, Mutable, Lifetime, GenType, TypeName)`
    FieldType(bool, bool, Option<String>, Option<Rc<RustAST>>, String),
}

impl RustAST {
    pub(crate) fn to_token_stream(&self) -> TokenStream {
        let ast_str = self.to_string();
        println!("to token: {}", ast_str);
        ast_str.parse().unwrap()
    }

    fn fmt_struct(&self, vis: &String, name: &String, attrs: &Vec<RustAST>) -> String {
        let mut source: String = format!("{} struct {} {{", vis.as_str(), name.as_str());
        for attr in attrs.deref() {
            source += format!("{},\n", attr).as_str();
        }
        source += "}}\n";
        source
    }
}

impl Display for RustAST {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let source = match self {
            RustAST::Struct(visibility, name, atts) => self.fmt_struct(&visibility, &name, &atts),
            RustAST::Field(_, _, _) => todo!(),
            RustAST::FieldType(_, _, _, _, _) => todo!(),
        };
        write!(f, "{}", source)
    }
}

//! Utils code that it is used to generate other code
use kproc_parser::proc_macro::TokenStream;
use kproc_parser::rust::ast::RustAST;
use kproc_parser::rust::ast_nodes::StructToken;

// FIXME: use the filed attribute to generate the get method when the attribute
// is specified!
pub fn generate_impl(ast: &RustAST) -> TokenStream {
    if let RustAST::Struct(StructToken {
        name,
        fields: attributes,
        generics,
        ..
    }) = ast
    {
        let gen = if let Some(str_gen) = generics {
            format!("{}", str_gen)
        } else {
            "".to_owned()
        };
        let name_attr = attributes[0].name.to_string();
        let ty = attributes[0].ty.to_string();
        let code = format!(
            "impl{} {}{} {{ \
                    fn get_{name_attr}(&self) -> {ty} {{ \
                       return self.{name_attr}.clone()\
                    }} \
                       \
                    fn set_{name_attr}(&self, inner: {ty}) {{ }}
                }}",
            gen, name, gen,
        );
        return code.parse().unwrap();
    }
    panic!("unsupported");
}

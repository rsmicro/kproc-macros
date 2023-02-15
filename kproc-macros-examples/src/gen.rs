//! Utils code that it is used to generate other code
use kproc_parser::proc_macro::TokenStream;
use kproc_parser::rust::ast_nodes::StructToken;

// FIXME: use the filed attribute to generate the get method when the attribute
// is specified!
pub fn generate_impl(struct_tok: &StructToken) -> TokenStream {
    let gen = if let Some(str_gen) = &struct_tok.generics {
        format!("{}", str_gen)
    } else {
        "".to_owned()
    };
    let name_attr = &struct_tok.fields[0].identifier;
    let ty = struct_tok.fields[0].ty.to_string();
    let code = format!(
        "impl{} {}{} {{ \
                    fn get_{name_attr}(&self) -> {ty} {{ \
                       return self.{name_attr}.clone()\
                    }} \
                       \
                    fn set_{name_attr}(&self, inner: {ty}) {{ }}
                }}",
        gen, struct_tok.name, gen,
    );
    return code.parse().unwrap();
}

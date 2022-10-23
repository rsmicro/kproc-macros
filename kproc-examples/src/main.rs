use kproc_macros_examples::RustBuilder;

#[derive(RustBuilder)]
pub struct Foo {
    attr: String,
    self_ref: u32,
}

fn main() {}

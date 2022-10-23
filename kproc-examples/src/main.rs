use kproc_parser::RustBuilder;
use std::rc::Rc;

#[derive(RustBuilder)]
pub struct Foo {
    attr: String,
    self_ref: u32,
}

fn main() {}

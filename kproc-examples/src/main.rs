#![allow(dead_code)]
#![allow(unused_variables)]
use std::fmt::Debug;

use kproc_macros_examples::default_impl;
use kproc_macros_examples::derive_fn;
use kproc_macros_examples::derive_impl;
use kproc_macros_examples::EnumParser;
use kproc_macros_examples::RustBuilder;

trait GenTrait {}

#[derive(RustBuilder, Clone)]
// #[build_struct] // FIXME: support the parsing of this too
pub struct Foo {
    #[build]
    attr: String,
    #[allow(dead_code)]
    self_ref: u32,
}

#[derive(RustBuilder)]
pub struct Boo {
    #[allow(dead_code)]
    attr: String,
    #[allow(dead_code)]
    self_ref: u32,
    pub gen: Vec<Foo>,
}

#[derive(RustBuilder)]
pub struct BooLifetime<'a> {
    #[allow(dead_code)]
    attr: String,
    #[allow(dead_code)]
    self_ref: u32,
    #[allow(dead_code)]
    gen: Vec<&'a Foo>,
}

#[derive(RustBuilder)]
pub struct BooLifetimeDyn<'a> {
    #[allow(dead_code)]
    attr: String,
    #[allow(dead_code)]
    self_ref: u32,
    #[allow(dead_code)]
    gen: Vec<&'a dyn GenTrait>,
}

#[derive(RustBuilder)]
pub struct BooComplex {
    pub gen: Vec<Foo>,
    #[allow(dead_code)]
    attr: String,
    #[allow(dead_code)]
    self_ref: u32,
}

/// The macros take into count also the comments.
#[derive(RustBuilder)]
pub struct BooComplexCommit {
    pub gen: Vec<Foo>,
    #[allow(dead_code)]
    attr: String,
    #[allow(dead_code)]
    self_ref: u32,
}

#[derive(EnumParser)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

#[derive(EnumParser)]
enum MessageWithAttr {
    #[cli]
    Quit,
    #[cli = "foo"]
    Move {
        x: i32,
        y: i32,
    },
    #[cli(help = "this is an helper message")]
    Write(String),
    ChangeColor(i32, i32, i32),
}

struct ForImplDerive {}

/// this is a impl doc
#[derive_impl]
impl ForImplDerive {
    fn alibaba(&self) -> Result<(), ()> {
        todo!()
    }

    pub fn pub_fn(&self) -> Result<(), ()> {
        todo!()
    }
}
/// this is a trait docs
#[default_impl]
trait Seq<T> {
    fn len(&self) -> u32;
    fn elt_at(&self, n: u32) -> T;
    fn iter<F>(&self, f: F);
}

/// this is just a foo function
#[derive_fn]
fn foo(string: &str) -> Result<Vec<String>, ()> {
    todo!()
}

#[derive_fn]
async fn foo_async(string: &str) -> Result<Vec<String>, ()> {
    todo!()
}

#[derive_fn]
fn foo_mut(string: &mut str) -> Result<Vec<String>, ()> {
    todo!()
}

/// this is just a foo function
#[derive_fn]
fn foo_with_bound<T: Debug>(value: T) -> Result<Vec<String>, ()> {
    todo!()
}

/// this is just a foo function
#[derive_fn]
fn foo_with_bound_with_multiuple_type<T: Debug + ToString>(value: T) -> Result<Vec<String>, ()> {
    todo!()
}

/// this is just a foo function
#[derive_fn]
fn foo_with_bound_with_multiuple_type_double<T: Debug + ToString, R>(
    value: T,
) -> Result<Vec<R>, ()> {
    todo!()
}

#[derive_fn]
fn help<C: Debug, F: Debug>(top_level: Option<C>, sucommands: Vec<C>, flags: Vec<F>) {
    unimplemented!()
}

#[derive_fn]
fn help_stress<C: Debug, F: Debug>(top_level: Option<C>, sucommands: &Vec<C>, flags: &[F]) {
    unimplemented!()
}

fn main() {
    let obj = Foo {
        attr: "Alibaba".to_string(),
        self_ref: 0,
    };
    assert_eq!(obj.get_attr(), obj.attr);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let obj = crate::Foo {
            attr: "Alibaba".to_string(),
            self_ref: 0,
        };
        assert_eq!(obj.get_attr(), obj.attr);
    }

    #[test]
    fn generics_works() {
        let obj = crate::Boo {
            attr: "Alibaba".to_string(),
            self_ref: 0,
            gen: vec![],
        };
        assert!(obj.gen.is_empty());
    }

    #[test]
    fn generics_with_dyn_works() {
        let obj = crate::BooLifetimeDyn {
            attr: "Alibaba".to_string(),
            self_ref: 0,
            gen: vec![],
        };
        assert!(obj.gen.is_empty());
    }
}

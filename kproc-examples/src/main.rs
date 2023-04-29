use kproc_macros_examples::default_impl;
use kproc_macros_examples::derive_fn;
use kproc_macros_examples::derive_impl;
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

struct ForImplDerive {}

#[derive_impl]
impl ForImplDerive {
    fn alibaba(&self) -> Result<(), ()> {
        todo!()
    }

    pub fn pub_fn(&self) -> Result<(), ()> {
        todo!()
    }
}

#[default_impl]
trait Seq<T> {
    fn len(&self) -> u32;
    fn elt_at(&self, n: u32) -> T;
    fn iter<F>(&self, f: F);
}

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

    /*
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
    */
}

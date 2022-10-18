//! Kernel procedural macros
use proc_macro::{TokenStream, TokenTree};
use std::rc::Rc;

pub struct KTokenStream<'a> {
    pos: usize,
    tokens: &'a mut dyn Iterator<Item = TokenTree>,
}

impl KTokenStream<'_> {
    fn new(tokens: &mut TokenStream) -> Self {
        KTokenStream {
            pos: 0,
            tokens: &mut tokens.into_iter(),
        }
    }
}

// FIXME:fix, peek advance the stream :/
impl KTokenStream<'_> {
    pub fn advance<'c>(&mut self, ast: &'c mut dyn Iterator<Item = TokenTree>) -> Rc<TokenTree> {
        Rc::new(ast.next().unwrap())
    }

    pub fn peek<'c>(&mut self, ast: &'c mut dyn Iterator<Item = TokenTree>) -> Rc<TokenTree> {
        Rc::new(ast.peekable().peek().unwrap().to_owned())
    }

    pub fn is_end<'c>(&mut self, ast: &'c mut dyn Iterator<Item = TokenTree>) -> bool {
        ast.peekable().peek().is_none()
    }
}

//! Kernel procedural macros
use proc_macro2::{TokenStream, TokenTree};

#[derive(Debug)]
pub struct KTokenStream {
    pos: usize,
    kstream: Vec<TokenTree>,
    size: usize,
}

impl KTokenStream {
    pub fn new(tokens: &TokenStream) -> Self {
        KTokenStream::new_with_pos(tokens, 0)
    }

    pub fn new_with_pos(tokens: &TokenStream, pos: usize) -> Self {
        let mut kstream = Vec::new();
        tokens
            .to_owned()
            .into_iter()
            .for_each(|token| kstream.push(token));
        KTokenStream {
            pos,
            kstream: kstream.to_vec(),
            size: kstream.len(),
        }
    }

    /// advance the position and return the previous element
    /// in position - 1
    pub fn advance<'c>(&'c mut self) -> &'c TokenTree {
        self.next();
        self.prev()
    }

    pub fn next(&mut self) {
        self.pos += 1;
    }

    pub fn prev<'c>(&'c self) -> &'c TokenTree {
        &self.kstream[self.pos - 1]
    }

    /// return he token at the current position
    pub fn peek<'c>(&'c self) -> &'c TokenTree {
        &self.kstream[self.pos]
    }

    /// check if it is reach the end of the stream
    pub fn is_end(&self) -> bool {
        self.pos > self.size - 1
    }

    pub fn to_ktoken_stream(&self) -> KTokenStream {
        match self.peek() {
            TokenTree::Group(stream) => KTokenStream::new(&stream.stream()),
            _ => panic!("no stream on token {:?}", self.peek()),
        }
    }

    pub fn consume_brace(&mut self) {
        let tok = self.peek();
        match tok.to_string().as_str() {
            "{" | "}" => {
                self.advance();
            }
            _ => {}
        }
    }
}

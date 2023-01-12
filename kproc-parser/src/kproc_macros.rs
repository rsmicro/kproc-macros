//! core module that implement the basic concept
//! used inside the parser.
use crate::proc_macro::{TokenStream, TokenTree};

/// Convinient way to manage any kind of tokens stream
/// by exposing basic function to advance and consume
/// the stream.
///
/// Heavenly inspired to `albert_stream` <https://github.com/vincenzopalazzo/albert/tree/main/stream>
#[derive(Debug)]
pub struct KTokenStream {
    pos: usize,
    kstream: Vec<TokenTree>,
    size: usize,
}

impl KTokenStream {
    /// create a new instance from a TokenStream
    pub fn new(tokens: &TokenStream) -> Self {
        KTokenStream::new_with_pos(tokens, 0)
    }

    /// create the a new instance from a TokenStream and the
    /// initial position
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
    pub fn advance(&mut self) -> &TokenTree {
        self.next();
        if self.is_end() {
            return &self.kstream.last().unwrap();
        }
        self.prev()
    }

    /// perform a search operation inside the stream by a number
    /// of specified steps.
    pub fn lookup<'c>(&'c self, step: usize) -> &'c TokenTree {
        assert!(self.size > self.pos + step);
        &self.kstream[self.pos + step]
    }

    /// advance the position of the stream.
    pub fn next(&mut self) {
        self.pos += 1;
    }

    /// take the previous element of the stream from the
    /// current position.
    pub fn prev(&self) -> &TokenTree {
        assert!(self.pos < self.size, "prev: out of bound");
        &self.kstream[self.pos - 1]
    }

    /// return he token at the current position.
    pub fn peek(&self) -> &TokenTree {
        assert!(self.pos < self.size);
        &self.kstream[self.pos]
    }

    /// return the last token of the stream.
    pub fn last(&self) -> &TokenTree {
        &self.kstream.last().unwrap()
    }

    /// match the current token with the one specified.
    pub fn match_tok(&self, tok: &str) -> bool {
        self.peek().match_tok(tok)
    }

    /// check if it is reach the end of the stream
    pub fn is_end(&self) -> bool {
        self.pos > self.size - 1
    }

    /// unwrap the `TokenTree::Group` and the return the
    /// token stream that contains. If the current token
    /// it is not a `TokenTree::Group` the function will panic
    pub fn to_ktoken_stream(&self) -> KTokenStream {
        match self.peek() {
            TokenTree::Group(stream) => KTokenStream::new(&stream.stream()),
            _ => panic!("no stream on token {:?}", self.peek()),
        }
    }

    /// unwrap the `TokenTree::Group` and the return the
    /// token stream that contains. If the current token
    /// it is not a `TokenTree::Group` the function will panic
    pub fn opt_ktoken_stream(&self) -> Option<KTokenStream> {
        match self.peek() {
            TokenTree::Group(stream) => Some(KTokenStream::new(&stream.stream())),
            _ => None,
        }
    }

    /// check if the current token is a `TokenTree::Group`
    pub fn is_group(&self) -> bool {
        match self.peek() {
            TokenTree::Group(_) => true,
            _ => false,
        }
    }

    // FIXME: this can be removed?
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

pub trait MatchTok
where
    Self: ToString,
{
    fn match_tok(&self, tok: &str) -> bool {
        self.to_string().as_str() == tok
    }
}

impl MatchTok for TokenTree {}

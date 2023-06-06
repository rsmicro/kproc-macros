//! core module that implement the basic concept
//! used inside the parser.
use std::fmt::Debug;

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

impl From<TokenStream> for KTokenStream {
    fn from(value: TokenStream) -> Self {
        KTokenStream::new(&value)
    }
}

impl From<&TokenStream> for KTokenStream {
    fn from(value: &TokenStream) -> Self {
        KTokenStream::new(value)
    }
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
    pub fn advance(&mut self) -> TokenTree {
        self.next();
        if self.is_end() {
            return self.kstream.last().unwrap().to_owned();
        }
        self.prev().to_owned()
    }

    /// perform a search operation inside the stream by a number
    /// of specified steps.
    pub fn lookup<'c>(&'c self, step: usize) -> &'c TokenTree {
        assert!(self.size > self.pos + step);
        &self.kstream[self.pos + step]
    }

    /// perform a search operation inside the stream by a number
    /// of specified steps.
    pub fn has<'c>(&'c self, step: usize) -> bool {
        self.size > self.pos + step
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
        assert!(
            self.pos < self.size,
            "peek: out of bound requested {} vs tot size {}",
            self.pos,
            self.size
        );
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
        self.pos >= self.size
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

    pub fn unwrap_group(&self) -> TokenTree {
        match self.peek() {
            TokenTree::Group(_) => self.peek().clone(),
            _ => panic!("the token {:?} is not a `TokenTree::Group`", self.peek()),
        }
    }

    pub fn unwrap_group_as_stream(&self) -> TokenStream {
        match self.peek() {
            TokenTree::Group(stream) => stream.stream(),
            _ => panic!("the token {:?} is not a `TokenTree::Group`", self.peek()),
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
    Self: ToString + Debug,
{
    fn match_tok(&self, tok: &str) -> bool {
        self.to_string().as_str() == tok
    }

    fn to_token_stream(&self) -> KTokenStream;
}

impl MatchTok for TokenTree {
    fn to_token_stream(&self) -> KTokenStream {
        match self {
            TokenTree::Group(stream) => KTokenStream::new(&stream.stream()),
            _ => panic!("no stream on token {:?}", self),
        }
    }
}

#[derive(Clone, Debug)]
pub struct OrderedTokenTree {
    idx: u64,
    token: TokenTree,
}

impl OrderedTokenTree {
    pub fn new(idx: u64, token: TokenTree) -> Self {
        OrderedTokenTree { idx, token }
    }

    pub fn token(&self) -> TokenTree {
        self.token.clone()
    }

    pub fn idx(&mut self, idx: u64) {
        self.idx = idx;
    }
}

impl From<TokenTree> for OrderedTokenTree {
    fn from(value: TokenTree) -> Self {
        OrderedTokenTree::new(0, value)
    }
}

impl PartialOrd for OrderedTokenTree {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.idx.partial_cmp(&other.idx)
    }
}

impl Ord for OrderedTokenTree {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.idx.cmp(&other.idx)
    }
}

impl PartialEq for OrderedTokenTree {
    fn eq(&self, other: &Self) -> bool {
        self.idx.eq(&other.idx)
    }
}

impl Eq for OrderedTokenTree {}

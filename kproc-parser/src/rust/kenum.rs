use std::fmt::write;

// WIP: parsing enum code
use crate::kparser::{KParserError, KParserTracer};
use crate::kproc_macros::KTokenStream;
use crate::proc_macro::{TokenStream, TokenTree};
use crate::{check, kparser, parse_visibility, trace};

#[derive(Debug)]
pub struct EnumToken {
    pub visibility: Option<TokenTree>,
    pub identifier: TokenTree,
    pub raw_body: TokenStream,
}

impl std::fmt::Display for EnumToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(ref vis) = self.visibility {
            write!(f, "{vis}")?;
        }
        let identifier = &self.identifier;
        writeln!(f, " enum {identifier} {{")?;
        writeln!(f, "{}", self.raw_body)?;
        writeln!(f, "}}")
    }
}

impl Default for EnumToken {
    fn default() -> Self {
        unimplemented!()
    }
}

pub fn parse(stream: &mut KTokenStream, tracer: &dyn KParserTracer) -> kparser::Result<EnumToken> {
    trace!(tracer, "{:?}", stream);
    let visibility = parse_visibility!(stream);
    check!("enum", stream.advance())?;
    let identifier = stream.advance();
    let raw_body = stream.unwrap_group_as_stream();
    // FIXME: parse the body!
    Ok(EnumToken {
        visibility,
        identifier,
        raw_body,
    })
}

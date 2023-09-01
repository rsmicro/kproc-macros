// WIP: parsing enum code
use std::collections::HashMap;
use std::vec::Vec;

use crate::kparser::{KParserError, KParserTracer};
use crate::kproc_macros::KTokenStream;
use crate::proc_macro::{Delimiter, TokenStream, TokenTree};
use crate::{build_error, check, kparser, parse_attributes, parse_visibility, trace};

use super::ast_nodes::AttributeV2Token;
use super::kattr::prelude::*;

#[derive(Debug)]
pub struct EnumToken {
    pub attributes: HashMap<String, AttributeV2Token>,
    pub visibility: Option<TokenTree>,
    pub identifier: TokenTree,
    pub raw_body: TokenStream,
    pub values: Vec<EnumValue>,
}

#[derive(Debug)]
pub struct EnumValue {
    pub attributes: HashMap<String, AttributeV2Token>,
    pub kind: EnumValueKind,
    pub identifier: TokenTree,
}

#[derive(Debug)]
pub enum EnumValueKind {
    Named(HashMap<String, TokenTree>),
    Anonymus(Vec<TokenTree>),
    Simple,
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
    let attributes = parse_attributes!(stream, tracer)?;
    let visibility = parse_visibility!(stream);
    check!("enum", stream.advance())?;
    let identifier = stream.advance();
    let raw_body = stream.unwrap_group_as_stream();
    let mut body_stream = KTokenStream::new(&raw_body);
    let values = parse_body(&mut body_stream, tracer)?;
    Ok(EnumToken {
        attributes,
        visibility,
        identifier,
        raw_body,
        values,
    })
}

fn parse_body<T: KParserTracer + ?Sized>(
    stream: &mut KTokenStream,
    tracer: &T,
) -> kparser::Result<Vec<EnumValue>> {
    let mut values = Vec::new();
    while !stream.is_end() {
        let attributes = parse_attributes!(stream, tracer)?;
        let identifier = stream.advance();
        trace!(tracer, "identifier {:?}", identifier);
        let content = stream.peek();
        let kind = match stream.peek() {
            TokenTree::Group(ref group) => match group.delimiter() {
                Delimiter::Brace => {
                    let group = stream.unwrap_group_as_stream();
                    stream.next();
                    parse_named_value(group, tracer)?
                }
                Delimiter::Parenthesis => {
                    let group = stream.unwrap_group_as_stream();
                    stream.next();
                    parse_anonymus_value(group, tracer)?
                }
                _ => return Err(build_error!(content.clone(), "invalid token inside enum")),
            },
            TokenTree::Punct(_) => EnumValueKind::Simple,
            _ => {
                return Err(build_error!(
                    content.clone(),
                    "token `{}` not expected",
                    content
                ))
            }
        };
        if !stream.is_end() {
            check!(",", stream.advance())?;
        }
        trace!(tracer, "Enum kind found {:?}", kind);
        values.push(EnumValue {
            attributes,
            kind,
            identifier,
        });
    }
    Ok(values)
}

fn parse_named_value<T: KParserTracer + ?Sized>(
    stream: TokenStream,
    _: &T,
) -> kparser::Result<EnumValueKind> {
    let mut stream = KTokenStream::new(&stream);
    let mut fields = HashMap::new();
    while !stream.is_end() {
        let identifier = stream.advance();
        check!(":", stream.advance())?;
        let ttype = stream.advance();
        fields.insert(identifier.to_string(), ttype);
        if !stream.is_end() {
            check!(",", stream.advance())?;
        }
    }
    Ok(EnumValueKind::Named(fields))
}

fn parse_anonymus_value<T: KParserTracer + ?Sized>(
    stream: TokenStream,
    tracer: &T,
) -> kparser::Result<EnumValueKind> {
    let mut stream = KTokenStream::new(&stream);
    let mut fields = Vec::new();
    while !stream.is_end() {
        let ttype = stream.advance();
        trace!(tracer, "ttype {:?}", ttype);
        fields.push(ttype);
        if !stream.is_end() {
            check!(",", stream.advance())?;
        }
    }
    Ok(EnumValueKind::Anonymus(fields))
}

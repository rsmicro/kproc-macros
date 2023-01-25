//! KParser tracer API
use crate::kproc_macros::KTokenStream;

/// Trace Trait to inject inside the parser to keep track
/// what the parser is doing.
pub trait KParserTracer {
    fn log(&self, msg: &str);
}

/// A dummy tracer, no always we want
/// trace the parser (maybe).
pub struct DummyTracer;

impl KParserTracer for DummyTracer {
    fn log(&self, _: &str) {}
}

/// Parser trait that allow to each type to parse
/// itself.
pub trait KParser<T> {
    fn parse(&self, stream: &mut KTokenStream, tracer: &dyn KParserTracer) -> T;

    fn opt_parse(&self, stream: &mut KTokenStream, tracer: &dyn KParserTracer) -> Option<T> {
        let stm = self.parse(stream, tracer);
        Some(stm)
    }
}

//! KParser API

/// Trace the parser to keep track
/// what the parser is doing.
pub trait KParserTracer {
    fn log(&self, msg: &str);
}

/// A dummy tracer, no always we want
/// trace the parser.
pub struct DummyTracer;

impl KParserTracer for DummyTracer {
    fn log(&self, _: &str) {}
}

//! KParser tracer API
use crate::kdiagnostic::KDiagnInfo;
use crate::kproc_macros::KTokenStream;
use crate::proc_macro::TokenTree;

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

/// Generic error where with an specific
/// token Tree and and error message that
/// it is used to generate the diagnostic
/// later.
#[derive(Debug)]
pub struct KParserError {
    dig: KDiagnInfo,
}

impl KParserError {
    pub fn new(dig: KDiagnInfo) -> Self {
        KParserError { dig }
    }

    pub fn with_msg(tok: TokenTree, msg: &str, line: String, file: String) -> Self {
        let diag = KDiagnInfo::new(msg, tok, line, file);
        Self::new(diag)
    }

    pub fn expect(
        expect_tok: &str,
        tok: &TokenTree,
        line: String,
        file: String,
    ) -> Result<(), KParserError> {
        if expect_tok != &tok.to_string() {
            let msg = format!("expected `{expect_tok}` but got `{tok}`");
            return Err(KParserError {
                dig: KDiagnInfo::new(&msg, tok.to_owned(), line, file),
            });
        }
        Ok(())
    }

    pub fn emit(self) {
        self.dig.emit()
    }

    pub fn span(&self) -> TokenTree {
        self.dig.span()
    }
}

/// KParser generic parser that it is used to
/// parse any kind of token stream.
pub trait KParser {
    /// try to parse the token stream inside the type E, and if
    /// there is no option for kparser, return an error.
    fn parse<E>(
        &self,
        stream: &mut KTokenStream,
        tracer: &dyn KParserTracer,
    ) -> Result<E, KParserError>;
}

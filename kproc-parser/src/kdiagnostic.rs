use crate::{diagnostic::Diagnostic, proc_macro::TokenTree};

#[derive(Debug, Clone)]
pub struct KDiagnInfo {
    inner: Diagnostic,
    tok: TokenTree,
}

impl KDiagnInfo {
    pub fn new(msg: &str, tok: TokenTree, line: String, file: String) -> Self {
        KDiagnInfo {
            inner: Diagnostic::new(msg.to_owned(), tok.span(), line, file),
            tok,
        }
    }

    pub fn span(&self) -> TokenTree {
        self.tok.clone()
    }

    pub fn warn(mut self) -> Self {
        self.inner.is_warn();
        self.clone()
    }

    pub fn emit(self) {
        self.inner.emit()
    }
}

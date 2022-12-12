use crate::proc_macro::{Diagnostic, TokenTree};

pub struct KDiagnInfo {
    msg: Option<String>,
    help: Option<String>,
}

impl KDiagnInfo {
    #[allow(dead_code)]
    pub fn new(msg: &str, help: &str) -> Self {
        KDiagnInfo {
            msg: Some(msg.to_owned()),
            help: Some(help.to_owned()),
        }
    }

    pub fn with_msg(msg: &str) -> Self {
        KDiagnInfo {
            msg: Some(msg.to_owned()),
            help: None,
        }
    }

    pub fn with_help(&mut self, msg: &str) {
        self.help = Some(msg.to_owned())
    }
}

pub trait KDiagnostic {
    fn emit_error_on_token(&self, token: &TokenTree, msg: &str) -> Diagnostic {
        token.span().error(msg)
    }

    fn emit_warning_on_token(&self, token: &TokenTree, msg: &str) -> Diagnostic {
        token.span().warning(msg)
    }

    fn emit_error(&self, token: &TokenTree, data: &KDiagnInfo) {
        if let Some(msg) = &data.msg {
            let mut diag = self.emit_error_on_token(token, msg.as_str());
            if let Some(help) = &data.help {
                diag = diag.help(help);
            }
            diag.emit()
        }
    }

    fn emit_warn(&self, token: &TokenTree, data: &KDiagnInfo) {
        if let Some(msg) = &data.msg {
            let mut diag = self.emit_warning_on_token(token, msg.as_str());
            if let Some(help) = &data.help {
                diag = diag.help(help);
            }
            diag.emit()
        }
    }
}

impl KDiagnostic for TokenTree {}

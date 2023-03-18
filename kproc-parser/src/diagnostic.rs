use crate::proc_macro::TokenTree;

#[derive(Debug, Clone)]
pub struct KDiagnInfo {
    tok: TokenTree,
    msg: String,
    help: Option<String>,
    war: bool,
}

impl KDiagnInfo {
    pub fn new(msg: &str, tok: TokenTree) -> Self {
        KDiagnInfo {
            msg: msg.to_owned(),
            help: None,
            tok,
            war: false,
        }
    }

    pub fn help(mut self, msg: &str) -> Self {
        self.help = Some(msg.to_owned());
        self
    }

    pub fn warn(mut self) -> Self {
        self.war = true;
        self
    }

    pub fn emit(self) {
        if self.war {
            self.clone().emit_warn();
            return;
        }
        self.emit_error()
    }

    pub fn emit_error(self) {
        let mut diag = self.tok.span().error(self.msg);
        if let Some(help) = self.help {
            diag = diag.help(help);
        }
        diag.emit()
    }

    pub fn emit_warn(self) {
        let mut diag = self.tok.span().warning(self.msg);
        if let Some(help) = self.help {
            diag = diag.help(help);
        }
        diag.emit()
    }
}

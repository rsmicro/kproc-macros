//! kproc-macros diagnostic implementation
//!
//!
//! This crate implement the Diagnostic and allow to use also
//! some unstable feature of the compiler, such such as the
//! `feature(proc_macro_diagnostic)`.
//!
//! However to allow the compilation on nightly we use the
//! way to print a crafted message and then panic the compiler.
//!
//! An example of message in stable rust is
//! ```
//! error: `kproc-parser/src/diagnostic.rs:50` the token `error` has the following Boo
//! error: something happens
//! error: proc-macro derive panicked
//!  --> kproc-examples/src/main.rs:17:10
//!   |
//! 1 | #[derive(RustBuilder)]
//!   |          ^^^^^^^^^^^
//!   |
//!   = help: message: `kproc-parser/src/diagnostic.rs:58` an error during the compilation happens on token `Boo`

//! ```
use crate::proc_macro::Span;

#[derive(Debug, Clone)]
pub(crate) enum Level {
    Error,
    Warn,
}

#[derive(Debug, Clone)]
pub(crate) struct Diagnostic {
    msg: String,
    span: Span,
    level: Level,
    line: String,
    file: String,
}

impl std::fmt::Display for Level {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Level::Warn => write!(f, "warning"),
            Level::Error => write!(f, "error"),
        }
    }
}

impl Diagnostic {
    pub fn new(msg: String, span: Span, line: String, file: String) -> Self {
        Diagnostic {
            msg,
            span,
            level: Level::Error,
            line,
            file,
        }
    }

    #[cfg(not(feature = "builtin_diagnostic"))]
    fn get_colored_string(&self) -> String {
        match self.level {
            Level::Error => String::from("\x1b[1;31merror\x1b[1;97m"),
            Level::Warn => String::from("\x1b[1;33mwarning\x1b[1;97m"),
        }
    }

    #[cfg(not(feature = "builtin_diagnostic"))]
    pub fn emit(self) {
        let level = self.get_colored_string();
        eprintln!(
            "{level}: `{}:{}` the token `{}` has the following {} \n{level}\x1b[1;97m: \x1b[3;34m{}",
            self.file,
            self.line,
            self.level,
            self.span.source_text().unwrap_or_default(),
            self.msg
        );
        panic!(
            "\x1b[1;97m`{}:{}` an {} during the compilation happens on token `{}`",
            self.file,
            self.line,
            self.level,
            self.span.source_text().unwrap_or_default()
        );
    }

    #[cfg(feature = "builtin_diagnostic")]
    pub fn emit(self) {
        match self.level {
            Level::Error => self.emit_error(),
            Level::Warn => self.emit_warn(),
        }
    }

    #[cfg(feature = "builtin_diagnostic")]
    pub fn emit_error(self) {
        let diag = self.span.error(self.msg);
        diag.emit()
    }

    #[cfg(feature = "builtin_diagnostic")]
    pub fn emit_warn(self) {
        let diag = self.span.warning(self.msg);
        diag.emit()
    }

    pub fn is_warn(&mut self) {
        self.level = Level::Warn;
    }
}

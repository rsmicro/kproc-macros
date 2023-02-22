#[macro_export]
macro_rules! wassert {
    ($val:expr, $token:expr, $msg:expr) => {
        if !$val {
            use crate::diagnostic::KDiagnInfo;
            let mut data = KDiagnInfo::with_msg($msg.to_string().as_str());
            data.with_help("This is an assert failure consider to submit a bug report");
            $token.emit_warn(&$token, &data);
        }
    };
}

#[macro_export]
macro_rules! wassert_eq {
    ($a:expr, $b:expr, $token:expr, $msg:expr) => {
        if $a != $b {
            use crate::diagnostic::KDiagnInfo;
            let mut data = KDiagnInfo::with_msg($msg.to_string().as_str());

            data.with_help("This is an assert failure consider to submit a bug report");
            $token.emit_warn(&$token, &data);
        }
    };
}

#[macro_export]
macro_rules! eassert {
    ($val:expr, $token:item, $msg:literal) => {
        if !$val {
            use crate::diagnostic::KDiagnInfo;
            let mut data = KDiagnInfo::with_msg($msg.to_string().as_str());
            data.with_help("This is an assert failure consider to submit a bug report");
            $token.emit_error(&$token, &data);
        }
    };
}

#[macro_export]
macro_rules! eassert_eq {
    ($a:expr, $b:expr, $token:expr, $msg:expr) => {
        if $a != $b {
            use crate::diagnostic::KDiagnInfo;
            let mut data = KDiagnInfo::with_msg($msg.to_string().as_str());
            data.with_help("This is an assert failure consider to submit a bug report");
            $token.emit_error(&$token, &data);
        }
    };
}

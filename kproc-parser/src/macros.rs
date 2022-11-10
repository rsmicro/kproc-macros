#[macro_export]
macro_rules! wassert {
    ($val:expr, $token:item, $msg:literal) => {
        if !val {
            let data = KDiagnInfo::with_msg($msg.to_string().as_str());
            $token.emit_warn(&$token, &data);
        }
    };
}

#[macro_export]
macro_rules! wassert_eq {
    ($a:expr, $b:expr, $token:expr, $msg:expr) => {
        if $a != $b {
            let data = KDiagnInfo::with_msg($msg.to_string().as_str());
            $token.emit_warn(&$token, &data);
        }
    };
}

#[macro_export]
macro_rules! eassert {
    ($val:expr, $token:item, $msg:literal) => {
        if !val {
            let data = KDiagnInfo::with_msg($msg.to_string().as_str());
            $token.emit_error(&$token, &data);
        }
    };
}

#[macro_export]
macro_rules! eassert_eq {
    ($a:expr, $b:expr, $token:expr, $msg:expr) => {
        if $a != $b {
            let data = KDiagnInfo::with_msg($msg.to_string().as_str());
            $token.emit_error(&$token, &data);
        }
    };
}

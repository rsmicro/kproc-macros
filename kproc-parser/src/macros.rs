#[macro_export]
macro_rules! check {
    ($a:literal, $b:expr) => {
        KParserError::expect($a, &$b, line!().to_string(), file!().to_string())
    };
}

#[macro_export]
/// emit a warning when the expression
/// is verified
macro_rules! warn {
    ($when: expr, $tok: expr, $($msg:tt)*) => {
        if $when {
            use $crate::kdiagnostic::KDiagnInfo;
            let msg = format!($($msg)*);
            KDiagnInfo::new(&msg, $tok, line!().to_string(), file!().to_string()).warn().emit()
        }
    };
}

#[macro_export]
/// emit a compiler error
macro_rules! error {
    ($tok: expr, $($msg:tt)*) => {{
        use $crate::kdiagnostic::KDiagnInfo;
        let msg = format!($($msg)*);
        KDiagnInfo::new(&msg, $tok, line!().to_string(), file!().to_string()).emit()
    }};
}

#[macro_export]
/// emit a compiler error
macro_rules! build_error {
    ($tok: expr, $($msg:tt)*) => {{
        let msg = format!($($msg)*);
        KParserError::with_msg($tok, &msg, line!().to_string(), file!().to_string())
    }};
}

#[macro_export]
macro_rules! trace {
    ($trace:expr, $($msg:tt)*) => {
        $trace.log(&format!($($msg)*))
    };
}

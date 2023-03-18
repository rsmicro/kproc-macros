#[macro_export]
macro_rules! check {
    ($a:literal, $b:expr) => {
        KParserError::expect($a, &$b)
    };
}

#[macro_export]
/// emit a warning when the expression
/// is verified
macro_rules! warn {
    ($when: expr, $tok: expr, $($msg:tt)*) => {
        if $when {
            use $crate::diagnostic::KDiagnInfo;
            let msg = format!($($msg)*);
            KDiagnInfo::new(&msg, $tok).warn().emit()
        }
    };
}

#[macro_export]
macro_rules! trace {
    ($trace:expr, $($msg:tt)*) => {
        $trace.log(&format!($($msg)*))
    };
}

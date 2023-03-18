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
/// emit a compiler error
macro_rules! error {
    ($when: expr, $tok: expr, $($msg:tt)*) => {{
        use $crate::diagnostic::KDiagnInfo;
        let msg = format!($($msg)*);
        KDiagnInfo::new(&msg, $tok).emit()
    }};
}

#[macro_export]
macro_rules! trace {
    ($trace:expr, $($msg:tt)*) => {
        $trace.log(&format!($($msg)*))
    };
}

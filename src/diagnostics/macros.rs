#[macro_export]
macro_rules! debug {
    ($($args:tt)*) => {
       $crate::diagnostics::log::log($crate::diagnostics::log::LogLevel::Debug, format_args!($($args)*));
    };
}

#[macro_export]
macro_rules! info {
    ($($args:tt)*) => {
       $crate::diagnostics::log::log($crate::diagnostics::log::LogLevel::Info, format_args!($($args)*));
    };
}

#[macro_export]
macro_rules! warn {
    ($($args:tt)*) => {
       $crate::diagnostics::log::log($crate::diagnostics::log::LogLevel::Warn, format_args!($($args)*));
    };
}

#[macro_export]
macro_rules! error {
    ($($args:tt)*) => {
       $crate::diagnostics::log::log($crate::diagnostics::log::LogLevel::Error, format_args!($($args)*));
    };
}

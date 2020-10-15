extern crate macros;

#[cfg(test)]
mod tests;

#[cfg(feature = "macro")]
pub use macros::*;

#[cfg(feature = "bus")]
extern crate dbus;

pub const DBUS_NAME: &'static str = "org.gale.mandwm";

/// These are currently implemented in a way that will not be done in the final version of the
/// program. I definitely will replace them for macros sooner or later.
pub mod log {
    use crate::DBUS_NAME;

    #[macro_export]
    macro_rules! log_debug {
        () => {
            println!("{}, line {}: ", file!(), line!())
        };
        ($fmt:expr) => {
            println!("{}, line {}: {}", file!(), line!(), $fmt)
        };
        ($fmt:expr, $($args:tt)*) => {
            println!("{}, line {}: {}", file!(), line!(), format_args!($fmt, $($args)*))
        };
    }

    #[macro_export]
    macro_rules! log_info {
        ($fmt:expr) => {
            println!("{}", $fmt)
        };
        ($fmt:expr, $($args:tt)*) => {
            println!("{}", format_args!($fmt, $($args)*))
        };
    }

    #[macro_export]
    // maybe use escape sequences for color?
    macro_rules! log_warn {
        ($fmt:expr) => {
            println!("warn: {}", $fmt)
        };
        ($fmt:expr, $($args:tt)*) => {
            println!("warn: {}", format_args!($fmt, $($args)*))
        };
    }

    #[macro_export]
    // maybe use escape sequences for color?
    macro_rules! log_critical {
        ($fmt:expr) => {
            println!("CRITICAL {}, line {}: {}", file!(), line!(), $fmt)
        };
        ($fmt:expr, $($args:tt)*) => {
            println!("CRITICAL {}, line {}: {}", file!(), line!(), format_args!($fmt, $($args)*))
        };
    }
}

/// Since this doesn't take self, this must connect to dbus before
/// sending the string
// TODO Make a new ffi package in C for console use
pub fn set_primary_string(primary: String) {
    println!("{}: {}", DBUS_NAME, primary);
}

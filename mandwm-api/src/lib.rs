#[cfg(test)]
mod tests;

#[cfg(feature = "bus")]
extern crate dbus;

pub const DBUS_NAME: &'static str = "org.gale.mandwm";

pub mod log {
    use crate::DBUS_NAME;
    use std::fmt::{ Display, Debug };

    pub fn log_info<T: Display>(message: T) {
        println!("{} - Info: {}", DBUS_NAME, message);
    }

    pub fn log_debug<T: Debug>(message: T) {
        println!("{} - Debug: {:#?}", DBUS_NAME, message);
    }

    pub fn log_warn<T: Display>(message: T) {
        println!("{} - WARN: {}", DBUS_NAME, message);
    }

    pub fn log_critical<T: Display>(message: T) {
        println!("{} - CRITICAL: {}", DBUS_NAME, message);
    }
}

/// Since this doesn't take self, this must connect to dbus before
/// sending the string
// TODO Make a new ffi package in C for console use
pub fn set_primary_string(primary: String) {
    println!("{}: {}", DBUS_NAME, primary);
}

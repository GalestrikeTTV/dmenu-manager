#[cfg(test)]
mod tests;

#[cfg(feature = "core")]
pub mod core;

extern crate dbus;
use dbus::{blocking::LocalConnection, tree::Factory};
use std::process::{Command, Stdio};
use std::time::Duration;

const DBUS_NAME: &'static str = "com.gale.mandwm";

pub mod log {
    use crate::DBUS_NAME;

    pub fn log_info<T: Into<String>>(message: T) {
        println!("{} - Info: {}", DBUS_NAME, message.into());
    }

    pub fn log_debug<T: Into<String>>(message: T) {
        println!("{} - Debug: {}", DBUS_NAME, message.into());
    }

    pub fn log_warn<T: Into<String>>(message: T) {
        println!("{} - WARN: {}", DBUS_NAME, message.into());
    }

    pub fn log_critical<T: Into<String>>(message: T) {
        println!("{} - CRITICAL: {}", DBUS_NAME, message.into());
    }
}

/// Since this doesn't take self, this must connect to dbus before
/// sending the string
// TODO No mangle for the sake of FFI (?)
#[no_mangle]
pub fn set_primary_string(primary: &str) {

    println!("{}: {}", DBUS_NAME, primary);
}

extern crate dbus;
extern crate mandwm_api;
extern crate x11;

// Replace this with something easier to change (config file)
pub use mandwm_api::DBUS_NAME;

mod core;
#[cfg(test)]
mod tests;
mod xfuncs;

use crate::core::{AppendTo::*, *};
pub use mandwm_api::log::*;
use std::{thread, time::Duration};

fn main() {
    let mut mandwm = MandwmCore::setup_mandwm()
        .unwrap()
        .set_delimiter(" | ")
        .set_default_scripts(&["time", "date", "power"], std::path::PathBuf::from("./"))
        .connect()
        .unwrap();

    mandwm.set_primary_string("By default, mandwm only displays time and date.");

    // FIRST, NEXT, LAST, SHORTEST(?)
    mandwm.append(FIRST, "This is appended to the first string.");

    log_debug(format!("Mandwm {:?}", mandwm));

    let mandwm_mutex = mandwm.run();

    // TODO check for dbus messages here
    while mandwm_mutex.lock().unwrap().is_running() {
        log_info("Mandwm is running.");
        thread::sleep(Duration::new(1, 0));
    }
}

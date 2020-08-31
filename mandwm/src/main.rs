extern crate mandwm_api;
extern crate dbus;
extern crate x11;

// Replace this with something easier to change (config file)
pub use mandwm_api::DBUS_NAME;

mod core;
#[cfg(test)]
mod tests;

use std::{ thread, time::Duration, sync::{ Arc, Mutex } };

use crate::core::{ MandwmCore, AppendTo::* };
use mandwm_api::log::*;

fn main() {
    let mut mandwm = MandwmCore::setup_mandwm().unwrap()
        .set_delimiter(" | ")
        .set_default_scripts(&["time", "date", "power"], std::path::PathBuf::from("./"))
        .connect().unwrap();

    mandwm.set_primary_string("By default, mandwm only displays time and date.");

    // FIRST, NEXT, LAST, SHORTEST(?)
    mandwm.append(FIRST, "This is appended to the first string.");

    // Maybe spin this off into another thread so we can
    // check for messages on a loop.

    let mandwm_mutex = Arc::new(Mutex::new(mandwm));

    MandwmCore::run(Arc::clone(&mandwm_mutex));

    // TODO check for dbus messages here
    while mandwm_mutex.lock().unwrap().is_running() == true {
       log_info("Mandwm is running.");
       thread::sleep(Duration::new(10, 0));
    }
}


extern crate mandwm_api;

use std::{ thread, time::Duration, sync::{ Arc, Mutex } };

use mandwm_api::core::{ MandwmCore, AppendTo::* };
use mandwm_api::log::*;

fn main() {
    let mut mandwm = MandwmCore::setup_mandwm().unwrap()
        .set_delimiter("|")
        .set_default_scripts(&["time", "date", "power"], std::path::PathBuf::from("./"))
        .connect().unwrap();

    mandwm.set_primary_string("By default, mandwm only displays time and date.");

    // FIRST, NEXT, LAST, SHORTEST(?)
    mandwm.append(FIRST, "This is appended to the first string.");

    // Maybe spin this off into another thread so we can
    // check for messages on a loop.

    let mandwm_mutex = Arc::new(Mutex::new(mandwm));

    MandwmCore::run(Arc::clone(&mandwm_mutex));

    while mandwm_mutex.lock().unwrap().is_running() == true {
       log_info("Mandwm is running.");
       thread::sleep(Duration::new(10, 0));
    }

    log_critical("This app is not finished so getting here is pointless :(");

}


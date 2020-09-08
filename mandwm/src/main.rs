extern crate dbus;
extern crate mandwm_api;
extern crate x11;

// Replace this with something easier to change (config file)
pub use mandwm_api::DBUS_NAME;

mod core;
#[cfg(test)]
mod tests;
mod xfuncs;

use crate::{
    core::{AppendTo::*, *},
    xfuncs::*,
};
pub use mandwm_api::log::*;
use std::{thread, time::Duration};

fn main() {

    let config = mandwm_handle_args().unwrap();

    log_debug(format!("Config: {}", config));

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

    mandwm_run(&mut mandwm, &config);
}

fn mandwm_run(mandwm: &mut MandwmCore, config: &MandwmConfig) {
    mandwm.is_running = true;

    log_debug("Starting mandwm.");

    let mut final_str: String = String::new();
    let mut counter: u8 = 0;
    while mandwm.is_running {
        final_str = String::new();
        for string in mandwm.dwm_bar_string.iter() {
            final_str.push_str(string.as_str());
        }

        xdisplay_set_root(final_str, config.display_var).unwrap();

        thread::sleep(Duration::from_secs(1));

        counter += 1;
        if counter > 5 {
            break;
        }
    }

    mandwm.is_running = false;
}

fn mandwm_handle_args() -> Result<MandwmConfig, MandwmError> {
    let mut config = MandwmConfig {
        display_var: "",
    };

    let display = env!("DISPLAY");

    if display.is_empty() {
        return Err(MandwmError::critical(String::from("DISPLAY variable not set")));    
    } else {
        config.display_var = display;
    }

    Ok(config)
}

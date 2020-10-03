// TODO remove this once the thing works in general
#![allow(dead_code)]
#![allow(unused_imports)]

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
use std::{thread, time::Duration, path::Path, process::{Command, Stdio}};

fn main() {
    let config = mandwm_handle_args().unwrap();

    log_debug(format!("Config: {}", config));

    let mut mandwm = MandwmCore::setup_mandwm()
        .unwrap()
        .set_delimiter(" | ")
        .set_shell_scripts(&["time", "date", "power"], std::path::PathBuf::from("./"))
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

    while mandwm.is_running {
        // Spin off a thread that checks for dbus messages
        // (Maybe not even spin off a thread? Dunno if it blocks
        
        // Check for the timer to see if we should output the bar
        
        // Execute bar scripts
        let sh_scripts = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "mandwm/scripts/shell/"));
        for script in sh_scripts.iter() {
            log_debug(script);
        }
        
        // Set the root
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
        log_debug(display);
    }

    Ok(config)
}

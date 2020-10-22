// TODO remove this once the thing works in general
#![allow(dead_code)]
#![allow(unused_imports)]

#[macro_use]
extern crate mandwm_api;
#[macro_use]
extern crate tokio;

// Replace this with something easier to change (config file)
pub use mandwm_api::DBUS_NAME;

mod core;
mod dbus;
#[cfg(test)]
mod tests;
mod xfuncs;

use crate::{
    core::{AppendTo::*, *},
    xfuncs::*,
};
pub use mandwm_api::{ log::*, error::*};
use std::{
    path::Path,
    process::{Command, Stdio},
    thread,
    time::Duration,
};

#[tokio::main]
async fn main() {
    let config = mandwm_handle_args().unwrap();

    log_debug!(format!("Config: {}", config));

    let mut mandwm = MandwmCore::setup_mandwm().unwrap();
    mandwm.set_primary_string("By default, mandwm only displays time and date.");
    // FIRST, NEXT, LAST, SHORTEST(?); different algorithms for where the string should be placed.
    mandwm.append(First, "This is appended to the first string.");

    log_debug!("Mandwm {:?}", mandwm);

    // TODO move this inside of the struct implementation, also have it return information about
    // the dbus and 'should close' variables
    mandwm_run(&mut mandwm, &config);
    let runner = mandwm.run(&config);
    /*
    while runner.should_run {
        // Check dbus
    }
    */
}

#[allow(unused_variables)] // TODO remove this
fn mandwm_run(mandwm: &mut MandwmCore, config: &MandwmConfig) {
    mandwm.is_running = true;

    log_info!("Starting mandwm.");

    while mandwm.is_running {
        // Spin off a thread that checks for dbus messages
        // (Maybe not even spin off a thread? Dunno if it blocks

        // Check for the timer to see if we should output the bar
        // Execute bar scripts

        thread::sleep(Duration::from_secs(1));

        // Set the root
    }

    mandwm.is_running = false;
}

fn mandwm_handle_args() -> Result<MandwmConfig, MandwmError> {
    let mut config = MandwmConfig { display_var: "" };

    let display = env!("DISPLAY");

    if display.is_empty() {
        return Err(MandwmError::critical(String::from(
            "DISPLAY variable not set",
        )));
    } else {
        config.display_var = display;
        log_debug!(display);
    }

    Ok(config)
}

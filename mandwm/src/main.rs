// TODO remove this once the thing works in general
#![allow(dead_code)]
#![allow(unused_imports)]

#[macro_use]
extern crate mandwm_api;
#[macro_use]
extern crate tokio;

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

    log_debug!("Config: {}", config);

    let mut mandwm = MandwmCore::setup_mandwm().unwrap();
    mandwm.set_primary_string("By default, mandwm only displays time and date.");
    // FIRST, NEXT, LAST, SHORTEST(?); different algorithms for where the string should be placed.
    mandwm.append(First, "This is appended to the first string.");

    log_debug!("Mandwm {:?}", mandwm);

    // TODO move this inside of the struct implementation, also have it return information about
    // the dbus and 'should close' variables
    // mandwm_run(&mut mandwm, &config);
    let runner = mandwm.run(&config);
    
    // THIS CODE IS REACHED BEFORE THE RUNNER ACTUALLY STARTS
    while runner.is_running.lock().unwrap().eq(&true) {
        // lock should be dropped by now if not already
        
        // Check dbus
    }

    runner.handle.await.expect("There was a problem with the thread")
        .expect("There was an error running the core loop.");
}

fn mandwm_handle_args() -> Result<MandwmConfig, MandwmError> {
    let mut config = MandwmConfig { display_var: "", use_stdout: false };

    let args: Vec<String> = std::env::args().collect();

    let display = env!("DISPLAY");

    if display.is_empty() {
        return Err(MandwmError::critical(String::from(
            "DISPLAY variable not set",
        )));
    } else {
        config.display_var = display;
        log_debug!(display);
    }

    for arg in args {
        // Route to stdout instead of xsetroot
        if arg == "--stdout" || arg == "-s" {
            log_info!("Printing to stdout instead of xsetroot");
            config.use_stdout = true;
        }
    }

    Ok(config)
}

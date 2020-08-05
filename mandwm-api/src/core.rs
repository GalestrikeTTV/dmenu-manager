use crate::log::*;
/// The module `core` is used to initialize a session mandwm,
/// if it doesn't already exist
use crate::DBUS_NAME;

use MandwmErrorLevel::*;

use dbus::{blocking::LocalConnection, tree::Factory};
use std::thread;
use std::time::Duration;
use std::sync::{ Arc, Mutex };

#[derive(Debug)]
pub struct MandwmError {
    msg: String,
    level: MandwmErrorLevel,
}

impl MandwmError {
    pub fn critical(message: String) -> Self {
        MandwmError {
            msg: message,
            level: CRITICAL,
        }
    }

    pub fn warn(message: String) -> Self {
        MandwmError {
            msg: message,
            level: WARN,
        }
    }

    pub fn debug(message: String) -> Self {
        MandwmError {
            msg: message,
            level: DEBUG,
        }
    }
}

#[derive(Debug)]
pub enum MandwmErrorLevel {
    CRITICAL,
    WARN,
    DEBUG,
}

#[derive(Debug)]
pub enum AppendTo {
    FIRST,
    NEXT,
    LAST,
    SHORTEST,
}

pub struct MandwmCore {
    pub dwm_bar_string: Vec<String>,
    pub delimiter: String,
    is_running: bool,
    max_length: usize,
}

impl MandwmCore {
    pub fn setup_mandwm() -> Result<MandwmCore, Box<dyn std::error::Error>> {
        // We'll do something with this later, just to make sure we're running as daemon or something.
        let _args: Vec<String> = std::env::args().collect();

        Ok(MandwmCore::default())
    }

    /// Called once the MandwmCore object is initialized.
    pub fn connect(mut self) -> Result<Self, MandwmError> {
        let conn = match LocalConnection::new_session() {
            Ok(c) => c,
            Err(e) => {
                return Err(MandwmError::critical(format!(
                    "Could not connect to dbus. Error: {}",
                    e
                )));
            }
        };

        match conn.request_name(DBUS_NAME, false, true, false) {
            Ok(_) => {}
            Err(e) => {
                return Err(MandwmError::critical(format!(
                    "Could not request name \"{}\" from dbus. ERROR: {}",
                    DBUS_NAME, e
                )));
            }
        }

        let factory = Factory::new_fn::<()>();

        let proxy = conn.with_proxy("org.freedesktop.DBus", "/", Duration::from_millis(5000));

        let (names,): (Vec<String>,) = proxy
            .method_call("org.freedesktop.DBus", "ListNames", ())
            .unwrap();
        for name in names {
            println!("{:?}", name);
        }
        match conn.release_name(DBUS_NAME) {
            Ok(_) => {
                self.is_running = true;
            }
            Err(e) => {
                return Err(MandwmError::warn(format!(
                    "Could not release name of {}. ERROR: {}",
                    DBUS_NAME, e,
                )));
            }
        };

        Ok(self)
    }

    pub fn is_running(&self) -> bool {
        self.is_running
    }

    fn set_running(&mut self, run: bool) {
        self.is_running = run;
    }

    pub fn set_delimiter<T: Into<String>>(mut self, delimiter: T) -> Self {
        self.delimiter = delimiter.into();
        self
    }

    pub fn set_primary_string<T: Into<String>>(&mut self, message: T) {
        if self.dwm_bar_string.len() >= 1 {
            self.dwm_bar_string[0] = message.into();
        } else {
            self.dwm_bar_string.push(message.into());
        }
        println!("Primary string set.");
    }

    pub fn append<T: Into<String>>(&mut self, place: AppendTo, message: T) {
        use AppendTo::*;

        let append_message = message.into();

        // TODO later
        // Change this so that it appends whatever message was sent
        // after the set delimiter
        log_debug("MandwmCore.append is currently unimplemented.");

        match place {
            FIRST => {
                // Append to first part of the list.
                self.dwm_bar_string.insert(0, append_message);
            }
            LAST => {
                // Append to the end of the list
                self.dwm_bar_string.push(append_message);
            }
            SHORTEST => {
                // Append to the shortest list
                unimplemented!(
                    "MandwmCore does not contain a way to know which list is where yet."
                );
            }
            NEXT => {
                // Append to the next available spot
                unimplemented!(
                    "MandwmCore does not contain a way to know which list is where yet."
                );
            }
        }
    }

    pub fn run(core: Arc<Mutex<MandwmCore>>) {

        core.lock().unwrap().set_running(true);

        thread::spawn(move || {
            thread::sleep(Duration::from_secs(5));
            println!("Mandwm has finished running");

            core.lock().unwrap().set_running(false);
        });
    }
}

impl Default for MandwmCore {
    fn default() -> Self {
        MandwmCore {
            dwm_bar_string: Vec::new(),
            delimiter: " | ".to_string(),
            is_running: false,
            // TODO find a way to figure this out from dwm
            max_length: 50,
        }
    }
}

use crate::DBUS_NAME;

use MandwmErrorLevel::*;

use dbus::{blocking::Connection, blocking::LocalConnection, tree::Factory};
use std::ffi::CString;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use x11::xlib::*;

use crate::xfuncs::*;
use mandwm_api::log::*;

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

#[derive(Debug)]
pub struct MandwmCore {
    pub dwm_bar_string: Vec<String>,
    pub default_scripts: Vec<Command>,
    pub scripts_path: PathBuf,
    pub delimiter: String,
    pub is_running: bool,
    pub should_close: bool,
    pub max_length: usize,
}

impl MandwmCore {
    pub fn setup_mandwm() -> Result<MandwmCore, Box<dyn std::error::Error>> {
        // We'll do something with this later, just to make sure we're running as daemon or something.
        let _args: Vec<String> = std::env::args().collect();

        Ok(MandwmCore::default())
    }

    /// Called once the MandwmCore object is initialized.
    pub fn connect(mut self) -> Result<Self, MandwmError> {
        // We don't connect to the X11 display here because it messes up since we're working
        // across threads.

        // Connect to DBUS
        {
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
        }

        Ok(self)
    }

    pub fn set_delimiter<T: Into<String>>(mut self, delimiter: T) -> Self {
        self.delimiter = delimiter.into();
        self
    }

    pub fn set_default_scripts(mut self, slice: &[&str], path: PathBuf) -> Self {
        use std::fs::read_dir;

        // TODO Cache default scripts (create a command and clone them into a vec?)

        let res = read_dir(concat!(env!("CARGO_MANIFEST_DIR"), "/scripts/default/"))
            .unwrap()
            .map(|res| res.map(|e| e.path()))
            .collect::<Result<Vec<_>, std::io::Error>>()
            .unwrap();

        for path in res.iter() {
            log_debug(path);
        }

        self
    }

    pub fn set_primary_string<T: Into<String>>(&mut self, message: T) {
        if self.dwm_bar_string.len() >= 1 {
            self.dwm_bar_string[0] = message.into();
        } else {
            self.dwm_bar_string.push(message.into());
        }
        log_debug("Primary string set.");
    }

    pub fn append<T: Into<String>>(&mut self, place: AppendTo, message: T) {
        use AppendTo::*;

        let append_message = message.into();

        // Change this so that it appends whatever message was sent
        // after the set delimiter

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

    /*
    pub fn run(mut self) -> Arc<Mutex<Self>> {
        self.is_running = true;

        let mutex = Arc::new(Mutex::new(self));

        let thread_mutex = mutex.clone();

        thread::spawn(move || {
            log_debug("Starting mandwm.");

            let mut final_str: String = String::new();
            let mut counter: u8 = 0;
            while thread_mutex.lock().unwrap().is_running {
                final_str = String::new();
                for string in thread_mutex.lock().unwrap().dwm_bar_string.iter() {
                    final_str.push_str(string.as_str());
                }

                let res =
                    xdisplay_set_root(final_str).unwrap();

                thread::sleep(Duration::from_secs(1));

                counter += 1;
                if counter > 5 {
                    break;
                }
            }

            thread_mutex.lock().unwrap().set_running(false);

            log_debug("Mandwm has finished running");
        });

        return mutex;
    }
    */
}

impl Default for MandwmCore {
    fn default() -> Self {
        MandwmCore {
            dwm_bar_string: Vec::new(),
            delimiter: " | ".to_string(),
            default_scripts: vec![],
            scripts_path: PathBuf::new(),
            is_running: false,
            should_close: false,
            // TODO find a way to figure this out from dwm
            max_length: 50,
        }
    }
}

pub struct MandwmConfig {
    pub display_var: &'static str,
}

impl std::fmt::Display for MandwmConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DISPLAY: {}", self.display_var)
    }
}

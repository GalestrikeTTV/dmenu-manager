use crate::DBUS_NAME;

use MandwmErrorLevel::*;

use dbus::{blocking::LocalConnection, blocking::Connection, tree::Factory};
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use std::path::{ Path, PathBuf };

use mandwm_api::log::*;

// Send u8 instead of nothing
pub fn set_root_name<T: Into<String>>(string: T) -> Result<u8, MandwmError> {
    use std::ffi::CString;
    use x11::*;
    
    let name: CString = match CString::new(string.into()) {
        Ok(name) => { name },
        Err(e) => {
            return Err(MandwmError::critical(format!("Invalid name for set_root_name: {}", e)));
        }
    };

    // TODO check the length of the string and make sure that it isn't too big
    
    unsafe {
        let mut display = xlib::XOpenDisplay(std::ptr::null());

        if display.is_null() {
            panic!("XOpenDisplay failed");
        }

        // Create window.
        let screen = xlib::XDefaultScreen(display);
        let root = xlib::XRootWindow(display, screen);

        let res = x11::xlib::XStoreName(display, root, name.as_ptr());

        xlib::XCloseDisplay(display);
    }

    Ok(0)
}

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
    default_scripts: Vec<Command>,
    scripts_path: PathBuf,
    delimiter: String,
    is_running: bool,
    should_close: bool,
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

    pub fn set_default_scripts(mut self, slice: &[&str], path: PathBuf) -> Self {
        use std::fs::read_dir;

        // TODO Cache default scripts (create a command and clone them into a vec?)

        let res = read_dir("./").unwrap()
            .map(|res| res.map(|e| e.path()))
            .collect::<Result<Vec<_>, std::io::Error>>().unwrap();

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

    pub fn run(core: Arc<Mutex<MandwmCore>>) {
        core.lock().unwrap().set_running(true);

        thread::spawn(move || {
            log_debug("Starting mandwm.");

            while core.lock().unwrap().should_close == false {
                // Check for dbus messages
                //  <=== TODO

                /* let mut command = Command::new("xsetroot");
                command.arg("-name");

                let dwm_bar_string = core.lock().unwrap().dwm_bar_string.clone();
                let delimiter = core.lock().unwrap().delimiter.clone();
                let mut final_string = String::new();

                for (i, bar_string) in dwm_bar_string.iter().enumerate() {
                    if i >= dwm_bar_string.len() || i == 0 {
                        final_string.push_str(bar_string.as_str());
                    } else {
                        final_string.push_str(format!(" {} {}", delimiter, bar_string).as_str());
                    }
                }

                command.arg(format!(" {} ", final_string));

                let output = command.output().unwrap();

                if output.stderr.len() > 0 {
                    log_critical(String::from_utf8(output.stderr.to_vec()).unwrap());
                }

                */

                let mut final_str: String = String::new();
                for string in core.lock().unwrap().dwm_bar_string.iter() {
                    final_str.push_str(string.as_str());
                }

                set_root_name(final_str).unwrap();

                thread::sleep(Duration::from_secs(1));
            }

            core.lock().unwrap().set_running(false);

            log_debug("Mandwm has finished running");
        });
    }
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


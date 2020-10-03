// TODO remove checks once the core functionality is set
// I wrote this and I have no idea what this means. I think I wanted to not rely on this crate's
// DBUS_NAME
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
pub struct MandwmCore<'a> {
    /// A cached version of what's on the dwm bar
    // Might need to wrap this for clarity
    dwm_bar_string: Vec<String>,
    shell_scripts: Vec<MandwmCommand<'a>>,
    delimiter: String,
    pub is_running: bool,
    should_close: bool,
    max_length: usize,
}

impl<'a> MandwmCore<'a> {
    pub fn setup_mandwm() -> Result<MandwmCore<'a>, Box<dyn std::error::Error>> {
        // TODO We'll do something with this later, just to make sure we're running as daemon or something.
        let _args: Vec<String> = std::env::args().collect();

        Ok(MandwmCore::default())
    }

    /// Sets up the DBUS/TCP connection.
    pub fn connect(mut self) -> Result<Self, MandwmError> {
        // Connect to DBUS
        let conn = LocalConnection::new_session().unwrap();
        conn.request_name(DBUS_NAME, false, true, false).unwrap();

        let factory = Factory::new_fn::<()>();

        // TODO get rid of this once you know what you are doing
        let signal = Arc::new(
            factory
                .signal("SomethingHappened", ())
                .sarg::<&str, _>("sender"),
        );
        let signal2 = signal.clone();

        // Programmer notes
        //
        // GENERAL STRUCTURE
        // * Create all methods and properties
        // * Put methods into interfaces
        // * Add interfaces into an object path in a tree

        let method_rerun_scripts = factory.method("RerunScripts", (), move |m| {
            // TODO send a command to rerun scripts
            Ok(vec![m.msg.method_return()])
        })
        // .outarg for returning args
        // .inarg for taking args
        ;

        let mandwm_interface = factory.interface(DBUS_NAME, ()).add_m(method_rerun_scripts);

        let mandwm_tree = factory.tree(()).add(
            factory
                .object_path("org/gale/mandwm", ())
                .introspectable()
                .add(mandwm_interface),
        );

        mandwm_tree.start_receive(&conn);

        // TODO move this to the main loop
        conn.process(Duration::from_secs(1)).unwrap();

        Ok(self)
    }

    pub fn set_delimiter<T: Into<String>>(mut self, delimiter: T) -> Self {
        self.delimiter = delimiter.into();
        self
    }

    /// Takes all scripts in the scripts directory and creates Commands for them.
    /// Functionality for this will be expanded in the future.
    pub fn parse_shell_scripts(&self) {
        use std::fs::read_dir;

        let path = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/scripts/shell/"));
        let res = read_dir(path)
            .unwrap()
            .map(|res| res.map(|e| e.path()).unwrap())
            .collect::<Vec<_>>();

        let mut scripts = Vec::<MandwmCommand>::new();

        for script_path in res.iter() {
            // TODO better error handling
            let name: &str = script_path
                .to_str()
                .expect("Failed to get script in script path.");
            let mut script = Command::new("sh");
            script.args(&["-c", name]);

            let command = MandwmCommand { name, script, path };
            scripts.push(command);
        }

        self.shell_scripts = scripts;
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

impl<'a> Default for MandwmCore<'a> {
    fn default() -> Self {
        MandwmCore {
            dwm_bar_string: Vec::new(),
            delimiter: " | ".to_string(),
            shell_scripts: vec![],
            is_running: false,
            should_close: false,
            // TODO find a way to figure this out from dwm or X
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

#[derive(Debug)]
struct MandwmCommand<'a> {
    name: &'a str,
    script: Command,
    path: &'a Path, // This is a PathBuf because Path isn't sized
}

impl<'a> MandwmCommand<'a> {
    pub fn set_name(&mut self, name: &'a str) {
        self.name = name;
    }

    // Will return the formatted output (from a cache too potentially)
    // Returns stdout or stderr
    pub fn output(&mut self) -> Result<String, MandwmError> {
        let finished = self.script.output();
        match finished {
            Ok(v) => {
                let out = String::from_utf8(v.stdout).unwrap();
                let err = String::from_utf8(v.stderr).unwrap();
                log_debug(format!("STDOUT: {}\nSTDERR: {}", out, err));
                if v.status.success() {
                    Ok(out)
                } else {
                    Err(MandwmError::warn(format!(
                        "Error executing command.\nName: {}\nSTDERR: {}",
                        self.name, err
                    )))
                }
            }
            Err(e) => Err(MandwmError::warn(format!(
                "Unable to execute command.\nName: {}\nExecution error: {}",
                self.name, e
            ))),
        }
    }
}

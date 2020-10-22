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

use crate::{dbus::*, xfuncs::*};
use mandwm_api::log::*;

// TODO move these to mandwm-api
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
pub struct MandwmCore<'core> {
    /// A cached version of what's on the dwm bar.
    // Might need to wrap this for clarity
    dwm_bar_string: Vec<String>,
    /// Commands that run on their specified timers.
    shell_scripts: Vec<MandwmCommand<'core>>,
    /// The delimiter between the output of each shell script.
    delimiter: String,
    pub is_running: bool,
    /// States whether or not the app needs to be cleaned up. Useful for checking whether or not a
    /// dbus message has been sent to shut down the daemon.
    should_close: bool,
    /// The maximum length of the string that mandwm will send to xsetroot.
    max_length: usize,
}

impl<'core> MandwmCore<'core> {
    pub fn setup_mandwm() -> Result<MandwmCore<'core>, Box<dyn std::error::Error>> {
        // TODO We'll do something with this later, just to make sure we're running as daemon or something.
        let _args: Vec<String> = std::env::args().collect();

        let mut mandwm = MandwmCore::default();
        mandwm.parse_shell_scripts();

        Ok(mandwm)
    }

    pub fn run(mut self, _config: &MandwmConfig) -> MandwmRunner {
        MandwmRunner { is_running: true }
    }

    async fn internal_run(mut self) {
        self.is_running = true;

        log_info!("Starting mandwm.");

        while self.is_running {
            // Spin off a thread that checks for dbus messages
            // (Maybe not even spin off a thread? Dunno if it blocks

            // Check for the timer to see if we should output the bar
            // Execute bar scripts

            thread::sleep(Duration::from_secs(1));

            // Set the root
        }

        self.is_running = false;
        log_info!("Mandwm has finished running");
    }

    /// Sets up the DBUS/TCP connection.
    #[allow(unused_mut)] // TODO remove this
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
        let _signal2 = signal.clone();

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

    pub fn set_delimiter<T: Into<String>>(&mut self, delimiter: T) {
        self.delimiter = delimiter.into();
    }

    /// Takes all scripts in the scripts directory and creates Commands for them.
    /// Functionality for this will be expanded in the future.
    fn parse_shell_scripts(&mut self) {
        let path = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/scripts/shell/"));
        let res = std::fs::read_dir(path)
            .expect("Could not read shell script directory")
            .map(|res| res.map(|e| e.path()).unwrap())
            .collect::<Vec<_>>();

        let mut scripts = Vec::<MandwmCommand>::new();

        for script_path in res {
            // TODO better error handling
            let owned_script_path = script_path.to_owned();
            let script = Command::new(owned_script_path.clone());

            let name = owned_script_path.into_os_string();
            let command = MandwmCommand { name, script, path };
            scripts.push(command);
        }

        self.shell_scripts = scripts;
    }

    // TODO return a Result
    pub fn set_primary_string<T: Into<String>>(&mut self, message: T) {
        if self.dwm_bar_string.len() >= 1 {
            self.dwm_bar_string[0] = message.into();
        } else {
            self.dwm_bar_string.push(message.into());
        }
        log_debug!("Primary string set.");
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
}

impl<'core> Default for MandwmCore<'core> {
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

#[derive(Debug, Copy, Clone)]
pub struct MandwmConfig {
    pub display_var: &'static str,
}

impl std::fmt::Display for MandwmConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DISPLAY: {}", self.display_var)
    }
}

#[derive(Debug)]
struct MandwmCommand<'comm> {
    name: std::ffi::OsString,
    script: Command,
    path: &'comm Path,
}

impl<'comm> MandwmCommand<'comm> {
    pub fn set_name(&mut self, name: &'comm str) {
        self.name = name.into();
    }

    // Will return the formatted output (from a cache too potentially)
    // Returns stdout or stderr
    pub fn output(&mut self) -> Result<String, MandwmError> {
        let finished = self.script.output();
        match finished {
            Ok(v) => {
                let out = String::from_utf8(v.stdout).unwrap();
                let err = String::from_utf8(v.stderr).unwrap();
                log_debug!(format!("STDOUT: {}\nSTDERR: {}", out, err));
                if v.status.success() {
                    Ok(out)
                } else {
                    Err(MandwmError::warn(format!(
                        "Error executing command.\nName: {:?}\nSTDERR: {}",
                        self.name, err
                    )))
                }
            }
            Err(e) => Err(MandwmError::warn(format!(
                "Unable to execute command.\nName: {:?}\nExecution error: {}",
                self.name, e
            ))),
        }
    }
}

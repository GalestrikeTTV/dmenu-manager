use crate::core::*;
use mandwm_api::{error::*, log::*};
use std::{
    ffi::{CStr, CString},
    os::raw::*,
    ptr::{null, null_mut},
};
use x11::xlib::*;

pub type XDisplay = *mut Display;
pub type XScreen = i32;
pub type XRoot = u64;
/// A wrapper for the xlib display types
#[derive(Debug)]
pub struct MandwmDisplay(XDisplay, XScreen, XRoot);

impl MandwmDisplay {
    pub fn get_display(&self) -> XDisplay {
        self.0
    }

    pub fn get_screen(&self) -> XScreen {
        self.1
    }

    pub fn get_root(&self) -> XRoot {
        self.2
    }
}

impl Default for MandwmDisplay {
    fn default() -> Self {
        MandwmDisplay(std::ptr::null_mut::<Display>() as XDisplay, -1, 0)
    }
}

pub fn xdisplay_connect(display_var: &'static str) -> Option<MandwmDisplay> {
    let (display, screen, root) = unsafe {
        let display_var_ptr: *const i8 = if display_var.is_empty() && display_var.is_ascii() {
            null()
        } else {
            CString::new(display_var).unwrap().as_ptr()
        };

        let display = XOpenDisplay(":0\0".as_ptr() as *mut c_char);

        if display == null_mut() {
            return None;
        }

        let screen = XDefaultScreen(display);
        let root = XRootWindow(display, screen);

        log_debug!(format!(
            "display: {:?}, screen: {}, root: {}",
            display, screen, root
        ));

        (display, screen, root)
    };

    Some(MandwmDisplay(display, screen, root))
}

/// I'm trying to optimize this so you don't have to open and close a display every time that you
/// set the root but it doesn't seem to be working on my system for whatever reason.
pub fn xdisplay_fast_set_root(name: String, display: &MandwmDisplay) -> Result<(), MandwmError> {
    // Use this as the name later on
    let _null_term_name = CString::new(name).unwrap();
    unsafe {
        let res = XStoreName(
            display.get_display(),
            display.get_root(),
            // null_term_name.as_ptr() as *const i8,
            null() as _,
        );
        log_debug!(format!("Result of set_root: {}", res));
    }

    Ok(())
}

/// Replacement for xsetroot which is about 1.5x faster in Rust since we aren't spawning a shell
pub fn xdisplay_set_root(name: String, display_var: &'static str) -> Result<(), MandwmError> {
    let display = xdisplay_connect(display_var).unwrap();

    let null_term_name = CString::new(name).unwrap();

    // This SHOULD work without destructing the connection every time.
    // I think it's because the X server doesn't flush the queue until
    // the screen is disconnected.
    // TODO find a way to flush the X queue manually.
    unsafe {
        let res = XStoreName(
            display.get_display(),
            display.get_root(),
            null_term_name.as_ptr() as *const i8,
        );
        log_debug!(format!("Result of set_root: {}", res));
    }

    xdisplay_disconnect(&display);

    Ok(())
}

pub fn xdisplay_disconnect(xdisplay: &MandwmDisplay) {
    unsafe {
        let res = XCloseDisplay(xdisplay.get_display());
        log_debug!(format!("Result of disconnect_display: {}", res));
    }
}

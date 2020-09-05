use crate::core::*;
use mandwm_api::log::*;
use x11::xlib::*;
use std::ffi::CString;

pub type XDisplay = *mut Display;
pub type XScreen = i32;
pub type XRoot = u64;
/// A wrapper for the xlib display types because they aren't thread safe.
#[derive(Debug)]
pub struct MandwmDisplay(XDisplay, XScreen, XRoot);

unsafe impl Send for MandwmDisplay {}
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
        use std::ptr::null;
        MandwmDisplay(null::<Display> as XDisplay, -1, 0)
    }
}

pub fn xdisplay_connect() -> Option<MandwmDisplay> {
    let (display, screen, root) = unsafe {
        let display = XOpenDisplay(std::ptr::null());

        if display.is_null() {
            return None;
        }

        let screen = XDefaultScreen(display);
        let root = XRootWindow(display, screen);

        log_debug(format!("screen: {}, root: {}", screen, root));

        (display, screen, root)
    };

    Some(MandwmDisplay(display, screen, root))
}

/// Replacement for xsetroot which is about 1.5x faster in Rust since we aren't spawning a shell
pub fn xdisplay_set_root(name: String) -> Result<(), MandwmError> {
    let null_term_name = CString::new(name.as_str()).unwrap();

    // TODO figure out why we have to connect to X11 every time that we run this function and not
    // just store the display in a mutex (maybe because we're running across threads?) 
    let display = xdisplay_connect().unwrap();

    unsafe {
        let res = XStoreName(
            display.get_display(),
            display.get_root(),
            null_term_name.as_ptr() as *const i8,
        );
        log_debug(format!("Result of set_root: {}", res));
    }

    xdisplay_disconnect(display);

    Ok(())
}

pub fn xdisplay_disconnect(xdisplay: MandwmDisplay) {
    unsafe {
        let res = XCloseDisplay(xdisplay.get_display());
        log_debug(format!("Result of disconnect_display: {}", res));
    }
}

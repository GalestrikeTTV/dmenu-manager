const ITER_AMT: u32 = 1_000_000;

#[test]
fn speed_of_xsetroot_output() {
    use std::{ process::Command, time::Instant };

    let mut output = Command::new("xsetroot");
    output.args(&["-name", "seesaw-hehehe"]);

    let now = Instant::now();

    for _ in 0..ITER_AMT {
        output.output().unwrap();
    }
    
    println!("Time to completion: {}", now.elapsed().as_secs_f64());
}

#[test]
fn speed_of_custom_set_root_optimized() {

    use std::{ time::Instant, ffi::CString};
    use x11::xlib;
    
    let (display, root) = unsafe {
        let display = xlib::XOpenDisplay(std::ptr::null());
        if display.is_null() {
            panic!("XOpenDisplay failed");
        }

        
        let screen = xlib::XDefaultScreen(display);
        let root = xlib::XRootWindow(display, screen);
        
        (display, root)
    };

    let name = CString::new("Optimized SetRoot.").unwrap();

    let now = Instant::now();

    for _ in 0..ITER_AMT {      
        unsafe { 
            xlib::XStoreName(display, root, name.as_ptr());
        }
    }

    unsafe {
        xlib::XCloseDisplay(display);
    }
    
    println!("Time to completion: {}", now.elapsed().as_secs_f64());
}

#[test]
fn speed_of_custom_set_root_slow() {
    
    use std::{ time::Instant, ffi::CString};
    use x11::xlib;
    
    let name = CString::new("Unoptimized SetRoot.").unwrap();
    let now = Instant::now();

    for _ in 0..ITER_AMT {      
        unsafe { 
            
            let mut display = xlib::XOpenDisplay(std::ptr::null());
            if display.is_null() {
                panic!("XOpenDisplay failed");
            }

            let screen = xlib::XDefaultScreen(display);
            let root = xlib::XRootWindow(display, screen);
            let res = x11::xlib::XStoreName(display, root, name.as_ptr());

            xlib::XCloseDisplay(display);
        }
    }

    
    println!("Time to completion: {}", now.elapsed().as_secs_f64());
}

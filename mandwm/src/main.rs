extern crate mandwm_api;

use mandwm_api::core::{ setup_mandwm, AppendTo::* };
use mandwm_api::log::*;

fn main() {
    let mut mandwm = setup_mandwm().unwrap()
        .set_delimiter("|")
        .connect().unwrap();

    mandwm.set_primary_string("By default, mandwm only displays time and date.");

    // FIRST, NEXT, LAST, SHORTEST(?)
    mandwm.append(FIRST, "This is appended to the first string.");

    // Maybe spin this off into another thread so we can check for messages on a loop.
    let thread = mandwm.run();

    while mandwm.is_running() == true {
       println!("Mandwm is running.");
    }

    log_critical("This app is not finished so getting here is pointless :(");
}

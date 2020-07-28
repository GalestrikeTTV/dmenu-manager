extern crate mandwm_api;

fn main() {
    mandwm_api::init_mandwm().unwrap();

    mandwm_api::set_primary_string("Test this string.");
}

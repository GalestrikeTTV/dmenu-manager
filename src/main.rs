extern crate dbus;

use dbus::{ blocking::LocalConnection, tree::Factory };
use std::process::{Command, Stdio};
use std::time::Duration;

const DBUS_NAME: &'static str = "com.gale.mandwm";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _args: Vec<String> = std::env::args().collect();

    let conn = LocalConnection::new_session()?;

    conn.request_name(DBUS_NAME, false, true, false)?;

    let factory = Factory::new_fn::<()>();

    let proxy = conn.with_proxy("org.freedesktop.DBus", "/", Duration::from_millis(5000));

    let (names,): (Vec<String>,) = proxy.method_call("org.freedesktop.DBus", "ListNames", ())?;

    for name in names {
        println!("{:?}", name);
    }

    conn.release_name(DBUS_NAME)?;

    Ok(())
}

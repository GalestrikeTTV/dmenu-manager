/// Given after mandwm.run() is called. It should give off events based on what's happening on the
/// dbus.
use crate::core::*;
use mandwm_api::error::*;
use std::future::Future;
use std::sync::{ Arc, Mutex };
use tokio::task::JoinHandle;

pub type CoreHandle = JoinHandle<Result<(), MandwmError>>;
pub struct MandwmRunner{
    // pub internal: Arc<Mutex<MandwmCore>>,
    pub handle: CoreHandle,
    pub is_running: Arc<Mutex<bool>>,
}

// TODO here put all the dbus initialization code

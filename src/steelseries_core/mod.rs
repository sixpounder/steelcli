//# This modules contains all the basic structs and traits to interact with Steelseries devices
//#
use std::time::Duration;

mod color;
mod log;
mod steelseries_device;
mod errors;
mod side;

pub mod support;
pub use color::*;
pub use log::*;
pub use steelseries_device::*;
pub use errors::*;
pub use side::*;

pub const STEELSERIES_VENDOR_ID: u16 = 0x1038;

#[derive(Debug)]
pub struct URBConfiguration {
    pub request_type: u8,
    pub request: u8,
    pub index: u16,
    pub value: u16,
    pub debug_message: Option<String>,
    pub buf: Option<Vec<u8>>,
    pub timeout: Duration,
}

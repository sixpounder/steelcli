//# This modules contains all the basic structs and traits to interact with Steelseries devices
//#

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

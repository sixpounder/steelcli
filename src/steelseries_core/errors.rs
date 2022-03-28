/// All error types used internally
#[derive(Debug)]
pub enum Error {
    Generic(&'static str),
    NoDevice,
    Privileges,
    Conversion,
    InvalidCapability,
    Usb(hidapi::HidError),
    UsbComm
}

impl From<hidapi::HidError> for Error {
    fn from(original: hidapi::HidError) -> Self {
        Error::Usb(original)
    }
}

pub type Result<T> = std::result::Result<T, Error>;

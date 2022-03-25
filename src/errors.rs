#[derive(Debug)]
pub enum SteelseriesError {
    NoDevice,
    Privileges,
    InvalidCapability,
    ClaimInterface(u8),
    // ReleaseInterface(u8),
    Usb(hidapi::HidError),
}

impl From<hidapi::HidError> for SteelseriesError {
    fn from(original: hidapi::HidError) -> Self {
        SteelseriesError::Usb(original)
    }
}

pub type SteelseriesResult<T> = std::result::Result<T, SteelseriesError>;

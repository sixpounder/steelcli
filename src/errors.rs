#[derive(Debug)]
pub enum SteelseriesError {
    Privileges,
    Usb(rusb::Error)
}

impl From<rusb::Error> for SteelseriesError {
    fn from(original: rusb::Error) -> Self {
        SteelseriesError::Usb(original)
    }
}

pub type SteelseriesResult<T> = std::result::Result<T, SteelseriesError>;
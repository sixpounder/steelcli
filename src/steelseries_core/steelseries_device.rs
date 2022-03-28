use std::{collections::HashMap};
use std::fmt::Display;

use hidapi::{HidError};

use super::{Result, Error};

pub trait FromCode {
    fn from_code(code: &str) -> Self;
}

pub trait ToCode {
    fn to_code(&self) -> &str;
}

pub trait ToDescription {
    fn to_description(&self) -> &str;
}

/// Represents a device property that can be queried and/or manipulated
#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum DeviceProperty {
    LeftHeadphoneLedColor,
    RightHeadphoneLedColor,
    HeadphonesColor,
    WheelLedColor,
    LedColor,
}

impl FromCode for DeviceProperty {
    fn from_code(code: &str) -> Self {
        match code {
            "lhc" => DeviceProperty::LeftHeadphoneLedColor,
            "rhc" => DeviceProperty::RightHeadphoneLedColor,
            "hc" => DeviceProperty::HeadphonesColor,
            "lc" => DeviceProperty::LedColor,
            _ => DeviceProperty::LedColor,
        }
    }
}

impl ToCode for DeviceProperty {
    fn to_code(&self) -> &str {
        match self {
            DeviceProperty::LeftHeadphoneLedColor => "lhc",
            DeviceProperty::RightHeadphoneLedColor => "rhc",
            DeviceProperty::HeadphonesColor => "hc",
            DeviceProperty::LedColor => "lc",
            DeviceProperty::WheelLedColor => "wc",
        }
    }
}

impl ToDescription for DeviceProperty {
    fn to_description(&self) -> &str {
        match self {
            DeviceProperty::LeftHeadphoneLedColor => "Left headphone led color",
            DeviceProperty::RightHeadphoneLedColor => "Right headphone led color",
            DeviceProperty::HeadphonesColor => "Headphone leds color",
            DeviceProperty::LedColor => "Led color",
            DeviceProperty::WheelLedColor => "Wheel led color",
        }
    }
}

impl Display for DeviceProperty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_description())
    }
}

impl<'a> From<&'a str> for DeviceProperty {
    fn from(code: &'a str) -> Self {
        DeviceProperty::from_code(code)
    }
}

/// An handle over a single device. Tipically obtained by calling `open` on a `SteelSeriesDevice` implementor.
/// Use to read and write data to the device.
pub struct SteelseriesDeviceHandle {
    _info: hidapi::DeviceInfo,
    pub(crate) handle: hidapi::HidDevice,
}

impl SteelseriesDeviceHandle {
    pub fn new(device_info: hidapi::DeviceInfo) -> Self {
        let api = &crate::HIDAPI;
        let device_handle = device_info
            .open_device(&api)
            .expect("Could not open device");
        Self {
            _info: device_info,
            handle: device_handle,
        }
    }

    #[allow(dead_code)]
    pub fn write(&self, buf: &[u8]) -> Result<usize> {
        match self.handle.write(buf) {
            Ok(written) => Ok(written),
            Err(some_error) => Err(Error::Usb(some_error))
        }
    }

    pub fn send_feature_report(&self, buf: &[u8]) -> Result<()> {
        match self.handle.send_feature_report(buf) {
            Ok(_) => Ok(()),
            Err(some_error) => Err(Error::Usb(some_error))
        }
    }
}

impl<T> From<T> for SteelseriesDeviceHandle where T: SteelseriesDevice {
    fn from(source: T) -> Self {
        let api = &crate::HIDAPI;
        let device_info = api.device_list().find(|d| source.matches(d.vendor_id(), d.product_id())).unwrap().clone();
        device_info.open().unwrap()
    }
}

/// Each device has one or more profile specific values that need to be stored
/// for later usage. These are the types that are storable.
#[allow(dead_code)]
pub enum DeviceProfileValue {
    Str(&'static str),
    Byte(u8),
    Hex(u16),
    ByteList(&'static [u8]),
}

impl DeviceProfileValue {
    #[allow(dead_code)]
    pub(crate) fn as_str(&self) -> Option<&'static str> {
        if let DeviceProfileValue::Str(value) = self {
            Some(value)
        } else {
            None
        }
    }

    pub(crate) fn as_byte(&self) -> Option<&u8> {
        if let DeviceProfileValue::Byte(value) = self {
            Some(value)
        } else {
            None
        }
    }

    pub(crate) fn as_hex(&self) -> Option<&u16> {
        if let DeviceProfileValue::Hex(value) = self {
            Some(value)
        } else {
            None
        }
    }

    pub(crate) fn as_byte_list(&self) -> Option<&[u8]> {
        if let DeviceProfileValue::ByteList(value) = self {
            Some(value)
        } else {
            None
        }
    }
}

/// A trait implemented by structures designed to represent a single device
pub trait SteelseriesDevice {

    /// Enumerates this devices capabilities and configurable properties
    fn enumerate_capabilities(&self) -> std::slice::Iter<DeviceProperty>;

    /// Extended device name
    fn get_name(&self) -> &str;

    /// A short name to identify the device
    fn get_slug(&self) -> &str;

    /// Changes a single property of this device. Implementation varies per device.
    fn change_property(&self, property: DeviceProperty, value: &str) -> Result<()>;

    /// The device vendor id
    fn get_vendor_id(&self) -> u16;

    /// The device product id
    fn get_product_id(&self) -> u16;

    /// Returns a map of profile values for this device
    fn get_profile(&self) -> Option<&HashMap<&str, DeviceProfileValue>>;

    /// If `capability` is contained in the `enumerate_capabilities` iterators returns `Some(capability)`,
    /// `None` otherwise
    fn supports_capability(&self, capability: DeviceProperty) -> Option<&DeviceProperty> {
        self.enumerate_capabilities().find(|c| **c == capability)
    }

    /// Opens the device and returns an handle to it
    fn open(&self) -> Result<SteelseriesDeviceHandle> {
        let api = &crate::HIDAPI;
        let dev = api.device_list().find(|d| {
            d.vendor_id() == self.get_vendor_id() && d.product_id() == self.get_product_id()
        });

        match dev {
            Some(connected_device) => Ok(SteelseriesDeviceHandle::new(connected_device.clone())),
            None => Err(Error::Usb(HidError::HidApiErrorEmpty)),
        }
    }

    /// Gets a value from this device profile, if present
    fn get_profile_value(&self, key: &str) -> Option<&DeviceProfileValue> {
        match self.get_profile() {
            Some(settings_collection) => settings_collection.get(key),
            None => None,
        }
    }

    /// Checks if the device mathches a given `vendor_id:product_id` combination
    fn matches(&self, vendor_id: u16, product_id: u16) -> bool {
        self.get_vendor_id() == vendor_id && self.get_product_id() == product_id
    }
}

impl SteelseriesDevice for hidapi::DeviceInfo {
    fn enumerate_capabilities(&self) -> std::slice::Iter<DeviceProperty> {
        panic!("Cannot enumerate capabilities on raw device info. You shouldn't have called this.")
    }

    fn get_name(&self) -> &str {
        self.product_string().unwrap_or("???")
    }

    fn get_slug(&self) -> &str {
        self.get_name()
    }

    fn change_property(&self, _property: DeviceProperty, _value: &str) -> Result<()> {
        Ok(())
    }

    fn get_vendor_id(&self) -> u16 {
        self.vendor_id()
    }

    fn get_product_id(&self) -> u16 {
        self.product_id()
    }

    fn get_profile(&self) -> Option<&HashMap<&str, DeviceProfileValue>> {
        None
    }
}

impl<T> ToDescription for T
where
    T: SteelseriesDevice,
{
    fn to_description(&self) -> &str {
        self.get_name()
    }
}

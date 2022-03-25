use std::collections::HashMap;
use std::fmt::Display;

use hidapi::{HidError};

use crate::errors::{SteelseriesError, SteelseriesResult};

pub trait FromCode {
    fn from_code(code: &str) -> Self;
}

pub trait ToCode {
    fn to_code(&self) -> &str;
}

pub trait ToDescription {
    fn to_description(&self) -> &str;
}

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

pub struct SteelseriesDeviceHandle {
    info: hidapi::DeviceInfo,
    pub(crate) handle: hidapi::HidDevice,
}

impl SteelseriesDeviceHandle {
    pub fn new(device_info: hidapi::DeviceInfo) -> Self {
        let api = &crate::HIDAPI;
        let device_handle = device_info
            .open_device(&api)
            .expect("Could not open device");
        Self {
            info: device_info,
            handle: device_handle,
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

#[allow(dead_code)]
pub enum DeviceProfileValue {
    Str(&'static str),
    Byte(u8),
    Hex(u16),
    ByteList(&'static [u8]),
}

pub trait SteelseriesDevice {
    fn enumerate_capabilities(&self) -> std::slice::Iter<DeviceProperty>;
    fn get_name(&self) -> &str;
    fn get_slug(&self) -> &str;
    fn change_property(&self, property: DeviceProperty, value: &str) -> SteelseriesResult<()>;
    fn get_vendor_id(&self) -> u16;
    fn get_product_id(&self) -> u16;
    fn get_profile(&self) -> Option<&HashMap<&str, DeviceProfileValue>>;

    fn open(&self) -> Result<SteelseriesDeviceHandle, SteelseriesError> {
        let api = &crate::HIDAPI;
        let dev = api.device_list().find(|d| {
            d.vendor_id() == self.get_vendor_id() && d.product_id() == self.get_product_id()
        });

        match dev {
            Some(connected_device) => Ok(SteelseriesDeviceHandle::new(connected_device.clone())),
            None => Err(SteelseriesError::Usb(HidError::HidApiErrorEmpty)),
        }
    }

    fn get_profile_value(&self, key: &str) -> Option<&DeviceProfileValue> {
        match self.get_profile() {
            Some(settings_collection) => settings_collection.get(key),
            None => None,
        }
    }

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

    fn change_property(&self, _property: DeviceProperty, _value: &str) -> SteelseriesResult<()> {
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

use rusb::{Context, Device, DeviceHandle, UsbContext};
use crate::LOGGER;
use crate::errors::SteelseriesResult;

pub struct DeviceCapability<'a> {
    pub label: &'a str,
    pub description: &'a str,
}

impl<'a> From<(&'a str, &'a str)> for DeviceCapability<'a> {
    fn from(tuple: (&'a str, &'a str)) -> Self {
        DeviceCapability {
            label: tuple.0,
            description: tuple.1,
        }
    }
}

pub trait SteelseriesDevice {
    fn enumerate_capabilities(&self) -> std::slice::Iter<DeviceCapability>;
    fn get_name(&self) -> &str;
    fn change_property(&self, property: &str, value: &str) -> SteelseriesResult<()>;
    fn get_vendor_id(&self) -> u16;
    fn get_product_id(&self) -> u16;

    fn matches(&self, vendor_id: u16, product_id: u16) -> bool {
        self.get_vendor_id() == vendor_id && self.get_product_id() == product_id
    }

    fn open_device(&self) -> Option<(Device<Context>, DeviceHandle<Context>)> {
        let context = Context::new().unwrap();
        let devices = match context.devices() {
            Ok(d) => d,
            Err(e) => {
                LOGGER.error(format!("{}", e).as_str());
                return None;
            }
        };

        for device in devices.iter() {
            let device_desc = match device.device_descriptor() {
                Ok(d) => d,
                Err(_) => continue,
            };

            if device_desc.vendor_id() == self.get_vendor_id()
                && device_desc.product_id() == self.get_product_id()
            {
                match device.open() {
                    Ok(handle) => return Some((device, handle)),
                    Err(e) => {
                        LOGGER.warn(format!("{}", e).as_str());
                        continue;
                    }
                }
            }
        }

        None
    }
}

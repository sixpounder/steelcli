use rusb::{Context, UsbContext, Device, DeviceHandle};

use super::arctis;

pub struct DeviceCapability {
    pub label: String,
    pub description: String,
}

pub trait SteelseriesDevice {
    fn matches(&self, vendor_id: u16, product_id: u16) -> bool;
    fn enumerate_capabilities(&self) -> std::slice::Iter<DeviceCapability>;
    fn get_name(&self) -> &str;
    fn change_property(&self, property: &str, value: &str) -> bool;
    fn get_vendor_id(&self) -> u16;
    fn get_product_id(&self) -> u16;
    fn open_device(&self) -> Option<(Device<Context>, DeviceHandle<Context>)> {
        let context = Context::new().unwrap();
        let devices = match context.devices() {
            Ok(d) => d,
            Err(e) => {
                println!("{}", e);
                return None;
            }
        };

        for device in devices.iter() {
            let device_desc = match device.device_descriptor() {
                Ok(d) => d,
                Err(_) => continue,
            };

            if device_desc.vendor_id() == self.get_vendor_id() && device_desc.product_id() == self.get_product_id() {
                match device.open() {
                    Ok(handle) => return Some((device, handle)),
                    Err(e) => {
                        println!("{}", e);
                        continue;
                    }
                }
            }
        }

        None
    }
}

pub struct DevicePool {
    devices: Vec<Box<dyn SteelseriesDevice>>,
}

impl DevicePool {
    pub fn new() -> Self {
        let mut devices = vec![];

        let arctis5: Box<dyn SteelseriesDevice> = Box::new(arctis::Arctis5Headphones::new());
        devices.push(arctis5);

        Self { devices }
    }

    pub fn find_one(&self, vendor_id: u16, product_id: u16) -> Option<&dyn SteelseriesDevice> {
        let mut ret: Option<&dyn SteelseriesDevice> = None;
        self.devices.iter().for_each(|f| {
            if f.matches(vendor_id, product_id) {
                ret = Some(&**f);
            }
        });

        ret
    }
}

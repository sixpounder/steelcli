use rusb::DeviceHandle;
use rusb::Device;
use rusb::UsbContext;
use colored::*;

#[derive(Debug)]
pub struct Endpoint {
    config: u8,
    iface: u8,
    setting: u8,
    address: u8,
    number: u8,
}

pub fn open_device<T: UsbContext>(
    context: &mut T,
    vendor_id: u16,
    product_id: u16,
) -> Option<(Device<T>, DeviceHandle<T>)> {
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

        if device_desc.vendor_id() == vendor_id && device_desc.product_id() == product_id {
            match device.open() {
                Ok(handle) => return Some((device, handle)),
                Err(e) => {
                    match e {
                        rusb::Error::Access => {
                            crate::OUTPUT.error(
                                format!("Access denied. Try running with {} or use the --escalate argument.", "sudo".bold()).as_str()
                            );
                        }
                        _ => {
                            crate::OUTPUT.error(format!("{}", e).as_str());
                        }
                    }
                    continue
                },
            }
        }
    }
    None
}

/// Returns all readable endpoints for given usb device
pub fn find_readable_endpoints<T: UsbContext>(device: &mut Device<T>) -> rusb::Result<Vec<Endpoint>> {
    let device_desc = device.device_descriptor()?;
    let mut endpoints = vec![];
    for n in 0..device_desc.num_configurations() {
        let config_desc = match device.config_descriptor(n) {
            Ok(c) => c,
            Err(_) => continue,
        };
        // println!("{:#?}", config_desc);
        for interface in config_desc.interfaces() {
            for interface_desc in interface.descriptors() {
                // println!("{:#?}", interface_desc);
                for endpoint_desc in interface_desc.endpoint_descriptors() {
                    // println!("{:#?}", endpoint_desc);
                    endpoints.push(Endpoint {
                        config: config_desc.number(),
                        iface: interface_desc.interface_number(),
                        setting: interface_desc.setting_number(),
                        address: endpoint_desc.address(),
                        number: endpoint_desc.number(),
                    });
                }
            }
        }
    }

    Ok(endpoints)
}

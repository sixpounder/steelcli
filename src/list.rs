use crate::device_ops::open_device;
use crate::device_ops::{supported_devices, find_readable_endpoints};
use rusb::DeviceHandle;
use rusb::UsbContext;
use rusb::{Context, Result};
use std::time::Duration;

pub fn list() -> Result<()> {
    let mut context = Context::new()?;
    for (vendor_id, product_id, _name) in supported_devices().iter() {
        let (mut device, mut handle) =
            open_device(&mut context, *vendor_id, *product_id).expect("Failed to open device");
        print_device_info(&mut handle)?;
        match find_readable_endpoints(&mut device) {
            Ok(endpoints) => {
                for e in endpoints.iter() {
                    println!("\t{:?}", e);
                }
            },
            Err(_) => ()
        }
        println!("--------------------------");
        println!("\n");
    }

    Ok(())
}

fn print_device_info<T: UsbContext>(handle: &mut DeviceHandle<T>) -> Result<()> {
    let device_desc = handle.device().device_descriptor()?;
    let timeout = Duration::from_secs(1);
    let languages = handle.read_languages(timeout)?;

    println!(
        "Device ID: {}",
        device_desc.product_id()
    );

    println!(
        "Vendor ID: {}",
        device_desc.vendor_id()
    );

    if !languages.is_empty() {
        let language = languages[0];

        println!(
            "Manufacturer: {}",
            handle
                .read_manufacturer_string(language, &device_desc, timeout)
                .unwrap_or("Not Found".to_string())
        );
        println!(
            "Product: {}",
            handle
                .read_product_string(language, &device_desc, timeout)
                .unwrap_or("Not Found".to_string())
        );
        println!(
            "Serial Number: {}",
            handle
                .read_serial_number_string(language, &device_desc, timeout)
                .unwrap_or("Not Found".to_string())
        );

        println!("Language: {:?}", language);
    }

    println!("Active configuration: {}", handle.active_configuration()?);

    Ok(())
}

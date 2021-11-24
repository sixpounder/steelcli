use crate::{device_ops::open_device, errors::SteelseriesResult, steelseries_core::support::supported_devices};
use crate::device_ops::find_readable_endpoints;
use crate::utils;
use rusb::DeviceHandle;
use rusb::UsbContext;
use rusb::{Context, Result};
use std::time::Duration;
use colored::*;
use crate::OUTPUT;

pub fn list() -> SteelseriesResult<()> {
    let mut context = Context::new()?;
    let mut first = true;
    for (vendor_id, product_id, _name) in supported_devices().iter() {
        let open_result = open_device(&mut context, *vendor_id, *product_id);
        match open_result {
            Some((mut device, mut handle)) => {
                if !first {
                    println!("|");
                    println!("|");
                    println!("|");
                    println!("|");
                } else {
                    first = false;
                }
                print_device_info(&mut handle)?;
                match find_readable_endpoints(&mut device) {
                    Ok(endpoints) => {
                        println!("{}:", "Endpoints".cyan());
                        for e in endpoints.iter() {
                            println!("  ├ {:?}", e);
                        }
                    },
                    Err(_) => {
                        OUTPUT.warn("No endpoints found for this device");
                    }
                }
            },
            _ => () // Device not connected?
        }
    }

    Ok(())
}

fn print_device_info<T: UsbContext>(handle: &mut DeviceHandle<T>) -> Result<()> {
    let device_desc = handle.device().device_descriptor()?;
    let timeout = Duration::from_secs(1);
    let languages = handle.read_languages(timeout)?;

    println!(
        "{}: {}",
        "Vendor ID".cyan(),
        utils::format_radix(device_desc.vendor_id() as u32, 16)
    );

    println!(
        "{}: {}",
        "Device ID".cyan(),
        utils::format_radix(device_desc.product_id() as u32, 16)
    );

    if !languages.is_empty() {
        let language = languages[0];

        println!(
            "{}: {}",
            "Manufacturer".cyan(),
            handle
                .read_manufacturer_string(language, &device_desc, timeout)
                .unwrap_or("Not Found".to_string())
        );
        println!(
            "{}: {}",
            "Product".cyan(),
            handle
                .read_product_string(language, &device_desc, timeout)
                .unwrap_or("Not Found".to_string())
        );
        println!(
            "{}: {}",
            "Serial Number".cyan(),
            handle
                .read_serial_number_string(language, &device_desc, timeout)
                .unwrap_or("Not Found".to_string())
        );

        println!("{}: {:?}", "Primary language".cyan(), language.primary_language());
    }

    println!("{}: {}", "Active configuration".cyan(), handle.active_configuration()?);

    Ok(())
}

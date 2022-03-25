use crate::errors::SteelseriesResult;
use crate::steelseries_core::support::DevicePool;
use crate::steelseries_core::{SteelseriesDevice};
use crate::{utils};
use colored::*;

pub fn list() -> SteelseriesResult<()> {
    for device in DevicePool::new().sync().devices.iter() {
        print_device_info(&**device).unwrap();
    }

    Ok(())
}

fn print_device_info(handle: &dyn SteelseriesDevice) -> SteelseriesResult<()> {
    let device_desc = handle;

    println!(
        "{}: {}",
        "Vendor ID".cyan(),
        utils::format_radix(device_desc.get_vendor_id() as u32, 16)
    );

    println!(
        "{}: {}",
        "Device ID".cyan(),
        utils::format_radix(device_desc.get_product_id() as u32, 16)
    );

    println!("{}: {}", "Product".cyan(), handle.get_name());

    Ok(())
}

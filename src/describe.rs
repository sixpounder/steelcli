use colored::Colorize;

use crate::steelseries_core::{Result, SteelseriesDevice, ToCode, ToDescription};
use crate::utils;

pub fn describe(device: &dyn SteelseriesDevice) -> Result<()> {
    print_device_info(device);

    for c in device.enumerate_capabilities() {
        println!("  Attributes:");
        println!("    {}\t{}", c.to_code().bold(), c.to_description());
    }

    Ok(())
}

pub(crate) fn print_device_info(handle: &dyn SteelseriesDevice) {
    println!("{}", handle.get_name().cyan().bold());

    println!("  {}: {}", "Nick", handle.get_slug());

    println!(
        "  {}: {}",
        "Vendor ID",
        utils::format_radix(handle.get_vendor_id() as u32, 16)
    );

    println!(
        "  {}: {}",
        "Device ID",
        utils::format_radix(handle.get_product_id() as u32, 16)
    );
}

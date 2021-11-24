use crate::{errors::SteelseriesResult, steelseries_core::support::DevicePool};
use crate::OUTPUT;

pub fn describe(vendor_id: u16, product_id: u16) -> SteelseriesResult<()> {
    let pool = DevicePool::new();

    let device = pool
        .find_one(vendor_id, product_id)
        .expect("Device not supported");

    for c in device.enumerate_capabilities() {
        OUTPUT.log(format!("{} - {}", c.name, c.description).as_str());
    }
    
    Ok(())
}

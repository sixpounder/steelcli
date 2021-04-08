use crate::{errors::SteelseriesResult, steelseries_core::support::DevicePool};
use crate::LOGGER;

pub fn describe(vendor_id: u16, product_id: u16) -> SteelseriesResult<()> {
    let pool = DevicePool::new();

    let device = pool
        .find_one(vendor_id, product_id)
        .expect("Device not supported");

    for c in device.enumerate_capabilities() {
        LOGGER.log(format!("{} - {}", c.label, c.description).as_str());
    }
    
    Ok(())
}

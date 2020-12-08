use crate::{device_ops::open_device, devices::DevicePool};
use rusb::{Context, Result};

pub fn change(vendor_id: u16, product_id: u16, prop: &str, value: &str) -> Result<()> {
    let pool = DevicePool::new();

    pool.find_one(vendor_id, product_id).unwrap().change_property(prop, value);

    Ok(())
}

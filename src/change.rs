use rusb::{Result};

use crate::device_ops::DevicePool;

pub fn change(vendor_id: u16, product_id: u16, prop: &str, value: &str) -> Result<()> {
    let pool = DevicePool::new();

    pool.find_one(vendor_id, product_id).unwrap().change_property(prop, value)
}

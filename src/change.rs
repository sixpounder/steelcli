use crate::{device_ops::DevicePool, errors::SteelseriesResult};

pub fn change(vendor_id: u16, product_id: u16, prop: &str, value: &str) -> SteelseriesResult<()> {
    let pool = DevicePool::new();

    pool.find_one(vendor_id, product_id).unwrap().change_property(prop, value)
}

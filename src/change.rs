use crate::{errors::SteelseriesResult, steelseries_core::support::DevicePool};

pub fn change(vendor_id: u16, product_id: u16, prop: &str, value: &str) -> SteelseriesResult<()> {
    let pool = DevicePool::new();

    pool.find_one(vendor_id, product_id)
        .expect("Could not find device")
        .change_property(prop, value)
}

use crate::describe::print_device_info;
use crate::errors::SteelseriesResult;
use crate::steelseries_core::support::DevicePool;

pub fn list(pool: &DevicePool) -> SteelseriesResult<()> {
    for device in pool.devices.iter() {
        print_device_info(&**device);
    }

    Ok(())
}

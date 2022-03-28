use crate::describe::print_device_info;
use crate::steelseries_core::support::DevicePool;
use crate::steelseries_core::Result;

pub fn list(pool: &DevicePool) -> Result<()> {
    for device in pool.devices.iter() {
        print_device_info(&**device);
    }

    Ok(())
}

pub fn list_all() -> Result<()> {
    for device in DevicePool::new().devices {
        print_device_info(&*device);
    }

    Ok(())
}

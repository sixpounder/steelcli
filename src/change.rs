use crate::{errors::SteelseriesResult, steelseries_core::{DeviceProperty, FromCode, SteelseriesDevice}};

pub fn change(device: &dyn SteelseriesDevice, prop: &str, value: &str) -> SteelseriesResult<()> {
    device.change_property(DeviceProperty::from_code(prop), value)
}

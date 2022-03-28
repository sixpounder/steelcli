use crate::steelseries_core::{
    Result, {DeviceProperty, FromCode, SteelseriesDevice},
};

pub fn change(device: &dyn SteelseriesDevice, prop: &str, value: &str) -> Result<()> {
    device.change_property(DeviceProperty::from_code(prop), value)
}

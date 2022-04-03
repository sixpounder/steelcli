use crate::steelseries_core::{
    Result, {DeviceProperty, FromCode, SteelseriesDevice}, TaskOptions,
};

pub fn change(device: &dyn SteelseriesDevice, prop: &str, value: &str, options: &TaskOptions) -> Result<()> {
    device.change_property(DeviceProperty::from_code(prop), value, options)
}

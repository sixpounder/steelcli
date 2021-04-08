use crate::errors::SteelseriesResult;
use crate::{errors::SteelseriesError, LOGGER};
use rusb::{Context, Device, DeviceHandle, UsbContext};

pub struct DeviceCapability<'a> {
    pub label: &'a str,
    pub description: &'a str,
}

impl<'a> From<(&'a str, &'a str)> for DeviceCapability<'a> {
    fn from(tuple: (&'a str, &'a str)) -> Self {
        DeviceCapability {
            label: tuple.0,
            description: tuple.1,
        }
    }
}

pub struct SteelseriesDeviceHandle {
    _device: Device<Context>,
    usb_handle: DeviceHandle<Context>,
    claimed_interfaces: Vec<u8>,
}

impl SteelseriesDeviceHandle {
    pub fn new(device: Device<Context>, handle: DeviceHandle<Context>) -> Self {
        Self {
            _device: device,
            usb_handle: handle,
            claimed_interfaces: vec![],
        }
    }

    pub fn _usb_device(&self) -> &Device<Context> {
        &self._device
    }

    pub fn usb_handle(&self) -> &DeviceHandle<Context> {
        &self.usb_handle
    }

    pub fn claim_interface(&mut self, iface: u8) -> SteelseriesResult<()> {
        self.usb_handle
            .set_auto_detach_kernel_driver(false)
            .expect("Could not set detach kernel driver mode");

        match self.usb_handle.kernel_driver_active(iface) {
            Ok(has_kernel_driver) => {
                if has_kernel_driver {
                    self.usb_handle
                        .detach_kernel_driver(iface)
                        .expect("Could not detach kernel driver");
                }
                self.claimed_interfaces.push(iface);
                Ok(())
            }
            Err(_e) => Err(SteelseriesError::ClaimInterface(iface)),
        }
    }

    pub fn release_interface(&mut self, iface: u8) -> SteelseriesResult<()> {
        match self.claimed_interfaces.iter().find(|i| **i == iface) {
            Some(claimed_interface) => {
                match self.usb_handle.release_interface(*claimed_interface) {
                    Ok(_) => match self.usb_handle.kernel_driver_active(*claimed_interface) {
                        Ok(has_kernel_driver) => {
                            if has_kernel_driver {
                                self.usb_handle
                                    .attach_kernel_driver(*claimed_interface)
                                    .expect("Could not reattach kernel driver");
                            }
                            Ok(())
                        }
                        Err(_e) => {
                            LOGGER.warn(
                                format!(
                                    "Could not reattach kernel driver to interface {}",
                                    claimed_interface
                                )
                                .as_str(),
                            );
                            return Err(SteelseriesError::ReleaseInterface(iface));
                        }
                    },
                    Err(_) => {
                        return Err(SteelseriesError::ReleaseInterface(iface));
                    }
                }
            }
            None => Ok(()),
        }
    }
}

impl Drop for SteelseriesDeviceHandle {
    fn drop(&mut self) {
        let claimed_interfaces = self.claimed_interfaces.iter();
        for claimed_interface in claimed_interfaces {
            match self.usb_handle.release_interface(*claimed_interface) {
                Ok(_) => match self.usb_handle.kernel_driver_active(*claimed_interface) {
                    Ok(has_kernel_driver) => {
                        if has_kernel_driver {
                            self.usb_handle
                                .attach_kernel_driver(*claimed_interface)
                                .expect(
                                    format!(
                                        "Could not reattach kernel driver for interface {}",
                                        claimed_interface
                                    )
                                    .as_str(),
                                );
                        }
                    }
                    Err(_e) => {
                        LOGGER.warn(
                            format!(
                                "Could not reattach kernel driver to interface {}",
                                claimed_interface
                            )
                            .as_str(),
                        );
                    }
                },
                Err(_) => (),
            }
        }
    }
}

pub trait SteelseriesDevice {
    fn enumerate_capabilities(&self) -> std::slice::Iter<DeviceCapability>;
    fn get_name(&self) -> &str;
    fn change_property(&self, property: &str, value: &str) -> SteelseriesResult<()>;
    fn get_vendor_id(&self) -> u16;
    fn get_product_id(&self) -> u16;

    fn matches(&self, vendor_id: u16, product_id: u16) -> bool {
        self.get_vendor_id() == vendor_id && self.get_product_id() == product_id
    }

    fn open_device(&self) -> Option<SteelseriesDeviceHandle> {
        let context = Context::new().unwrap();
        let devices = match context.devices() {
            Ok(d) => d,
            Err(e) => {
                LOGGER.error(format!("{}", e).as_str());
                return None;
            }
        };

        for device in devices.iter() {
            let device_desc = match device.device_descriptor() {
                Ok(d) => d,
                Err(_) => continue,
            };

            if device_desc.vendor_id() == self.get_vendor_id()
                && device_desc.product_id() == self.get_product_id()
            {
                match device.open() {
                    Ok(handle) => {
                        return Some(SteelseriesDeviceHandle::new(device, handle));
                    }
                    Err(e) => {
                        LOGGER.warn(format!("{}", e).as_str());
                        continue;
                    }
                }
            }
        }

        None
    }
}

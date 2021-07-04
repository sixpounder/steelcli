use crate::errors::SteelseriesResult;
use crate::{errors::SteelseriesError, OUTPUT};
use rusb::{Context, Device, DeviceHandle, UsbContext};

use super::DeviceOperation;

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
                            OUTPUT.warn(
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

    pub fn process_commands(&self, device_ops: Vec<DeviceOperation>) -> SteelseriesResult<()> {
        for usb_comm_operation in device_ops.iter() {
            match usb_comm_operation {
                DeviceOperation::WriteControl(payload) => {
                    match self.usb_handle().write_control(
                        payload.request_type,
                        payload.request,
                        payload.value,
                        payload.index,
                        &payload.buf,
                        payload.timeout,
                    ) {
                        Ok(size) => {
                            OUTPUT.verbose(format!("URB_CONTROL out {} bytes -> Device", size).as_str());
    
                            if let Some(m) = payload.debug_message.as_ref() {
                                OUTPUT.verbose(m);
                            }
                        }
                        Err(e) => {
                            OUTPUT.error(format!("Error: {}", e).as_str());
                        }
                    }
                },
                DeviceOperation::ReadControl(payload) => {
                    let mut response_buf = vec![];
                    match self.usb_handle().read_control(
                        payload.request_type,
                        payload.request,
                        payload.value,
                        payload.index,
                        &mut response_buf,
                        payload.timeout,
                    ) {
                        Ok(n) => {
                            OUTPUT.verbose(format!("Host <- URB_CONTROL out {} bytes", n).as_str());
                        },
                        Err(read_error) => {
                            OUTPUT.error(format!("Host <- URB_CONTROL out").as_str());
                            return Err(SteelseriesError::Usb(read_error));
                        }
                    }
                }
                DeviceOperation::WriteInterrupt(endpoint, buf) => {
                    match self.usb_handle().write_interrupt(*endpoint, buf, std::time::Duration::from_millis(50)) {
                        Ok(n_bytes) => {
                            OUTPUT.verbose(format!("Device Interrupt <- {} bytes", n_bytes).as_str());
                        },
                        Err(some_error) => {
                            // println!("{:?}", _some_error);
                            OUTPUT.error("Interrupt error");
                            return Err(SteelseriesError::Usb(some_error));
                        }
                    }
                },
                DeviceOperation::ReadInterrupt(endpoint) => {
                    let mut buf = vec![];
                    match self.usb_handle().read_interrupt(*endpoint, &mut buf, std::time::Duration::from_millis(50)) {
                        Ok(_bytes_read) => {
                            OUTPUT.verbose("Interrupt IN with");
                        },
                        Err(interrupt_error) => {
                            OUTPUT.error("Interrupt error");
                            // return Err(SteelseriesError::Usb(interrupt_error));
                            return Err(SteelseriesError::Usb(interrupt_error));
                        }
                    }
                }
            }
        }
    
        Ok(())
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
                        OUTPUT.warn(
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
                OUTPUT.error(format!("{}", e).as_str());
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
                        OUTPUT.warn(format!("{}", e).as_str());
                        continue;
                    }
                }
            }
        }

        None
    }
}

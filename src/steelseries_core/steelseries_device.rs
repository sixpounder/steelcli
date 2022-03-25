use std::fmt::Display;

use crate::errors::SteelseriesResult;
use crate::{errors::SteelseriesError, OUTPUT};
use rusb::{Context, Device, DeviceHandle, UsbContext};

use super::DeviceOperation;

pub trait FromCode {
    fn from_code(code: &str) -> Self;
}

pub trait ToCode {
    fn to_code(&self) -> &str;
}

pub trait ToDescription {
    fn to_description(&self) -> &str ;
}

#[derive(PartialEq)]
pub enum DeviceCapability {
    LeftHeadphoneLedColor,
    RightHeadphoneLedColor,
    HeadphonesColor,
    LedColor
}

impl FromCode for DeviceCapability {
    fn from_code(code: &str) -> Self {
        match code {
            "lhc" => DeviceCapability::LeftHeadphoneLedColor,
            "rhc" => DeviceCapability::RightHeadphoneLedColor,
            "hc" => DeviceCapability::HeadphonesColor,
            "lc" => DeviceCapability::LedColor,
            _ => DeviceCapability::LedColor,
        }
    }
}

impl ToCode for DeviceCapability {
    fn to_code(&self) -> &str {
        match self {
            DeviceCapability::LeftHeadphoneLedColor => "lhc",
            DeviceCapability::RightHeadphoneLedColor => "rhc",
            DeviceCapability::HeadphonesColor => "hc",
            DeviceCapability::LedColor => "lc",
        }
    }
}

impl ToDescription for DeviceCapability {
    fn to_description(&self) -> &str  {
        match self {
            DeviceCapability::LeftHeadphoneLedColor => "Left headphone led color",
            DeviceCapability::RightHeadphoneLedColor => "Right headphone led color",
            DeviceCapability::HeadphonesColor => "Headphone leds color",
            DeviceCapability::LedColor => "Led color",
        }
    }
}

impl Display for DeviceCapability {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_description())
    }
}

impl<'a> From<&'a str> for DeviceCapability {
    fn from(code: &'a str) -> Self {
        DeviceCapability::from_code(code)
    }
}

pub struct SteelseriesDeviceHandle {
    _device: Device<Context>,
    usb_handle: DeviceHandle<Context>,
    claimed_interfaces: Vec<u8>,
}

impl SteelseriesDeviceHandle {
    pub fn new(device: Device<Context>, handle: DeviceHandle<Context>) -> Self {
        let mut instance = Self {
            _device: device,
            usb_handle: handle,
            claimed_interfaces: vec![],
        };

        instance
            .usb_handle
            .set_auto_detach_kernel_driver(true)
            .expect("Could not set detach kernel driver mode");

        instance
    }

    pub fn _usb_device(&self) -> &Device<Context> {
        &self._device
    }

    pub fn usb_handle(&self) -> &DeviceHandle<Context> {
        &self.usb_handle
    }

    pub fn claim_interface(&mut self, iface: u8) -> SteelseriesResult<()> {
        match self.usb_handle.claim_interface(iface) {
            Ok(()) => {
                self.claimed_interfaces.push(iface);
                Ok(())
            }
            Err(e) => {
                OUTPUT.error(format!("Error: {}", e).as_str());
                Err(SteelseriesError::ClaimInterface(iface))
            }
        }
    }

    pub fn process_commands(&self, device_ops: Vec<DeviceOperation>, delay: Option<std::time::Duration>) -> SteelseriesResult<()> {
        for usb_comm_operation in device_ops.iter() {
            match delay.as_ref() {
                Some(time) => std::thread::sleep(*time),
                None => (),
            }

            match usb_comm_operation {
                DeviceOperation::WriteControl(payload) => {
                    let default_payload: Vec<u8> = vec![];
                    let buf = payload.buf.as_ref().unwrap_or(&default_payload).as_slice();
                    match self.usb_handle().write_control(
                        payload.request_type,
                        payload.request,
                        payload.value,
                        payload.index,
                        buf,
                        payload.timeout,
                    ) {
                        Ok(size) => {
                            OUTPUT.verbose(
                                format!("Host -> Device - URB_CONTROL out {} bytes", size).as_str(),
                            );

                            if let Some(m) = payload.debug_message.as_ref() {
                                OUTPUT.verbose(m);
                            }
                        }
                        Err(e) => {
                            OUTPUT.error(format!("Error: {}", e).as_str());
                        }
                    }
                }
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
                            OUTPUT.verbose(
                                format!("Device -> Host - URB_CONTROL out ({} bytes)", n).as_str(),
                            );
                        }
                        Err(read_error) => {
                            OUTPUT.error(format!("Device -> Host - URB_CONTROL out").as_str());
                            return Err(SteelseriesError::Usb(read_error));
                        }
                    }
                }
                DeviceOperation::WriteInterrupt(endpoint, buf) => {
                    match self.usb_handle().write_interrupt(
                        *endpoint,
                        buf,
                        std::time::Duration::from_millis(500),
                    ) {
                        Ok(n_bytes) => {
                            OUTPUT.verbose(
                                format!("Host -> Device - Interrupt ({} bytes)", n_bytes).as_str(),
                            );
                        }
                        Err(interrupt_error) => {
                            // println!("{:?}", _some_error);
                            OUTPUT.error(
                                format!("Host -> Device - Interrupt - {}", interrupt_error)
                                    .as_str(),
                            );
                            return Err(SteelseriesError::Usb(interrupt_error));
                        }
                    }
                }
                DeviceOperation::ReadInterrupt(endpoint) => {
                    let mut buf = vec![];
                    match self.usb_handle().read_interrupt(
                        *endpoint,
                        &mut buf,
                        std::time::Duration::from_millis(500),
                    ) {
                        Ok(bytes_read) => {
                            OUTPUT.verbose(
                                format!("Device -> Host - Interrupt IN ({} bytes)", bytes_read)
                                    .as_str(),
                            );
                        }
                        Err(interrupt_error) => {
                            OUTPUT.error(
                                format!("Device -> Host - Interrupt - {}", interrupt_error)
                                    .as_str(),
                            );
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
        OUTPUT.verbose("Releasing claimed interfaces");
        let claimed_interfaces = self.claimed_interfaces.iter();
        for claimed_interface in claimed_interfaces {
            match self.usb_handle.release_interface(*claimed_interface) {
                Ok(()) => (),
                Err(_) => (),
            }
        }
    }
}

pub trait SteelseriesDevice {
    fn enumerate_capabilities(&self) -> std::slice::Iter<DeviceCapability>;
    fn get_name(&self) -> &str;
    fn change_property(&self, property: DeviceCapability, value: &str) -> SteelseriesResult<()>;
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

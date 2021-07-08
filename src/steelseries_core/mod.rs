//# This modules contains all the basic structs and traits to interact with Steelseries devices
//#
use std::time::Duration;

mod color;
mod log;
mod steelseries_device;

pub mod support;
pub use color::*;
pub use log::*;
pub use steelseries_device::*;

pub struct URBConfiguration {
    pub request_type: u8,
    pub request: u8,
    pub index: u16,
    pub value: u16,
    pub debug_message: Option<String>,
    pub buf: Option<Vec<u8>>,
    pub timeout: Duration,
}

pub enum Side {
    Left,
    Right,
}

pub enum DeviceOperation {
    WriteInterrupt(u8, Vec<u8>),
    ReadInterrupt(u8),
    WriteControl(URBConfiguration),
    ReadControl(URBConfiguration),
}

pub struct CommandFactory {
    control_timeout: Duration,
    interrupt_timeout: Duration,
    w_index: u16,
    w_value: u16,
    w_request: u8,
    request_type_out: u8,
    request_type_in: u8,
}

impl CommandFactory {
    pub fn new() -> Self {
        let request_type_out = rusb::request_type(
            rusb::Direction::Out,
            rusb::RequestType::Class,
            rusb::Recipient::Interface,
        );

        let request_type_in = rusb::request_type(
            rusb::Direction::In,
            rusb::RequestType::Class,
            rusb::Recipient::Interface,
        );

        Self {
            control_timeout: std::time::Duration::from_millis(500),
            interrupt_timeout: std::time::Duration::from_millis(500),
            w_index: 5,
            w_value: 0x0206,
            w_request: 9,
            request_type_out,
            request_type_in,
        }
    }

    pub fn control_timeout(&mut self, timeout: Duration) {
        self.control_timeout = timeout;
    }

    pub fn interrupt_timeout(&mut self, timeout: Duration) {
        self.interrupt_timeout = timeout;
    }

    pub fn value(&mut self, value: u16) {
        self.w_value = value;
    }

    pub fn index(&mut self, value: u16) {
        self.w_index = value;
    }

    pub fn build_write_control(&self, buf: Vec<u8>) -> DeviceOperation {
        DeviceOperation::WriteControl(URBConfiguration {
            request_type: self.request_type_out,
            request: self.w_request,
            value: self.w_value,
            index: self.w_index,
            buf: Some(buf),
            timeout: self.control_timeout,
            debug_message: None,
        })
    }

    pub fn build_read_control(&self) -> DeviceOperation {
        DeviceOperation::ReadControl(URBConfiguration {
            request_type: self.request_type_in,
            request: self.w_request,
            value: self.w_value,
            index: self.w_index,
            timeout: self.control_timeout,
            buf: None,
            debug_message: None,
        })
    }

    pub fn build_write_interrupt(&self, endpoint: u8) -> DeviceOperation {
        self.build_write_interrupt_with_data(endpoint, vec![])
    }

    pub fn build_write_interrupt_with_data(&self, ic: u8, data: Vec<u8>) -> DeviceOperation {
        DeviceOperation::WriteInterrupt(ic, data)
    }

    pub fn build_read_interrupt(
        &self,
        endpoint: u8
    ) -> DeviceOperation {
        DeviceOperation::ReadInterrupt(endpoint)
    }
}

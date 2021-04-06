use std::time::Duration;

mod color;
mod steelseries_device;
pub use color::*;
pub use steelseries_device::*;

pub struct Payload {
    pub request_type: u8,
    pub request: u8,
    pub index: u16,
    pub value: u16,
    pub debug_message: Option<String>,
    pub buf: Vec<u8>,
    pub timeout: Duration
}

pub enum Side {
    Left,
    Right,
}

pub enum DeviceOperation {
    Interrupt(u8, Vec<u8>),
    Control(Payload)
}

pub struct CommandBuilder {
    control_timeout: Duration,
    interrupt_timeout: Duration,
    w_index: u16,
    w_value: u16,
    request_type_out: u8
}

impl CommandBuilder {
    pub fn new() -> Self {
        let request_type_out = rusb::request_type(
            rusb::Direction::Out,
            rusb::RequestType::Class,
            rusb::Recipient::Interface,
        );
        Self {
            control_timeout: std::time::Duration::from_millis(500),
            interrupt_timeout: std::time::Duration::from_millis(50),
            w_index: 5,
            w_value: 0x0206,
            request_type_out
        }
    }

    pub fn build_control(&self, buf: Vec<u8>) -> DeviceOperation {
        DeviceOperation::Control(
            Payload {
                request_type: self.request_type_out,
                request: 9,
                value: self.w_value,
                index: self.w_index,
                buf,
                timeout: self.control_timeout,
                debug_message: None,
            }
        )
    }

    pub fn build_interrupt(&self, ic: u8) -> DeviceOperation {
        self.build_interrupt_with_data(ic, vec![])
    }

    pub fn build_interrupt_with_data(&self, ic: u8, data: Vec<u8>) -> DeviceOperation {
        DeviceOperation::Interrupt(ic, data)
    }
}

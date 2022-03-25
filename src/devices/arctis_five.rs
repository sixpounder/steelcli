use crate::{
    errors::*,
    steelseries_core::{
        Color, CommandFactory, DeviceCapability, DeviceOperation, Side, SteelseriesDevice,
        ToDescription,
    },
};

use super::OUTPUT;

const STEELSERIES_VENDOR_ID: u16 = 0x1038;
const ARCTIS_5_PID: u16 = 0x12aa;
const ARCTIS_OPS_DELAY: u64 = 20;

pub struct Arctis5Headphones {
    vendor_id: u16,
    product_id: u16,
    capabilities: Vec<DeviceCapability>,
}

impl Arctis5Headphones {
    pub fn new() -> Self {
        Self {
            vendor_id: STEELSERIES_VENDOR_ID,
            product_id: ARCTIS_5_PID,
            capabilities: vec![
                DeviceCapability::from("lhc"),
                DeviceCapability::from("rhc"),
                DeviceCapability::from("hc"),
            ],
        }
    }

    pub fn set_headphone_color(&self, side: Side, color: Color) -> SteelseriesResult<()> {
        let mut device_handle = self.open_device().expect("Failed to open device");
        let iface = match side {
            Side::Left => 5,
            Side::Right => 5,
        };

        super::OUTPUT.verbose(format!("Claimed interface {}", iface));

        match device_handle.claim_interface(iface) {
            Ok(()) => {
                device_handle.process_commands(
                    generate_color_change_operations(color.into()), 
                    Some(std::time::Duration::from_millis(ARCTIS_OPS_DELAY))
                )?;
                // std::thread::sleep(std::time::Duration::from_millis(5000));
                Ok(())
            }
            Err(e) => {
                OUTPUT.error(format!("Could not claim interface").as_str());
                Err(e)
            }
        }
    }
}

impl SteelseriesDevice for Arctis5Headphones {
    fn enumerate_capabilities(&self) -> std::slice::Iter<DeviceCapability> {
        self.capabilities.iter()
    }

    fn get_name(&self) -> &str {
        "Arctis 5"
    }

    fn change_property(&self, property: DeviceCapability, value: &str) -> SteelseriesResult<()> {
        let capability = self
            .capabilities
            .iter()
            .find(|c| **c == property);
        match capability {
            Some(prop) => {
                super::OUTPUT.verbose(
                    format!("Changing {} to {}", prop.to_description(), value).as_str(),
                );
                match prop {
                    DeviceCapability::LeftHeadphoneLedColor => self.set_headphone_color(Side::Left, Color::from(value)),
                    DeviceCapability::RightHeadphoneLedColor => self.set_headphone_color(Side::Right, Color::from(value)),
                    DeviceCapability::HeadphonesColor => match self.set_headphone_color(Side::Left, Color::from(value)) {
                        Ok(_) => self.set_headphone_color(Side::Right, Color::from(value)),
                        Err(e) => Err(e),
                    },
                    _ => Ok(()),
                }
            }
            None => Err(SteelseriesError::InvalidCapability),
        }
    }

    fn get_vendor_id(&self) -> u16 {
        self.vendor_id
    }

    fn get_product_id(&self) -> u16 {
        self.product_id
    }
}

fn generate_color_change_operations(color: Color) -> Vec<DeviceOperation> {
    let mut command_factory = CommandFactory::new();
    command_factory.control_timeout(std::time::Duration::from_millis(500));
    command_factory.interrupt_timeout(std::time::Duration::from_millis(500));

    let mut operations: Vec<DeviceOperation> = vec![
        // cmd.build_write_interrupt(4),
        // cmd.build_read_interrupt(0x84),
        command_factory.build_write_control(vec![
            0x06, 0x8a, 0x42, 0x00, 0x20, 0x40, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xc8,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ]),

        command_factory.build_write_control(vec![
            0x06, 0x81, 0x43, 0x01, 0x23, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ]),

        command_factory.build_write_control(vec![
            0x06, 0x8a, 0x42, 0x00, 0x20, 0x41, 0x00, color.red(), color.green(), color.blue(), 0xff, 0x32, 0xc8, 0xc8,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ]),

        command_factory.build_write_control(vec![
            0x06, 0x81, 0x43, 0x01, 0x23, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ]),

        command_factory.build_write_control(vec![
            0x06, 0x8a, 0x42, 0x00, 0x20, 0x41, 0x08, 0x01, 0x01, 0x00, 0x00, 0x00, 0x00, 0xc8,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ]),
        command_factory.build_write_control(vec![
            0x06, 0x81, 0x43, 0x01, 0x23, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ]),
        // // cmd.build_read_interrupt(4),
        command_factory.build_write_control(vec![
            0x06, 0x8a, 0x42, 0x00, 0x20, 0x60, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xc8,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ]),
        command_factory.build_write_control(vec![
            0x06, 0x81, 0x43, 0x01, 0x23, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ]),
        // cmd.build_read_interrupt(4),
        command_factory.build_write_control(vec![
            0x06, 0x8a, 0x42, 0x00, 0x20, 0x40, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xc8,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ]),
        command_factory.build_write_control(vec![
            0x06, 0x81, 0x43, 0x01, 0x23, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ]),
        // cmd.build_read_interrupt(4),
        command_factory.build_write_control(vec![
            0x06, 0x8a, 0x42, 0x00, 0x20, 0x41, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xc8,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ]),
        command_factory.build_write_control(vec![
            0x06, 0x81, 0x43, 0x01, 0x23, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ]),
        // cmd.build_read_interrupt(4),
        command_factory.build_write_control(vec![
            0x06, 0x8a, 0x42, 0x00, 0x20, 0x41, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xc8,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ]),
    ];

    let mut last_v_byte = 0x08;

    for _ in 0..16 {
        operations.push(command_factory.build_write_control(vec![
            0x06, 0x81, 0x43, 0x01, 0x23, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ]));

        last_v_byte += 8;

        operations.push(command_factory.build_write_control(vec![
            0x06, 0x8a, 0x42, 0x00, 0x20, 0x41, last_v_byte, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xc8,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ]));

        // command_factory.build_read_interrupt(0x84);
    }

    operations.push(command_factory.build_write_control(vec![
        0x06, 0x81, 0x43, 0x01, 0x23, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ]));

    operations.push(command_factory.build_write_control(vec![
        0x06, 0x8a, 0x42, 0x00, 0x20, 0x61, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xc8, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ]));

    // operations.push(cmd.build_write_interrupt(4));

    operations.push(command_factory.build_write_control(vec![
        0x06, 0x81, 0x43, 0x01, 0x23, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ]));

    operations.push(command_factory.build_write_control(vec![
        0x06, 0x8a, 0x42, 0x00, 0x20, 0x61, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xc8, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ]));

    // operations.push(cmd.build_write_interrupt(4));

    operations.push(command_factory.build_write_control(vec![
        0x06, 0x81, 0x43, 0x01, 0x23, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ]));

    operations.push(command_factory.build_write_control(vec![
        0x04, 0x40, 0x01, 0x11, 0x54, 0x9b, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ]));

    // operations.push(cmd.build_write_interrupt(4));

    operations.push(command_factory.build_write_control(vec![
        0x06, 0x81, 0x43, 0x01, 0x22, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ]));

    operations.push(command_factory.build_write_control(vec![
        0x06, 0x8a, 0x42, 0x00, 0x20, 0x05, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ]));

    operations.push(command_factory.build_write_control(vec![
        0x06, 0x81, 0x43, 0x01, 0x23, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ]));

    operations.push(command_factory.build_write_interrupt(4));

    operations
}

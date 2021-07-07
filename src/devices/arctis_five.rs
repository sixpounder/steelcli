use crate::{errors::*, steelseries_core::{Color, CommandFactory, DeviceCapability, DeviceOperation, Side, SteelseriesDevice}};

use super::OUTPUT;

const STEELSERIES_VENDOR_ID: u16 = 0x1038;
const ARCTIS_5_PID: u16 = 0x12aa;

pub struct Arctis5Headphones {
    vendor_id: u16,
    product_id: u16,
    capabilities: Vec<DeviceCapability<'static>>,
}

impl Arctis5Headphones {
    pub fn new() -> Self {
        Self {
            vendor_id: STEELSERIES_VENDOR_ID,
            product_id: ARCTIS_5_PID,
            capabilities: vec![
                DeviceCapability::from(("lhc", "Left headphone color")),
                DeviceCapability::from(("rhc", "Right headphone color")),
                DeviceCapability::from(("hc", "Both headphones color")),
            ],
        }
    }

    pub fn set_headphone_color(&self, side: Side, color: Color) -> SteelseriesResult<()> {
        let mut device_handle = self.open_device().expect("Failed to open device");
        let iface = match side {
            Side::Left => 5,
            Side::Right => 5,
        };

        match device_handle.claim_interface(iface) {
            Ok(()) => {
                
                device_handle.process_commands(generate_color_change_operations(color.into()))?;
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

    fn change_property(&self, property: &str, value: &str) -> SteelseriesResult<()> {
        super::OUTPUT.verbose(format!("Changing {} to {}", property, value).as_str());
        match property {
            "lhc" => self.set_headphone_color(Side::Left, Color::from(value)),
            "rhc" => self.set_headphone_color(Side::Right, Color::from(value)),
            "hc" => match self.set_headphone_color(Side::Left, Color::from(value)) {
                Ok(_) => self.set_headphone_color(Side::Right, Color::from(value)),
                Err(e) => Err(e),
            },
            _ => Ok(()),
        }
    }

    fn get_vendor_id(&self) -> u16 {
        self.vendor_id
    }

    fn get_product_id(&self) -> u16 {
        self.product_id
    }
}

fn generate_color_change_operations(color: (u8, u8, u8)) -> Vec<DeviceOperation> {
    let mut cmd = CommandFactory::new();
    cmd.control_timeout(std::time::Duration::from_millis(500));
    cmd.interrupt_timeout(std::time::Duration::from_millis(500));

    let operations: Vec<DeviceOperation> = vec![
        // cmd.build_write_interrupt(4),
        // cmd.build_read_interrupt(0x84),
        cmd.build_write_control(vec![
            0x06, 0x8a, 0x42, 0x00, 0x20, 0x40, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xc8,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ]),
        // cmd.build_read_control(),
        cmd.build_write_control(vec![
            0x06, 0x81, 0x43, 0x01, 0x23, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ]),
        // cmd.build_read_control(),

        // cmd.build_read_interrupt(0x84),
        // cmd.build_write_interrupt_with_data(0x84, vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]),
        // cmd.build_write_interrupt(0x84),

        cmd.build_write_control(vec![
            0x06, 0x8a, 0x42, 0x00, 0x20, 0x41, 0x00, color.0, color.1, color.2, 0xff, 0x32, 0xc8,
            0xc8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ]),
        // cmd.build_read_control(),
        cmd.build_write_control(vec![
            0x06, 0x81, 0x43, 0x01, 0x23, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ]),
        // cmd.build_read_control(),

        // cmd.build_read_interrupt(0x84),
        //cmd.build_write_interrupt(0x84),

        // cmd.build_write_control(vec![
        //     0x06, 0x8a, 0x42, 0x00, 0x20, 0x41, 0x08, 0x01, 0x01, 0x00, 0x00, 0x00, 0x00, 0xc8,
        //     0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        //     0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        // ]),
        // cmd.build_write_control(vec![
        //     0x06, 0x81, 0x43, 0x01, 0x23, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        //     0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        //     0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        // ]),
        // // cmd.build_read_interrupt(4),

        // cmd.build_write_control(vec![
        //     0x06, 0x8a, 0x42, 0x00, 0x20, 0x60, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xc8,
        //     0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        //     0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        // ]),
        // cmd.build_write_control(vec![
        //     0x06, 0x81, 0x43, 0x01, 0x23, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        //     0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        //     0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        // ]),
        // // cmd.build_read_interrupt(4),

        // cmd.build_write_control(vec![
        //     0x06, 0x8a, 0x42, 0x00, 0x20, 0x40, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xc8,
        //     0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        //     0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        // ]),
        // cmd.build_write_control(vec![
        //     0x06, 0x81, 0x43, 0x01, 0x23, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        //     0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        //     0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        // ]),
        // // cmd.build_read_interrupt(4),

        // cmd.build_write_control(vec![
        //     0x06, 0x8a, 0x42, 0x00, 0x20, 0x41, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xc8,
        //     0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        //     0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        // ]),
        // cmd.build_write_control(vec![
        //     0x06, 0x81, 0x43, 0x01, 0x23, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        //     0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        //     0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        // ]),
        // // cmd.build_read_interrupt(4),

        // cmd.build_write_control(vec![
        //     0x06, 0x8a, 0x42, 0x00, 0x20, 0x41, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xc8,
        //     0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        //     0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        // ]),
    ];

    // let mut last_v_byte = 0x08;

    // for _ in 0..16 {
    //     cmd.build_write_control(vec![
    //         0x06, 0x81, 0x43, 0x01, 0x23, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    //         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    //         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    //     ]);

    //     last_v_byte += 8;

    //     cmd.build_write_control(vec![
    //         0x06, 0x8a, 0x42, 0x00, 0x20, 0x41, last_v_byte, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xc8,
    //         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    //         0x00,0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    //     ]);

    //     cmd.build_read_interrupt(0x84);
    // }

    // operations.push(cmd.build_write_control(vec![
    //     0x06, 0x81, 0x43, 0x01, 0x23, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    //     0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    //     0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    // ]));

    // operations.push(cmd.build_write_control(vec![
    //     0x06, 0x8a, 0x42, 0x00, 0x20, 0x61, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xc8,
    //     0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    //     0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    // ]));

    // // operations.push(cmd.build_write_interrupt(4));

    // operations.push(cmd.build_write_control(vec![
    //     0x06, 0x81, 0x43, 0x01, 0x23, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    //     0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    //     0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    // ]));

    // operations.push(cmd.build_write_control(vec![
    //     0x04, 0x40, 0x01, 0x11, 0x54, 0x9b, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    //     0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    //     0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    // ]));

    // // operations.push(cmd.build_write_interrupt(4));

    // operations.push(cmd.build_write_control(vec![
    //     0x06, 0x81, 0x43, 0x01, 0x22, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    //     0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    //     0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    // ]));

    // operations.push(cmd.build_write_control(vec![
    //     0x06, 0x8a, 0x42, 0x00, 0x20, 0x05, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    //     0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    //     0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    // ]));

    // operations.push(cmd.build_write_control(vec![
    //     0x06, 0x81, 0x43, 0x01, 0x23, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    //     0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    //     0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    // ]));

    // operations.push(cmd.build_write_interrupt(4));

    operations
}

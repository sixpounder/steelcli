use rusb::Error;

use crate::lcore::{
    Color, CommandFactory, DeviceCapability, DeviceOperation, Side, SteelseriesDevice,
};

use super::LOGGER;

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

    pub fn set_headphone_color(&self, side: Side, color: Color) -> Result<(), Error> {
        let (mut _device, mut handle) = self.open_device().expect("Failed to open device");
        let iface = match side {
            Side::Left => 5,
            Side::Right => 5,
        };

        handle
            .set_auto_detach_kernel_driver(true)
            .expect("Could not detach kernel driver");

        // println!("Payload size: {}", payload.len());

        match handle.claim_interface(iface) {
            Ok(()) => {
                for op in generate_color_change_operations(color.into()).iter() {
                    match op {
                        DeviceOperation::Control(payload) => {
                            match handle.write_control(
                                payload.request_type,
                                payload.request,
                                payload.value,
                                payload.index,
                                &payload.buf,
                                payload.timeout,
                            ) {
                                Ok(size) => {
                                    LOGGER.verbose(|| {
                                        println!("Device <- {} bytes", size);
                                    });

                                    LOGGER.verbose(|| {
                                        if let Some(m) = payload.debug_message.as_ref() {
                                            println!("{}", m);
                                        }
                                    });
                                }
                                Err(e) => {
                                    println!("Error: {}", e);
                                }
                            }
                        }
                        DeviceOperation::Interrupt(endpoint, buf) => {
                            handle
                                .write_interrupt(
                                    *endpoint,
                                    buf,
                                    std::time::Duration::from_millis(50),
                                )
                                .expect("Interrupt error");
                        }
                    }
                }

                handle.release_interface(iface)
            }
            Err(e) => {
                println!("Could not claim interface: {}", e);
                Err(e)
            }
        }
    }
}

impl SteelseriesDevice for Arctis5Headphones {
    fn matches(&self, vendor_id: u16, product_id: u16) -> bool {
        self.product_id == product_id && self.vendor_id == vendor_id
    }

    fn enumerate_capabilities(&self) -> std::slice::Iter<DeviceCapability> {
        self.capabilities.iter()
    }

    fn get_name(&self) -> &str {
        "Arctis 5"
    }

    fn change_property(&self, property: &str, value: &str) -> Result<(), Error> {
        super::LOGGER.verbose(|| {
            println!("Changing {} to {}", property, value);
        });
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
    let cmd = CommandFactory::new();

    let mut operations: Vec<DeviceOperation> = vec![
        cmd.build_control(vec![
            0x06, 0x8a, 0x42, 0x00, 0x20, 0x40, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xc8,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ]),
        cmd.build_control(vec![
            0x06, 0x81, 0x43, 0x01, 0x23, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ]),
        cmd.build_interrupt(4),

        cmd.build_control(vec![
            0x06, 0x8a, 0x42, 0x00, 0x20, 0x41, 0x00, color.0, color.1, color.2, 0xff, 0x32, 0xc8,
            0xc8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ]),
        cmd.build_control(vec![
            0x06, 0x81, 0x43, 0x01, 0x23, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ]),
        cmd.build_interrupt(4),

        cmd.build_control(vec![
            0x06, 0x8a, 0x42, 0x00, 0x20, 0x41, 0x08, 0x01, 0x01, 0x00, 0x00, 0x00, 0x00, 0xc8,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ]),
        cmd.build_control(vec![
            0x06, 0x81, 0x43, 0x01, 0x23, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ]),
        cmd.build_interrupt(4),

        cmd.build_control(vec![
            0x06, 0x8a, 0x42, 0x00, 0x20, 0x60, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xc8,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ]),
        cmd.build_control(vec![
            0x06, 0x81, 0x43, 0x01, 0x23, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ]),
        cmd.build_interrupt(4),

        cmd.build_control(vec![
            0x06, 0x8a, 0x42, 0x00, 0x20, 0x40, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xc8,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ]),
        cmd.build_control(vec![
            0x06, 0x81, 0x43, 0x01, 0x23, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ]),
        cmd.build_interrupt(4),

        cmd.build_control(vec![
            0x06, 0x8a, 0x42, 0x00, 0x20, 0x41, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xc8,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ]),
        cmd.build_control(vec![
            0x06, 0x81, 0x43, 0x01, 0x23, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ]),
        cmd.build_interrupt(4),

        cmd.build_control(vec![
            0x06, 0x8a, 0x42, 0x00, 0x20, 0x41, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xc8,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ]),
    ];

    let mut last_v_byte = 0x08;

    for _ in 0..16 {
        operations.push(cmd.build_control(vec![
            0x06, 0x81, 0x43, 0x01, 0x23, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ]));

        last_v_byte += 8;

        operations.push(cmd.build_control(vec![
            0x06, 0x8a, 0x42, 0x00, 0x20, 0x41, last_v_byte, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xc8,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00,0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ]));

        operations.push(cmd.build_interrupt(4));
    }

    operations.push(cmd.build_control(vec![
        0x06, 0x81, 0x43, 0x01, 0x23, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ]));

    operations.push(cmd.build_control(vec![
        0x06, 0x8a, 0x42, 0x00, 0x20, 0x61, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xc8,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ]));

    operations.push(cmd.build_interrupt(4));

    operations.push(cmd.build_control(vec![
        0x06, 0x81, 0x43, 0x01, 0x23, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ]));

    operations.push(cmd.build_control(vec![
        0x04, 0x40, 0x01, 0x11, 0x54, 0x9b, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ]));

    operations.push(cmd.build_interrupt(4));

    operations.push(cmd.build_control(vec![
        0x06, 0x81, 0x43, 0x01, 0x22, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ]));

    operations.push(cmd.build_control(vec![
        0x06, 0x8a, 0x42, 0x00, 0x20, 0x05, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ]));

    operations.push(cmd.build_control(vec![
        0x06, 0x81, 0x43, 0x01, 0x23, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ]));

    operations.push(cmd.build_interrupt(4));

    operations
}

use rusb::{Context, DeviceHandle};

use super::{DeviceCapability, SteelseriesDevice};

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
                DeviceCapability::from(("hc", "Both headphones color"))
            ]
        }
    }

    pub fn set_left_color(&self, color: (u8, u8, u8)) {
        self.set_headphone_color(1, color);
    }

    pub fn set_right_color(&self, color: (u8, u8, u8)) {
        self.set_headphone_color(0, color);
    }

    pub fn set_both_color(&self, color: (u8, u8, u8)) {
        self.set_left_color(color);
        self.set_right_color(color);
    }

    fn set_headphone_color(&self, side: u8, color: (u8, u8, u8)) {
        let (mut _device, mut handle) = self.open_device().expect("Failed to open device");
        let iface = 5;

        handle.set_auto_detach_kernel_driver(true).expect("Could not detach kernel driver");
        let payloads: Vec<Vec<u8>> = vec![
            vec![
                0x06, 0x8a, 0x42, 0x00, 0x20, 0x41, 0x00, 0x00, 0x00, 0xff, 0xff, 0x32, 0xc8, 0xc8, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            ]
        ];

        // println!("Payload size: {}", payload.len());

        match handle.claim_interface(iface) {
            Ok(()) => {
                match handle.write_control(
                    rusb::request_type(
                        rusb::Direction::Out,
                        rusb::RequestType::Class,
                        rusb::Recipient::Interface,
                    ),
                    9,
                    0x0206,
                    iface.into(),
                    &payloads[0],
                    std::time::Duration::from_secs(1),
                ) {
                    Ok(size) => {
                        println!("{} bytes transferred", size);
                    }
                    Err(e) => {
                        // return Err(e);
                    }
                }

                handle.release_interface(iface).unwrap();
            }
            Err(e) => {
                println!("Could not claim interface: {}", e);
            }
        }
    }
}

impl SteelseriesDevice for Arctis5Headphones {
    fn matches(&self, vendor_id: u16, product_id: u16) -> bool {
        self.product_id == product_id && self.vendor_id == vendor_id
    }

    fn enumerate_capabilities(&self) -> std::slice::Iter<super::DeviceCapability> {
        self.capabilities.iter()
    }

    fn get_name(&self) -> &str {
        "Arctis 5"
    }

    fn change_property(&self, property: &str, value: &str) -> bool {
        todo!()
    }

    fn get_vendor_id(&self) -> u16 {
        self.vendor_id
    }

    fn get_product_id(&self) -> u16 {
        self.product_id
    }
}

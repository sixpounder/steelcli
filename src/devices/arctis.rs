use super::DeviceCapability;
use super::SteelseriesDevice;

const STEELSERIES_VENDOR_ID: u16 = 0x1038;
const ARCTIS_5_PID: u16 = 0x12aa;

pub struct Arctis5Headphones {
    vendor_id: u16,
    product_id: u16,
    capabilities: Vec<DeviceCapability>
}

impl Arctis5Headphones {
    pub fn new() -> Self {
        Self {
            vendor_id: STEELSERIES_VENDOR_ID,
            product_id: ARCTIS_5_PID,
            capabilities: vec![]
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

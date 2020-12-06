use super::SteelseriesDevice;

pub struct Arctis5Headphones {}

impl SteelseriesDevice for Arctis5Headphones {
    fn matches(&self, vendor_id: u16, product_id: u16) -> bool {
        todo!()
    }

    fn enumerate_capabilities(&self) -> std::slice::Iter<super::DeviceCapability> {
        todo!()
    }

    fn get_name(&self) -> &str {
        todo!()
    }

    fn change_property(&self, property: &str, value: &str) -> bool {
        todo!()
    }

    fn get_vendor_id(&self) -> u16 {
        todo!()
    }

    fn get_product_id(&self) -> u16 {
        todo!()
    }
}

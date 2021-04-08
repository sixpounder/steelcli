use super::SteelseriesDevice;

const STEELSERIES_VENDOR_ID: u16 = 0x1038;
const ARCTIS_5_PID: u16 = 0x12aa;

pub struct DevicePool {
    devices: Vec<Box<dyn SteelseriesDevice>>,
}

impl DevicePool {
    pub fn new() -> Self {
        let mut devices = vec![];

        let arctis_five: Box<dyn SteelseriesDevice> = Box::new(crate::devices::Arctis5Headphones::new());
        devices.push(arctis_five);

        Self { devices }
    }

    pub fn find_one(&self, vendor_id: u16, product_id: u16) -> Option<&dyn SteelseriesDevice> {
        let mut ret: Option<&dyn SteelseriesDevice> = None;
        self.devices.iter().for_each(|f| {
            if f.matches(vendor_id, product_id) {
                ret = Some(&**f);
            }
        });

        ret
    }
}

pub fn supported_devices() -> Vec<(u16, u16, String)> {
    let supported_devices: Vec<(u16, u16, String)> = vec![(
        STEELSERIES_VENDOR_ID, ARCTIS_5_PID, String::from("Arctis 5 Headset")
    )];

    supported_devices
}

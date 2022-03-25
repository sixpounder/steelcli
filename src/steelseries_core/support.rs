use lazy_static::__Deref;

use super::SteelseriesDevice;

pub struct DevicePool {
    pub(crate) devices: Vec<Box<dyn SteelseriesDevice>>,
}

impl DevicePool {
    pub fn new() -> Self {
        let mut devices = vec![];

        let arctis_five: Box<dyn SteelseriesDevice> =
            Box::new(crate::devices::ArctisFiveHeadphones::new());
        devices.push(arctis_five);

        let sensei_ten: Box<dyn SteelseriesDevice> =
            Box::new(crate::devices::SenseiTenMouse::new());
        devices.push(sensei_ten);

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

    pub fn find_by_slug(&self, slug: &str) -> Option<&dyn SteelseriesDevice> {
        let mut ret: Option<&dyn SteelseriesDevice> = None;
        self.devices.iter().for_each(|f| {
            if f.get_slug() == slug {
                ret = Some(&**f);
            }
        });

        ret
    }

    #[allow(dead_code)]
    pub fn first(&self) -> Option<&dyn SteelseriesDevice> {
        Some(self.devices[0].deref())
    }

    /**
     * Consumes `self` and returns a new `DevicePool` filtered with supported devices
     * that are actually connected to the host
     */
    pub fn sync(self) -> DevicePool {
        let api = &crate::HIDAPI;
        let mut connected_devices = api.device_list();
        let mut filtered_devices: Vec<Box<dyn SteelseriesDevice>> = vec![];
        for device in self.devices {
            if let Some(_connected_device) = connected_devices.find(|d| {
                d.vendor_id() == device.get_vendor_id() && d.product_id() == device.get_product_id()
            }) {
                filtered_devices.push(device);
            }
        }

        DevicePool {
            devices: filtered_devices,
        }
    }
}

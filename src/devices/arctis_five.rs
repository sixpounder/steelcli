use std::collections::HashMap;

use crate::steelseries_core::{
    Color, DeviceProfileValue, DeviceProperty, Side, SteelseriesDevice, Error,
    Result, ToDescription, STEELSERIES_VENDOR_ID,
};

const ARCTIS_5_PID: u16 = 0x12aa;

pub struct ArctisFiveHeadphones {
    vendor_id: u16,
    product_id: u16,
    capabilities: Vec<DeviceProperty>,
    profile: HashMap<&'static str, DeviceProfileValue>,
}

impl ArctisFiveHeadphones {
    pub fn new() -> Self {
        Self {
            vendor_id: STEELSERIES_VENDOR_ID,
            product_id: ARCTIS_5_PID,
            capabilities: vec![
                DeviceProperty::from("lhc"),
                DeviceProperty::from("rhc"),
                DeviceProperty::from("hc"),
            ],
            profile: HashMap::new(),
        }
    }

    pub fn set_headphone_color(&self, _side: Side, _color: Color) -> Result<()> {
        todo!()
    }
}

impl SteelseriesDevice for ArctisFiveHeadphones {
    fn enumerate_capabilities(&self) -> std::slice::Iter<DeviceProperty> {
        self.capabilities.iter()
    }

    fn get_name(&self) -> &str {
        "Arctis Five Headphones"
    }

    fn get_slug(&self) -> &str {
        "arctis5"
    }

    fn change_property(&self, property: DeviceProperty, value: &str) -> Result<()> {
        let capability = self.capabilities.iter().find(|c| **c == property);
        match capability {
            Some(prop) => {
                super::OUTPUT
                    .verbose(format!("Changing {} to {}", prop.to_description(), value).as_str());
                match prop {
                    DeviceProperty::LeftHeadphoneLedColor => {
                        self.set_headphone_color(Side::Left, Color::from(value))
                    }
                    DeviceProperty::RightHeadphoneLedColor => {
                        self.set_headphone_color(Side::Right, Color::from(value))
                    }
                    DeviceProperty::HeadphonesColor => {
                        match self.set_headphone_color(Side::Left, Color::from(value)) {
                            Ok(_) => self.set_headphone_color(Side::Right, Color::from(value)),
                            Err(e) => Err(e),
                        }
                    }
                    _ => {
                        super::OUTPUT.verbose(format!(
                            "Property {} not supported by this device",
                            prop.to_description()
                        ));
                        Ok(())
                    }
                }
            }
            None => Err(Error::InvalidCapability),
        }
    }

    fn get_vendor_id(&self) -> u16 {
        self.vendor_id
    }

    fn get_product_id(&self) -> u16 {
        self.product_id
    }

    fn get_profile(
        &self,
    ) -> Option<&std::collections::HashMap<&str, crate::steelseries_core::DeviceProfileValue>> {
        Some(&self.profile)
    }
}

use std::collections::HashMap;

use crate::{
    errors::{SteelseriesError, SteelseriesResult},
    steelseries_core::{
        Color, DeviceProfileValue, DeviceProperty, SteelseriesDevice, ToDescription,
        STEELSERIES_VENDOR_ID,
    },
};

const SENSEI_TEN_PID: u16 = 0x1832;

// GRADIENT HEADER = {
//     "header_length": 26,       # Length of the header excuding command / LED ID
//     "led_id_offsets": [0],     # Offset of the "led_id" fields
//     "duration_offset": 1,      # Offset of the "duration" field
//     "duration_length": 2,      # Length of the "duration" field (in Bytes)
//     "repeat_offset": 17,       # Offset of the "repeat" flag
//     "triggers_offset": 21,     # Offset of the "triggers" field (buttons mask)
//     "color_count_offset": 25,  # Offset of the "color_count" field
// }

pub struct SenseiTenMouse {
    vendor_id: u16,
    product_id: u16,
    capabilities: Vec<DeviceProperty>,
    profile: HashMap<&'static str, DeviceProfileValue>,
}

impl SenseiTenMouse {
    pub fn new() -> Self {
        let mut profile = HashMap::new();
        profile.insert("rgbgradh_header_length", DeviceProfileValue::Hex(26));
        profile.insert("rgbgradh_led_id_offsets", DeviceProfileValue::Hex(26));
        profile.insert(
            "rgbgradh_duration_offset",
            DeviceProfileValue::ByteList(&[0]),
        );
        profile.insert("rgbgradh_duration_length", DeviceProfileValue::Byte(2));
        profile.insert("rgbgradh_repeat_offset", DeviceProfileValue::Byte(17));
        profile.insert("rgbgradh_triggers_offset", DeviceProfileValue::Byte(21));
        profile.insert("rgbgradh_color_count_offset", DeviceProfileValue::Hex(25));

        Self {
            vendor_id: STEELSERIES_VENDOR_ID,
            product_id: SENSEI_TEN_PID,
            capabilities: vec![DeviceProperty::LedColor],
            profile,
        }
    }

    pub fn set_logo_color(&self, value: Color) -> SteelseriesResult<()> {
        let api = &crate::HIDAPI;
        let handle = api
            .open(self.get_vendor_id(), self.get_product_id())
            .unwrap();
        let buf = [0x00, 0x5B, 0x00, 0xff, 0x00, 0x00];
        let size = handle.send_feature_report(&buf).unwrap();
        let buf = [0x59, 0x00];
        let size = handle.write(&buf).unwrap();
        Ok(())
    }
}

impl SteelseriesDevice for SenseiTenMouse {
    fn enumerate_capabilities(&self) -> std::slice::Iter<DeviceProperty> {
        self.capabilities.iter()
    }

    fn get_name(&self) -> &str {
        "Sensei Ten Mouse"
    }

    fn get_slug(&self) -> &str {
        "senseiten"
    }

    fn change_property(
        &self,
        property: DeviceProperty,
        value: &str,
    ) -> crate::errors::SteelseriesResult<()> {
        let capability = self.capabilities.iter().find(|c| **c == property);
        match capability {
            Some(prop) => {
                super::OUTPUT
                    .verbose(format!("Changing {} to {}", prop.to_description(), value).as_str());
                match prop {
                    DeviceProperty::LedColor => self.set_logo_color(Color::from(value)),
                    _ => {
                        super::OUTPUT.verbose(format!(
                            "Property {} not supported by this device",
                            prop.to_description()
                        ));
                        Ok(())
                    }
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

    fn get_profile(&self) -> Option<&HashMap<&str, crate::steelseries_core::DeviceProfileValue>> {
        Some(&self.profile)
    }
}

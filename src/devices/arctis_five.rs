use std::collections::HashMap;

use crate::{
    get_profile_value,
    steelseries_core::{
        Color, DeviceProfileValue, DeviceProperty, Error, RGBGradient, RGBGradientSettings, Result,
        Side, SteelseriesDevice, TaskOptions, ToDescription, STEELSERIES_VENDOR_ID,
    },
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
        let mut profile = HashMap::new();
        profile.insert("rgbgradh_header_length", DeviceProfileValue::Hex(26));
        profile.insert(
            "rgbgradh_led_id_offsets",
            DeviceProfileValue::ByteList(&[0]),
        );
        profile.insert("rgbgradh_duration_offset", DeviceProfileValue::Byte(1));
        profile.insert("rgbgradh_duration_length", DeviceProfileValue::Byte(2));
        profile.insert("rgbgradh_repeat_offset", DeviceProfileValue::Byte(17));
        profile.insert("rgbgradh_triggers_offset", DeviceProfileValue::Byte(21));
        profile.insert("rgbgradh_color_count_offset", DeviceProfileValue::Hex(25));

        profile.insert(
            "logo_color_command",
            DeviceProfileValue::ByteList(&[0x5B, 0x00]),
        );

        profile.insert("save_command", DeviceProfileValue::ByteList(&[0x59, 0x00]));

        Self {
            vendor_id: STEELSERIES_VENDOR_ID,
            product_id: ARCTIS_5_PID,
            capabilities: vec![
                DeviceProperty::from("lhc"),
                DeviceProperty::from("rhc"),
                DeviceProperty::from("hc"),
            ],
            profile: profile,
        }
    }

    pub fn set_headphone_color(&self, _side: Side, color: Color) -> Result<()> {
        if let Ok(handle) = self.open() {
            let header_length = get_profile_value!(self, "rgbgradh_header_length", as_hex);
            let led_id_offsets = get_profile_value!(self, "rgbgradh_led_id_offsets", as_byte_list);
            let duration_offset = get_profile_value!(self, "rgbgradh_duration_offset", as_byte);
            let duration_length = get_profile_value!(self, "rgbgradh_duration_length", as_byte);
            let repeat_offset = get_profile_value!(self, "rgbgradh_repeat_offset", as_byte);
            let triggers_offset = get_profile_value!(self, "rgbgradh_triggers_offset", as_byte);
            let color_count_offset =
                get_profile_value!(self, "rgbgradh_color_count_offset", as_hex);

            let command = get_profile_value!(self, "logo_color_command", as_byte_list);
            let rgbgradient = RGBGradient::from(color);

            let processed = rgbgradient.process(RGBGradientSettings {
                header_length: *header_length,
                led_id_offsets: led_id_offsets,
                duration_offset: *duration_offset,
                duration_length: *duration_length,
                repeat_offset: *repeat_offset,
                triggers_offset: *triggers_offset,
                color_count_offset: *color_count_offset,
            });

            let merged_command = [command, processed.as_slice()].concat();

            handle.send_feature_report(merged_command.as_slice())
        } else {
            Err(Error::OpenDevice)
        }
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

    fn change_property(
        &self,
        property: DeviceProperty,
        value: &str,
        _options: &TaskOptions,
    ) -> Result<()> {
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

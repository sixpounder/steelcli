use std::{collections::HashMap, convert::TryFrom, slice::Iter, time::Duration};
use super::Error;

lazy_static! {
    pub static ref NAMED_COLORS: HashMap<&'static str, Color> = {
        let mut map = HashMap::new();
        map.insert("white", Color::from((0xFF, 0xFF, 0xFF)));
        map.insert("silver", Color::from((0xC0, 0xC0, 0xC0)));
        map.insert("gray", Color::from((0x80, 0x80, 0x80)));
        map.insert("black", Color::from((0x00, 0x00, 0x00)));
        map.insert("red", Color::from((0xFF, 0x00, 0x00)));
        map.insert("maroon", Color::from((0x80, 0x00, 0x00)));
        map.insert("yellow", Color::from((0xFF, 0xFF, 0x00)));
        map.insert("olive", Color::from((0x80, 0x80, 0x00)));
        map.insert("lime", Color::from((0x00, 0xFF, 0x00)));
        map.insert("green", Color::from((0x00, 0x80, 0x00)));
        map.insert("aqua", Color::from((0x00, 0xFF, 0xFF)));
        map.insert("teal", Color::from((0x00, 0x80, 0x80)));
        map.insert("blue", Color::from((0x00, 0x00, 0xFF)));
        map.insert("navy", Color::from((0x00, 0x00, 0x80)));
        map.insert("fuchsia", Color::from((0xFF, 0x00, 0xFF)));
        map.insert("purple", Color::from((0x80, 0x00, 0x80)));

        map
    };
}

const DEFAULT_DURATION: Duration = Duration::from_secs(1);

#[derive(Debug, PartialEq, Clone)]
pub struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl Color {
    pub fn red(&self) -> u8 {
        self.red
    }

    pub fn green(&self) -> u8 {
        self.green
    }

    pub fn blue(&self) -> u8 {
        self.blue
    }
}

impl From<(u8, u8, u8)> for Color {
    fn from(values: (u8, u8, u8)) -> Self {
        Self {
            red: values.0,
            green: values.1,
            blue: values.2,
        }
    }
}

impl Into<(u8, u8, u8)> for Color {
    fn into(self) -> (u8, u8, u8) {
        (self.red(), self.green(), self.blue())
    }
}

impl Into<[u8; 3]> for Color {
    fn into(self) -> [u8; 3] {
        [self.red(), self.green(), self.blue()]
    }
}

impl From<&[u8]> for Color {
    fn from(buf: &[u8]) -> Self {
        Self {
            red: buf[0],
            green: buf[1],
            blue: buf[2],
        }
    }
}

impl From<&str> for Color {
    fn from(s: &str) -> Self {
        let hex_regex = regex::Regex::new(r"^[a-fA-F0-9]{6}$").unwrap();

        if NAMED_COLORS.contains_key(s) {
            NAMED_COLORS.get(s).unwrap().clone()
        } else if hex_regex.is_match(s) {
            let hex_channels = s.replacen("#", "", 1);
            let hex_channels = hex_channels.trim();
            Self {
                red: u8::from_str_radix(&hex_channels[0..2], 16)
                    .expect("Cant't read RED channel")
                    .wrapping_shr(16)
                    & 255u8,
                green: u8::from_str_radix(&hex_channels[2..4], 16)
                    .expect("Cant't read GREEN channel")
                    .wrapping_shr(8)
                    & 255u8,
                blue: u8::from_str_radix(&hex_channels[4..6], 16)
                    .expect("Cant't read BLUE channel")
                    & 255,
            }
        } else {
            let parts: Vec<&str> = s.split(',').collect();
            Self {
                red: parts[0]
                    .trim()
                    .parse::<u8>()
                    .expect("Cant't read RED channel"),
                green: parts[1]
                    .trim()
                    .parse::<u8>()
                    .expect("Cant't read GREEN channel"),
                blue: parts[2]
                    .trim()
                    .parse::<u8>()
                    .expect("Cant't read BLUE channel"),
            }
        }
    }
}

pub struct RGBGradient {
    _duration: Duration,
    colors: Vec<Color>,
}

impl Default for RGBGradient {
    fn default() -> Self {
        Self {
            _duration: DEFAULT_DURATION,
            colors: vec![],
        }
    }
}

impl TryFrom<&str> for RGBGradient {
    type Error = Error;

    fn try_from(raw: &str) -> Result<Self, Self::Error> {
        let hex_regex = regex::Regex::new(r"^[a-fA-F0-9]{6}$").unwrap();
        let rgb_regex = regex::Regex::new(r"^[0-9]{1,3},[0-9]{1,3},[0-9]{1,3}$").unwrap();
        let color_literal_regex = regex::Regex::new(r"^[a-z]+$").unwrap();
        if hex_regex.is_match(raw) || rgb_regex.is_match(raw) || color_literal_regex.is_match(raw) {
            RGBGradient::new_with_colors(vec![Color::from(raw)])
        } else {
            Err(Error::Conversion)
        }
    }
}

impl From<Vec<Color>> for RGBGradient {
    fn from(colors: Vec<Color>) -> Self {
        RGBGradient::new_with_colors(colors).unwrap()
    }
}

impl RGBGradient {
    pub fn new_with_colors(colors: Vec<Color>) -> Result<Self, Error> {
        if colors.len() > 14 {
            Err(Error::Generic(
                "A maximum of 14 color stops can be defined in a gradient",
            ))
        } else {
            Ok(Self {
                _duration: DEFAULT_DURATION,
                colors,
            })
        }
    }

    pub(crate) fn iter_colors(&self) -> Iter<Color> {
        self.colors.iter()
    }

    pub fn is_gradient(&self) -> bool {
        self.colors.len() > 1
    }

    pub(crate) fn process(&self, settings: RGBGradientSettings) -> ProcessedRGBGradient {
        // Generate header

        let mut header: Vec<u8> = vec![];
        for _ in 0..settings.header_length {
            header.push(0x00);
        }

        if self.is_gradient() {
            header[settings.repeat_offset as usize] = 0x01;
        }

        header[settings.color_count_offset as usize] = self.colors.len() as u8;

        for led_id_offset in settings.led_id_offsets {
            header[*led_id_offset as usize] = 0x00;
        }

        // Generate body

        let mut body: Vec<u8> = vec![];

        body.push(self.colors[0].red);
        body.push(self.colors[0].green);
        body.push(self.colors[0].blue);

        let mut last_real_pos = 0;
        let mut color_index = 0;
        self.iter_colors().for_each(|color| {
            let real_pos = color_index * 255 / 100;
            let color_bytes: [u8; 3] = color.clone().into();
            let mut color_bytes: Vec<u8> = color_bytes.into();
            body.append(&mut color_bytes);
            body.push((real_pos - last_real_pos) as u8);
            last_real_pos = real_pos;
            color_index += 1;
        });

        header.append(&mut body);

        ProcessedRGBGradient {
            _gradient: self,
            bytes: header,
        }
    }
}

impl From<Color> for RGBGradient {
    fn from(source: Color) -> Self {
        Self {
            _duration: DEFAULT_DURATION,
            colors: vec![source],
        }
    }
}

#[allow(dead_code)]
pub(crate) struct RGBGradientSettings<'a> {
    pub(crate) header_length: u16,
    pub(crate) led_id_offsets: &'a [u8],
    pub(crate) duration_offset: u8,
    pub(crate) duration_length: u8,
    pub(crate) repeat_offset: u8,
    pub(crate) triggers_offset: u8,
    pub(crate) color_count_offset: u16,
}

pub struct ProcessedRGBGradient<'a> {
    _gradient: &'a RGBGradient,
    bytes: Vec<u8>,
}

impl ProcessedRGBGradient<'_> {
    pub fn as_slice(&self) -> &[u8] {
        self.bytes.as_slice()
    }
}

#[cfg(test)]
mod tests {
    use super::Color;
    #[test]
    fn conversion() {
        let rgb = "112,152,205";
        let hex = "#7098CD";
        assert_eq!(Color::from(hex), Color::from(rgb));
        let tuple: (u8, u8, u8) = Color::from(hex).into();
        assert_eq!(tuple, (112, 152, 205));
    }
}

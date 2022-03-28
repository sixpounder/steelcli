use std::{time::Duration, slice::Iter};

// lazy_static! {
//     pub static ref NAMED_COLORS = vec![
//         "white":   (0xFF, 0xFF, 0xFF),
//         "silver":  (0xC0, 0xC0, 0xC0),
//         "gray":    (0x80, 0x80, 0x80),
//         "black":   (0x00, 0x00, 0x00),
//         "red":     (0xFF, 0x00, 0x00),
//         "maroon":  (0x80, 0x00, 0x00),
//         "yellow":  (0xFF, 0xFF, 0x00),
//         "olive":   (0x80, 0x80, 0x00),
//         "lime":    (0x00, 0xFF, 0x00),
//         "green":   (0x00, 0x80, 0x00),
//         "aqua":    (0x00, 0xFF, 0xFF),
//         "teal":    (0x00, 0x80, 0x80),
//         "blue":    (0x00, 0x00, 0xFF),
//         "navy":    (0x00, 0x00, 0x80),
//         "fuchsia": (0xFF, 0x00, 0xFF),
//         "purple":  (0x80, 0x00, 0x80),
//     ]
// }

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

impl Into<(u8, u8, u8)> for Color {
    fn into(self) -> (u8, u8, u8) {
        (self.red, self.green, self.blue)
    }
}

impl Into<[u8; 3]> for Color {
    fn into(self) -> [u8; 3] {
        [self.red, self.green, self.blue]
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
        if s.starts_with("#") && s.len() == 7 {
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

pub(crate) struct ColorStop {
    color: Color,
    position: usize,
}

pub struct RGBGradient {
    duration: Option<Duration>,
    color_stops: Vec<ColorStop>,
}

impl Default for RGBGradient {
    fn default() -> Self {
        Self {
            duration: None,
            color_stops: vec![],
        }
    }
}

impl From<Vec<Color>> for RGBGradient {
    fn from(colors: Vec<Color>) -> Self {
        RGBGradient::new_with_colors(colors).unwrap()
    }
}

impl RGBGradient {
    pub fn new_with_colors(colors: Vec<Color>) -> Result<Self, &'static str> {
        if colors.len() > 14 {
            Err("A maximum of 14 color stops can be defined in a gradient")
        } else {
            let mut stops: Vec<ColorStop> = vec![];
            let mut i: usize = 0;
            for color in colors {
                stops.push(ColorStop {
                    color: color,
                    position: i,
                });
                i += 1;
            }

            Ok(Self {
                duration: None,
                color_stops: stops,
            })
        }
    }

    pub(crate) fn iter_colors(&self) -> Iter<ColorStop> {
        self.color_stops.iter()
    }

    pub fn is_gradient(&self) -> bool {
        self.color_stops.len() > 1
    }

    pub fn process(&self, settings: RGBGradientSettings) -> ProcessedRGBGradient {

        // Generate header

        let mut header: Vec<u8> = vec![];
        for _ in 0..settings.header_length {
            header.push(0x00);
        }

        if self.is_gradient() {
            header[settings.repeat_offset as usize] = 0x01;
        }

        header[settings.color_count_offset as usize] = self.color_stops.len() as u8;

        for led_id_offset in settings.led_id_offsets {
            header[*led_id_offset as usize] = 0x00;
        }

        // Generate body

        let mut body: Vec<u8> = vec![];
        
        body.push(self.color_stops[0].color.red);
        body.push(self.color_stops[0].color.green);
        body.push(self.color_stops[0].color.blue);

        let mut last_real_pos = 0;
        self.iter_colors().for_each(|color| {
            let real_pos = color.position * 255 / 100;
            let color_bytes: [u8; 3] = color.color.clone().into();
            let mut color_bytes: Vec<u8> = color_bytes.into();
            body.append(&mut color_bytes);
            body.push((real_pos - last_real_pos) as u8);
            last_real_pos = real_pos;
        });

        header.append(&mut body);

        ProcessedRGBGradient { gradient: self, bytes: header }
    }
}

impl From<Color> for RGBGradient {
    fn from(source: Color) -> Self {
        Self {
            duration: None,
            color_stops: vec![ColorStop {
                color: source,
                position: 0,
            }],
        }
    }
}

pub struct RGBGradientSettings<'a> {
    pub(crate) header_length: u16,
    pub(crate) led_id_offsets: &'a [u8],
    pub(crate) duration_offset: u8,
    pub(crate) duration_length: u8,
    pub(crate) repeat_offset: u8,
    pub(crate) triggers_offset: u8,
    pub(crate) color_count_offset: u16,
}

pub struct ProcessedRGBGradient<'a> {
    gradient: &'a RGBGradient,
    bytes: Vec<u8>
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

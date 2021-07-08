#[derive(Debug, PartialEq)]
pub struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl Into<(u8, u8, u8)> for Color {
    fn into(self) -> (u8, u8, u8) {
        (self.red, self.green, self.blue)
    }
}

impl From<&str> for Color {
    fn from(s: &str) -> Self {
        if s.starts_with("#") && s.len() == 7 {
            let hex_channels = s.replacen("#", "", 1);
            let hex_channels = hex_channels.trim();
            Self {
                red: u8::from_str_radix(&hex_channels[0..2], 16)
                    .expect("Cant't read RED channel").wrapping_shr(16)
                    & 255u8,
                green: u8::from_str_radix(&hex_channels[2..4], 16)
                    .expect("Cant't read GREEN channel").wrapping_shr(8)
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

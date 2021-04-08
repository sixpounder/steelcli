pub struct Color {
    red: u8,
    green: u8,
    blue: u8
}

impl Into<(u8, u8, u8)> for Color {
    fn into(self) -> (u8, u8, u8) {
        (self.red, self.green, self.blue)
    }
}

impl From<&str> for Color {
    fn from(s: &str) -> Self {
        let parts: Vec<&str> = s.split(',').collect();
        Self {
            red: parts[0].trim().parse::<u8>().expect("Cant't read RED channel"),
            green: parts[1].trim().parse::<u8>().expect("Cant't read GREEN channel"),
            blue: parts[2].trim().parse::<u8>().expect("Cant't read BLUE channel")
        }
    }
}

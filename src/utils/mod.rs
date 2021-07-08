/// Turns a number `x` into a string representation with a given `radix`
pub fn format_radix(mut x: u32, radix: u32) -> String {
    let mut result = vec![];

    loop {
        let m = x % radix;
        x = x / radix;

        // will panic if you use a bad radix (< 2 or > 36).
        result.push(std::char::from_digit(m, radix).unwrap());
        if x == 0 {
            break;
        }
    }
    result.into_iter().rev().collect()
}

/// Parses a device id in the form of `vendor_id:product_id`, where both ids are base16 numbers
pub fn parse_device_id(device_str: &str) -> (u16, u16) {
    let parts = device_str.split(":").collect::<Vec<&str>>(); // eg: 1038:12aa
    let vid = u16::from_str_radix(parts[0], 16).unwrap_or(0);
    let pid = u16::from_str_radix(parts[1], 16).unwrap_or(0);

    (vid, pid)
}

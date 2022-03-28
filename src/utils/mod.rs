use std::ops::{Div, Rem};

pub trait ToStringRadix
where
    Self: Copy + Into<u32> + Rem<u32> + Div<Output = u32> + PartialEq<u32>,
{
    fn to_string_radix(&self, radix: u32) -> String {
        let mut result = vec![];
        let mut x: u32 = self.clone().into();

        loop {
            let m: u32 = x % radix;
            x = x / radix;

            // will panic if you use a bad radix (< 2 or > 36).
            result.push(std::char::from_digit(m, radix).unwrap());
            if x == 0 {
                break;
            }
        }
        result.into_iter().rev().collect()
    }
}

macro_rules! to_string_radix_impl {
    ( $( $t:ty ),* ) => {
        $(
            impl ToStringRadix for $t {}
        )*
    };
}

to_string_radix_impl!(u32);

/// Turns a number `x` into a string representation with a given `radix`
/// 
/// # Panics
/// This will panic if you use a bad radix (< 2 or > 36)
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
    if !device_str.contains(":") {
        return (0, 0);
    }

    let parts = device_str.split(":").collect::<Vec<&str>>(); // eg: 1038:12aa
    let vid = u16::from_str_radix(parts[0], 16).unwrap_or(0);
    let pid = u16::from_str_radix(parts[1], 16).unwrap_or(0);

    (vid, pid)
}

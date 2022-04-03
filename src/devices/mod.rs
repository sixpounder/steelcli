mod arctis_five;
mod sensei_ten;

use super::OUTPUT;
pub use arctis_five::*;
pub use sensei_ten::*;

#[macro_export]
macro_rules! get_profile_value {
    ( $target:ident, $k:literal ) => {
        $target.get_profile_value($k).unwrap()
    };
    ( $target:ident, $k:literal, $t:tt ) => {
        $target.get_profile_value($k).unwrap().$t().unwrap()
    };
}

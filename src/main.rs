#[macro_use]
extern crate clap;

#[macro_use]
extern crate lazy_static;

mod change;
mod device_ops;
mod list;
mod describe;
mod devices;
mod steelseries_core;
mod errors;

use errors::{SteelseriesError, SteelseriesResult};
use steelseries_core::LogLevel;
use change::change;
use list::list;
use clap::App;
use regex::Regex;

const HEX_STR_REGEXP: &str = r"[a-f]";

lazy_static! {
    pub static ref LOGGER: crate::steelseries_core::Log = {
        let l = crate::steelseries_core::Log::new();
        l
    };
}

fn main() -> SteelseriesResult<()> {
    match sudo::escalate_if_needed() {
        Err(_some_error) => {
            return Err(SteelseriesError::Privileges);
        }
        _ => (),
    }

    let yaml = load_yaml!("config/cli.yml");
    let cli = App::from_yaml(yaml);
    let matches = cli.get_matches();
    let mut cli = App::from_yaml(yaml);

    if let 0 = matches.occurrences_of("verbose") + matches.occurrences_of("v") {
        LOGGER.set_level(LogLevel::Normal);
    } else {
        LOGGER.set_level(LogLevel::Verbose);
    }

    if let Some(_cmd) = matches.subcommand_matches("list") {
        list()
    } else if let Some(cmd) = matches.subcommand_matches("describe") {
        let device = device_from_arg(cmd.value_of("device").unwrap_or("0:0"));
        describe::describe(device.0, device.1)

    } else if let Some(cmd) = matches.subcommand_matches("change") {
        let device = device_from_arg(cmd.value_of("device").unwrap_or("0:0"));
        let prop = cmd.value_of("PROPERTY").unwrap_or("");
        let value = cmd.value_of("VALUE").unwrap_or("");

        change(device.0, device.1, prop, value)
    } else {
        cli.print_help().unwrap();
        Ok(())
    }
}

fn device_from_arg(device_str: &str) -> (u16, u16) {
    let regex = Regex::new(HEX_STR_REGEXP).unwrap();
    let parts = device_str.split(":").collect::<Vec<&str>>(); // eg: 1038:12aa
    let vid_radix = match regex.is_match(parts[0]) { true => 16, false => 10 };
    let pid_radix = match regex.is_match(parts[1]) { true => 16, false => 10 };
    let f_radix = match vid_radix == 16 || pid_radix == 16 { true => 16, false => 10 };

    let vid = u16::from_str_radix(parts[0], f_radix).unwrap_or(0);
    let pid = u16::from_str_radix(parts[1], f_radix).unwrap_or(0);

    (vid, pid)
}

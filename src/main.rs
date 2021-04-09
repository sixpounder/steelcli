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
mod utils;

use errors::{SteelseriesError, SteelseriesResult};
use steelseries_core::LogLevel;
use change::change;
use list::list;
use clap::App;

lazy_static! {
    pub static ref LOGGER: crate::steelseries_core::Log = {
        let l = crate::steelseries_core::Log::new();
        l
    };
}

fn main() -> SteelseriesResult<()> {
    let yaml = load_yaml!("config/cli.yml");
    let mut cli = App::from_yaml(yaml);

    // Clone "cli" to reuse it later
    let matches = cli.clone().get_matches();

    if matches.occurrences_of("escalate") + matches.occurrences_of("e") != 0 {
        match sudo::escalate_if_needed() {
            Err(_some_error) => {
                return Err(SteelseriesError::Privileges);
            }
            _ => (),
        }
    }

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
    let parts = device_str.split(":").collect::<Vec<&str>>(); // eg: 1038:12aa
    let vid = u16::from_str_radix(parts[0], 16).unwrap_or(0);
    let pid = u16::from_str_radix(parts[1], 16).unwrap_or(0);

    (vid, pid)
}

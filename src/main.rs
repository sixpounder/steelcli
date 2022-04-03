#[macro_use]
extern crate clap;

#[macro_use]
extern crate lazy_static;

mod change;
mod describe;
mod devices;
mod list;
mod steelseries_core;
mod utils;
// mod runner;

use change::change;
use clap::{App, ArgMatches};
use list::{list, list_all};
use steelseries_core::{support::DevicePool, LogLevel, SteelseriesDevice, Error, Result, TaskOptions};

lazy_static! {
    pub static ref OUTPUT: crate::steelseries_core::Log = crate::steelseries_core::Log::new();
}

lazy_static! {
    pub static ref HIDAPI: hidapi::HidApi = hidapi::HidApi::new().unwrap();
}

fn main() -> Result<()> {
    let yaml = load_yaml!("config/cli.yml");
    let mut cli = App::from_yaml(yaml);

    // Preallocate a synched device pool to use later
    let device_pool = DevicePool::new().sync();

    // Clone "cli" to reuse it later
    let matches = cli.clone().get_matches();

    let dry = std::env::vars()
        .find(|v| v.0 == "STEELCLI_DRY" && v.1 == "1")
        .is_some()
        || matches.occurrences_of("dry") != 0;

    if matches.occurrences_of("escalate") + matches.occurrences_of("e") != 0 {
        if let Err(_some_error) = sudo::escalate_if_needed() {
            return Err(Error::Privileges);
        }
    }

    if let 0 = matches.occurrences_of("verbose") + matches.occurrences_of("v") {
        OUTPUT.set_level(LogLevel::Normal);
    } else {
        OUTPUT.set_level(LogLevel::Verbose);
    }

    if let Some(_cmd) = matches.subcommand_matches("list") {
        list(&device_pool)
    } else if let Some(_cmd) = matches.subcommand_matches("supported") {
        list_all()
    } else if let Some(cmd) = matches.subcommand_matches("describe") {
        if let Some(device) = device_from_args(&device_pool, cmd) {
            describe::describe(device)
        } else {
            Err(Error::NoDevice)
        }
    } else if let Some(cmd) = matches.subcommand_matches("change") {
        let device = device_from_args(&device_pool, cmd);
        let options = TaskOptions {
            dry,
            save: matches.occurrences_of("save") != 0
        };

        match device {
            Some(device) => {
                let prop = cmd.value_of("PROPERTY").unwrap_or("");
                let value = cmd.value_of("VALUE").unwrap_or("");
                change(device, prop, value, &options)
            }
            None => {
                OUTPUT.error("No device specified");
                Err(Error::NoDevice)
            }
        }
    } else {
        cli.print_help().unwrap();
        Ok(())
    }
}

fn device_from_args<'a>(
    device_pool: &'a DevicePool,
    cmd: &'a ArgMatches,
) -> Option<&'a dyn SteelseriesDevice> {
    let device_spec_arg = cmd.value_of("device");
    match device_spec_arg {
        Some(device_spec) => {
            if device_spec.contains(":") {
                let parsed = utils::parse_device_id(cmd.value_of("device").unwrap_or("0:0"));
                if let Some(device) = device_pool.find_one(parsed.0, parsed.1) {
                    Some(device)
                } else {
                    None
                }
            } else {
                if let Some(device) = device_pool.find_by_slug(device_spec) {
                    Some(device)
                } else {
                    None
                }
            }
        }
        None => None,
    }
}

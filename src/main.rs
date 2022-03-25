#[macro_use]
extern crate clap;

#[macro_use]
extern crate lazy_static;

mod change;
mod describe;
mod device_ops;
mod devices;
mod errors;
mod list;
mod steelseries_core;
mod utils;

use change::change;
use clap::{App, ArgMatches};
use errors::{SteelseriesError, SteelseriesResult};
use list::list;
use steelseries_core::{support::DevicePool, LogLevel, SteelseriesDevice};

lazy_static! {
    #[allow(non_upper_case_globals)]
    pub static ref OUTPUT: crate::steelseries_core::Log = {
        let l = crate::steelseries_core::Log::new();
        l
    };
}

lazy_static! {
    pub static ref HIDAPI: hidapi::HidApi = hidapi::HidApi::new().unwrap();
}

fn main() -> SteelseriesResult<()> {
    let yaml = load_yaml!("config/cli.yml");
    let mut cli = App::from_yaml(yaml);
    let device_pool = DevicePool::new().sync();

    // Clone "cli" to reuse it later
    let matches = cli.clone().get_matches();

    if matches.occurrences_of("escalate") + matches.occurrences_of("e") != 0 {
        if let Err(_some_error) = sudo::escalate_if_needed() {
            return Err(SteelseriesError::Privileges);
        }
    }

    if let 0 = matches.occurrences_of("verbose") + matches.occurrences_of("v") {
        OUTPUT.set_level(LogLevel::Normal);
    } else {
        OUTPUT.set_level(LogLevel::Verbose);
    }

    if let Some(_cmd) = matches.subcommand_matches("list") {
        list()
    } else if let Some(cmd) = matches.subcommand_matches("describe") {
        let device = utils::parse_device_id(cmd.value_of("device").unwrap_or("0:0"));
        if device != (0, 0) {
            describe::describe(device.0, device.1)
        } else {
            describe::by_slug(cmd.value_of("device").unwrap_or(""))
        }
    } else if let Some(cmd) = matches.subcommand_matches("change") {
        let device = device_from_args(&device_pool, cmd);
        match device {
            Some(device) => {
                let prop = cmd.value_of("PROPERTY").unwrap_or("");
                let value = cmd.value_of("VALUE").unwrap_or("");
                change(device, prop, value)
            },
            None => {
                OUTPUT.error("No device specified");
                Err(SteelseriesError::NoDevice)
            }
        }
    } else {
        cli.print_help().unwrap();
        Ok(())
    }
}

fn device_from_args<'a>(device_pool: &'a DevicePool, cmd: &'a ArgMatches) -> Option<&'a dyn SteelseriesDevice> {
    let device_spec_arg = cmd.value_of("device");
    match device_spec_arg {
        Some(device_spec) => {
            let device;
            if device_spec.contains(":") {
                let parsed = utils::parse_device_id(cmd.value_of("device").unwrap_or("0:0"));
                device = device_pool
                    .find_one(parsed.0, parsed.1)
                    .expect("No supported device found");
            } else {
                device = device_pool
                    .find_by_slug(device_spec)
                    .expect("No supported device found");
            }

            Some(device)
        }
        None => {
            None
        }
    }
}
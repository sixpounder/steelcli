#[macro_use]
extern crate clap;

mod change;
mod device_ops;
mod list;
mod devices;

use crate::change::change;
use crate::list::list;
use clap::App;
use rusb::Result;

fn main() -> Result<()> {
    match sudo::escalate_if_needed() {
        _ => (),
    }

    let yaml = load_yaml!("config/cli.yml");
    let cli = App::from_yaml(yaml);
    let matches = cli.get_matches();
    let mut cli = App::from_yaml(yaml);

    if let Some(_cmd) = matches.subcommand_matches("list") {
        list()
    } else if let Some(cmd) = matches.subcommand_matches("change") {
        let device_str = cmd.value_of("device").unwrap_or("0:0");
        let parts = device_str.split(":").collect::<Vec<&str>>(); // eg: 1038:12aa

        let vid = u16::from_str_radix(parts[1], 16).unwrap_or(0);
        let pid = u16::from_str_radix(parts[0], 16).unwrap_or(0);
        let prop = cmd.value_of("PROPERTY").unwrap_or("");
        let value = matches.value_of("VALUE").unwrap_or("");

        change(pid, vid, prop, value)
    } else {
        cli.print_help().unwrap();
        Ok(())
    }
}

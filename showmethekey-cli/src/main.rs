mod interface;

use std::process;
use anyhow::Result;
use clap::{Command, Arg};
use input::Libinput;

#[inline]
fn is_root() -> bool {
    return nix::unistd::geteuid().is_root();
}

fn main() -> Result<()> {
    let args = Command::new("showmethekey-cli-rs")
        .version(env!("CARGO_PKG_VERSION"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::new("seat")
                .short('s')
                .value_name("SEAT_ID")
                .default_value("seat0")
                .help("Assign a custom seat to the libinput context")
        )
        .get_matches();

    if !is_root() {
        println!("Please run me as root!");
        process::exit(1);
    }

    let seat_id = args.get_one::<String>("seat").unwrap();

    let mut input = Libinput::new_with_udev(interface::Interface);
    match input.udev_assign_seat(seat_id) {
        Ok(_) => interface::run_eventloop(&mut input).unwrap(),
        Err(_) => eprintln!("Failed to set seat."),
    }

    Ok(())
}

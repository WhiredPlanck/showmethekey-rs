mod interface;

use std::process;
use anyhow::Result;
use clap::{Command, Arg};
use input::Libinput;

#[inline]
fn is_root() -> bool {
    return nix::unistd::geteuid().is_root();
}

fn handle_input() {
    let mut line = String::new();
    while let Ok(_) = std::io::stdin().read_line(&mut line) {
        if line == "stop\n" {
            process::exit(0);
        }
    }
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
        Ok(_) => {
	        // Typically this will be run with pkexec as a subprocess,
	        // and the parent cannot kill it because it is privileged,
	        // so we use another thread to see if it gets "stop\n" from stdin,
	        // it will exit by itself.
            std::thread::spawn(|| {
                handle_input();
            });

            interface::run_eventloop(&mut input).unwrap()
        }
        Err(_) => eprintln!("Failed to set seat."),
    }

    Ok(())
}

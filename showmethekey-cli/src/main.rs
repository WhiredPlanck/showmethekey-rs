use std::process;
use anyhow::Result;
use clap::Command;
use input::Libinput;

#[inline]
fn is_root() -> bool {
    return nix::unistd::geteuid().is_root();
}

fn main() -> Result<()> {
    Command::new("showmethekey-cli-rs")
        .version(env!("CARGO_PKG_VERSION"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .get_matches();

    if !is_root() {
        println!("Please run me as root!");
        process::exit(1);
    }

    let mut input = Libinput::new_with_udev(showmethekey_cli::Interface);
    match input.udev_assign_seat("seat0") {
        Ok(_) => showmethekey_cli::run_eventloop(&mut input).unwrap(),
        Err(_) => eprintln!("Failed to set seat."),
    }

    Ok(())
}

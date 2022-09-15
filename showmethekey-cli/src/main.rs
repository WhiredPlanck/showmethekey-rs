use std::process;
use anyhow::Result;
use clap::Command;
use input::Libinput;

#[inline]
fn is_root() -> bool {
    return nix::unistd::geteuid().is_root();
}

fn main() -> Result<()> {
    Command::new("showmethekey-cli")
        .version(env!("CARGO_PKG_VERSION"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .get_matches();

    if !is_root() {
        println!("Please run me as root!");
        process::exit(1);
    }

    let mut input = Libinput::new_with_udev(showmethekey_cli::Interface);
    input.udev_assign_seat("seat0").unwrap();

    showmethekey_cli::run_eventloop(&mut input)?;

    Ok(())
}

use std::{os::unix::prelude::{RawFd, OpenOptionsExt, IntoRawFd}, path::Path, fs::OpenOptions};
use anyhow::Result;
use input::LibinputInterface;
use nix::unistd::close;

struct Interface;

impl LibinputInterface for Interface {
    fn open_restricted(&mut self, path: &Path, flags: i32) -> Result<RawFd, i32> {
        OpenOptions::new()
            .custom_flags(flags)
            .open(path)
            .map(|file| file.into_raw_fd())
            .map_err(|err| err.raw_os_error().unwrap())
    }

    fn close_restricted(&mut self, fd: RawFd) {
        close(fd).unwrap();
    }
}

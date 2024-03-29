use anyhow::Result;
use input::{
    event::{
        keyboard::{KeyState, KeyboardEventTrait},
        pointer::{ButtonState, PointerEventTrait},
        EventTrait, PointerEvent,
    },
    Event, Libinput, LibinputInterface,
};
use nix::{
    libc::{O_RDONLY, O_RDWR, O_WRONLY},
    poll::{poll, PollFd, PollFlags},
};
use serde_json::json;
use std::{
    fs::{File, OpenOptions},
    os::{unix::prelude::{AsRawFd, OpenOptionsExt}, fd::OwnedFd},
    path::Path,
};

pub struct Interface;

impl LibinputInterface for Interface {
    fn open_restricted(&mut self, path: &Path, flags: i32) -> Result<OwnedFd, i32> {
        OpenOptions::new()
            .custom_flags(flags)
            .read((flags & O_RDONLY != 0) | (flags & O_RDWR != 0))
            .write((flags & O_WRONLY != 0) | (flags & O_RDWR != 0))
            .open(path)
            .map(|file| file.into())
            .map_err(|err| err.raw_os_error().unwrap())
    }

    #[allow(unused_must_use)]
    fn close_restricted(&mut self, fd: OwnedFd) {
        File::from(fd);
    }
}

fn print_key_and_button_event(event: &Event) -> Result<()> {
    match event {
        Event::Keyboard(ev) => {
            let key_code = ev.key() as u16;
            let key_state = ev.key_state();
            let state_code: i32 = match &key_state {
                KeyState::Pressed => 1,
                KeyState::Released => 0,
            };
            let key_event = json!({
                "event_name": "KEYBOARD_KEY",
                "device_name": ev.device().name(),
                "time_stamp": ev.time(),
                "key_code": key_code,
                "key_name": format!("{:?}", evdev::Key::new(key_code)),
                "state_code": state_code,
                "state_name": format!("{:?}", key_state),
            });
            println!("{}", serde_json::to_string_pretty(&key_event)?);
        }
        Event::Pointer(ev) => {
            if let PointerEvent::Button(e) = ev {
                let btn_code = e.button() as u16;
                let btn_state = e.button_state();
                let state_code: i32 = match &btn_state {
                    ButtonState::Pressed => 1,
                    ButtonState::Released => 0,
                };
                let btn_event = json!({
                    "event_name": "POINTER_BUTTON",
                    "device_name": e.device().name(),
                    "time_stamp": e.time(),
                    "key_code": btn_code,
                    "key_name": format!("{:?}", evdev::Key::new(btn_code)),
                    "state_code": state_code,
                    "state_name": format!("{:?}", btn_state),
                });
                println!("{}", serde_json::to_string_pretty(&btn_event)?);
            }
        }
        _ => (),
    }

    Ok(())
}

pub fn run_eventloop(input: &mut Libinput) -> Result<()> {
    let pollfd = PollFd::new(input.as_raw_fd(), PollFlags::POLLIN);
    while poll(&mut [pollfd], -1).is_ok() {
        input.dispatch().unwrap();
        for event in &mut *input {
            print_key_and_button_event(&event)?;
        }
    }

    Ok(())
}

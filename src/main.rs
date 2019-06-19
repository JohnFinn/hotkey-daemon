#[macro_use]
extern crate num_derive;

use std::fs::File;
use std::io::prelude::*;
use std::mem::size_of;
use std::mem::transmute;
use std::os::unix::io::AsRawFd;
use std::time::Duration;

use crate::input_event_to_enum::{convert, InputEvent};

mod input_event_to_enum;

struct Events {
    file: File
}

impl Iterator for Events {
    type Item = (Duration, InputEvent);

    fn next(&mut self) -> Option<Self::Item> {
        let mut buffer= [0u8; size_of::<libc::input_event>()];
        match self.file.read_exact(&mut buffer) {
            Err(_) => None,
            Ok(_) => {
                // actually safe since we declared with same size
                input_event_to_enum::convert(unsafe { transmute(buffer) })
            }
        }
    }
}

fn main() -> std::io::Result<()> {
    let file = File::open("/dev/input/by-path/platform-i8042-serio-0-event-kbd")?;
    let events = Events{ file };
    for (_d, ev) in events {
        if let input_event_to_enum::InputEvent::KEY(action) = ev {
            println!("{:?}", action);
        }
    }
    Ok(())
}

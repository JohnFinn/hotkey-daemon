#[macro_use]
extern crate num_derive;

use std::fs::File;
use std::io::prelude::*;
use std::mem::size_of;
use std::mem::transmute;
use std::time::Duration;
use std::process;

mod input_event_to_enum;
mod key_combo_handler;
use key_combo_handler::*;

struct Events {
    file: File
}

impl Iterator for Events {
    type Item = (Duration, InputEvent);

    fn next(&mut self) -> Option<Self::Item> {
        let mut buffer = [0u8; size_of::<libc::input_event>()];
        match self.file.read_exact(&mut buffer) {
            Err(_) => None,
            Ok(_) => {
                // actually safe since we declared with same size
                input_event_to_enum::convert(unsafe { transmute(buffer) })
            }
        }
    }
}

fn main() {
    let mut config = HashMap::<BTreeSet<Key>, &dyn Fn()>::new();
    config.insert(
        [Key::BRIGHTNESSUP].iter().cloned().collect(),
        &|| println!("brightnellup")
    );
    config.insert(
        [Key::LEFTMETA, Key::ENTER].iter().cloned().collect(),
        &|| {
            println!("super+enter");
            process::Command::new("termite").spawn();
        }
    );

    let file = File::open("/dev/input/by-path/platform-i8042-serio-0-event-kbd").unwrap();
    let mut key_events = Events{file}.filter_map(|(_, ev)| match ev {
        InputEvent::KEY(key_ev) => Some(key_ev),
        _ => None
    });
    // key_events.for_each(|ev| println!("{:?}", ev));
    let mut ksh = KeyComboHandler::new(config);
    ksh.handle_all(&mut key_events);
}

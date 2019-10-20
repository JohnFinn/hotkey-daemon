#[macro_use]
extern crate num_derive;

use std::fs::File;
use std::io::prelude::*;
use std::mem::size_of;
use std::mem::transmute;
use std::time::Duration;

use crate::input_event_to_enum::{convert, InputEvent, KeyEvent, Key};
use std::collections::{HashSet, HashMap, BTreeSet};
use crate::input_event_to_enum::InputEvent::KEY;

mod input_event_to_enum;

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

struct KeyComboHandler<'a> {
    config: HashMap<BTreeSet<Key>, &'a dyn Fn()>,
    pressed: BTreeSet<Key>
}

impl<'a> KeyComboHandler<'a> {

    fn new(config: HashMap<BTreeSet<Key>, &'a dyn Fn()>) -> KeyComboHandler {
        KeyComboHandler {
            config,
            pressed: BTreeSet::new()
        }
    }

    fn handle_one(&mut self, event: KeyEvent) {
        match event {
            KeyEvent::Autorepeat(_) => {},
            KeyEvent::Release(key) => { self.pressed.remove(&key); },
            KeyEvent::Press(key) => {
                self.pressed.insert(key);
                if let Some(func) = self.config.get(&self.pressed) {
                    func();
                }
            }
        }
    }

    fn handle_all(&mut self, events: &mut dyn Iterator<Item=(KeyEvent)>) {
        events.for_each(|ev| self.handle_one(ev));
    }
}

fn main() -> std::io::Result<()> {
    let mut config = HashMap::<BTreeSet<Key>, &dyn Fn()>::new();
    config.insert([Key::BRIGHTNESSUP].iter().cloned().collect(), &|| println!("brightnellup"));
    let mut ksh = KeyComboHandler::new(config);
    let file = File::open("/dev/input/by-path/platform-i8042-serio-0-event-kbd")?;
    let mut key_events = Events{file}.filter_map(|(_, ev)| match ev {
        InputEvent::KEY(key_ev) => Some(key_ev),
        _ => None
    });
    ksh.handle_all(&mut key_events);
    Ok(())
}


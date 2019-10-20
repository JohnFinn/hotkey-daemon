pub use std::collections::{HashSet, HashMap, BTreeSet};
pub use crate::input_event_to_enum::{convert, InputEvent, KeyEvent, Key};
pub use crate::input_event_to_enum::InputEvent::KEY;


pub struct KeyComboHandler<'a> {
    config: HashMap<BTreeSet<Key>, &'a dyn Fn()>,
    pressed: BTreeSet<Key>
}

impl<'a> KeyComboHandler<'a> {

    pub fn new(config: HashMap<BTreeSet<Key>, &'a dyn Fn()>) -> KeyComboHandler {
        KeyComboHandler {
            config,
            pressed: BTreeSet::new()
        }
    }

    fn handle_binding(&self) {
        if let Some(func) = self.config.get(&self.pressed) {
            func();
        }
    }

    fn handle_one(&mut self, event: KeyEvent) {
        match event {
            KeyEvent::Autorepeat(_) => { self.handle_binding(); },
            KeyEvent::Release(key) => { self.pressed.remove(&key); },
            KeyEvent::Press(key) => {
                self.pressed.insert(key);
                self.handle_binding();
            }
        }
    }

    pub fn handle_all(&mut self, events: &mut dyn Iterator<Item=(KeyEvent)>) {
        events.for_each(|ev| self.handle_one(ev));
    }
}

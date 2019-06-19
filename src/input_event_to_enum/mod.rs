extern crate num_traits;

use std::convert::TryFrom;
use std::time::Duration;

use num_traits::cast::*;

pub use keys::Key;

use crate::input_event_to_enum::InputEvent::*;
use crate::input_event_to_enum::KeyEvent::*;

mod keys;

pub fn convert(ev: libc::input_event) -> Option<(Duration, InputEvent)> {
    Some((timeval_to_duration(ev.time), InputEvent::from_input_event(ev)?))
}

#[derive(Debug)]
pub enum InputEvent {
    SYN(SynchronizationEvent), // 0x00
    KEY(KeyEvent),             // 0x01
    REL,                       // 0x02
    ABS,                       // 0x03
    MSC(MiscEvent),            // 0x04
    SW,                        // 0x05
    LED,                       // 0x11
    SND,                       // 0x12
    REP,                       // 0x14
    FF,                        // 0x15
    PWR,                       // 0x16
    FF_STATUS,                 // 0x17
}

impl InputEvent {
    fn from_input_event(value: libc::input_event) -> Option<Self> {
        match value.type_ {
            0x00 => Some(SYN(FromPrimitive::from_u16(value.code)?)),
            0x01 => {
                let kp = FromPrimitive::from_u16(value.code)?;
                match value.value {
                    0 => Some(KEY(Release(kp))),
                    1 => Some(KEY(Press(kp))),
                    2 => Some(KEY(Autorepeat(kp))),
                    _ => None,
                }
            }
            0x02 => Some(REL),
            0x03 => Some(ABS),
            0x04 => Some(MSC(FromPrimitive::from_u16(value.code)?)),
            0x05 => Some(SW),
            0x11 => Some(LED),
            0x12 => Some(SND),
            0x14 => Some(REP),
            0x15 => Some(FF),
            0x16 => Some(PWR),
            0x17 => Some(FF_STATUS),
            _ => None,
        }
    }
}

#[derive(Debug, FromPrimitive)]
pub enum MiscEvent {
    SERIAL		= 0x00,
    PULSELED	= 0x01,
    GESTURE		= 0x02,
    RAW			= 0x03,
    SCAN		= 0x04,
    TIMESTAMP	= 0x05,
}

#[derive(Debug, FromPrimitive)]
pub enum SynchronizationEvent {
    REPORT		= 0,
    CONFIG		= 1,
    MT_REPORT   = 2,
    DROPPED		= 3,
}

#[derive(Debug)]
pub enum KeyEvent {
    Release(Key),    // 0
    Press(Key),      // 1
    Autorepeat(Key), // 2
}

fn timeval_to_duration(t: libc::timeval) -> Duration {
    Duration::new(t.tv_sec as u64, t.tv_usec as u32 * 1000)
}

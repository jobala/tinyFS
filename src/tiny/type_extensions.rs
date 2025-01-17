use std::time::{SystemTime, UNIX_EPOCH};

use time::Timespec;

pub trait TinyTimespec {
    fn to_timespec(&self) -> Timespec;
}

impl TinyTimespec for SystemTime {
    fn to_timespec(&self) -> Timespec {
        match self.duration_since(UNIX_EPOCH) {
            Ok(dur) => Timespec {
                sec: dur.as_secs() as i64,
                nsec: dur.subsec_nanos() as i32,
            },
            Err(err) => {
                let dur = err.duration();
                Timespec {
                    sec: -(dur.as_secs() as i64),
                    nsec: -(dur.subsec_nanos() as i32),
                }
            }
        }
    }
}

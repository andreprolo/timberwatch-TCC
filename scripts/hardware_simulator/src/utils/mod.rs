use std::{thread, time};

pub mod io;
pub mod noise;

pub fn sleep(millis: u64) {
    thread::sleep(time::Duration::from_millis(millis));
}

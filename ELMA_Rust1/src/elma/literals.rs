use std::time::*;


pub fn _s(seconds: u64) -> Duration {
	Duration::from_secs(seconds)
}

pub fn _ms(seconds: u64) -> Duration {
	Duration::from_millis(seconds)
}

pub fn _us(seconds: u64) -> Duration {
	Duration::from_micros(seconds)
}
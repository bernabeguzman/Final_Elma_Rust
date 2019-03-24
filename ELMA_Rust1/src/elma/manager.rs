use std::*;
use super::process::*;
use super::channel::*;
use super::event::*;

pub struct Manager {
	// _processes: Vec<T>,
	// _channels: iter::Map<String, Channel>,
	// event_handlers: iter::Map<String, Vec<Event>>,
	_start_time: time::SystemTime,
	_elapsed: time::Duration,
	// _client: Client,
	_running: bool,
	_simulated_time: bool,
}

impl Manager {

	pub fn new() -> Manager {
		Manager {
			_start_time: time::SystemTime::now(),
			_elapsed: time::Duration::new(0,0),
			_running: false,
			_simulated_time: false,
 		}
	}
	
}
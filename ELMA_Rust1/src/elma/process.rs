use super::transition::*;
use super::state::*;
use std::*;

pub enum status_type { UNINITIALIZED, STOPPED, RUNNING }

pub struct Process {
	_name: String,
	_status: status_type,
	_period: time::Duration,
	_previous_update: time::Duration,
	_last_update: time::Duration,
	_start_time: time::SystemTime,
	_num_updates: i32,
	_priority: i32,
	// _manager_ptr: Manager,
}

impl Process {
	pub fn new(name: &str) -> Process {
		Process {
			_name: name.to_string(),
			_status: status_type::UNINITIALIZED,
			_period: time::Duration::new(0,0),
			_previous_update: time::Duration::new(0,0),
			_last_update: time::Duration::new(0,0),
			_start_time: time::SystemTime::now(),
			_num_updates: 0i32,
			_priority: 0i32,
			// _manager_ptr: Manager,
 		}
	}
	
	pub fn name(&self) -> String {
		self._name.to_string()
	}
	
	pub fn status(&self) -> &status_type {
		&self._status
	}

}
extern crate serde_json;
extern crate throw;

use std::collections::VecDeque;
use serde_json::*;

pub struct Channel {
	_name: String,
	_capacity: i32,
	_queue: VecDeque<serde_json::Value>,
}

impl Channel {

	// pub fn new(name: &str) -> Channel {
		// Channel {
			// _name: name.to_string(),
			// _capacity: 100i32,
			// _queue: VecDeque::new(),
 		// }
	// }
	pub fn new(name: &str, capacity: i32) -> Channel {
		Channel {
			_name: name.to_string(),
			_capacity: capacity,
			_queue: VecDeque::new(),
 		}
	}
	
	pub fn send(&mut self, j: serde_json::Value) -> &mut Channel {
		self._queue.push_front(j);
		while self.size() > self.capacity() {
			self._queue.pop_back();
		}
		self
	}
	
	pub fn flush(&mut self) -> &mut Channel {
		self._queue.clear();
		self
	}
	
	pub fn latest(&mut self) -> Option<serde_json::Value> {//serde_json::Value {
		// if self.empty() {
			// throw string("Tried to get the latest value in an empty channel.");
		// }
		self._queue.pop_front()
	}
	
	pub fn earliest(&mut self) -> Option<serde_json::Value> {//serde_json::Value {
		// if self.empty() {
			// throw_new!("Tried to get the earliest value in an empty channel.");
		// }
		self._queue.pop_back()
	}
	
	pub fn size(&self) -> i32 {
		self._queue.len() as i32
	}
	
	pub fn empty(&self) -> bool {
		self._queue.is_empty()
	}
	
	pub fn nonempty(&self) -> bool {
		!self._queue.is_empty()
	}
	
	pub fn name(&self) -> String {
		self._name.to_string()
	}
	
	pub fn capacity(&self) -> i32 {
		self._queue.capacity() as i32
	}
}

// #[test] 
// fn channel_constructor() {
	// let mut chan = Channel::new();
	// let v1: Value = serde_json::from_str(r#"{"name": "Joe"}"#)?;
	// let v2: Value = serde_json::from_str(r#"{"name": "Joe"}"#)?;
	// let v3: Value = serde_json::from_str(r#"{"name": "Joe"}"#)?;
	// assert!(chan.empty());
	// chan.send(v1);
	// assert!(chan.nonempty());
	// assert_eq!(0, chan.size());
	// chan.latest();
	// assert!(chan.nonempty());
// }
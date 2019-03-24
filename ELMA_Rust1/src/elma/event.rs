extern crate serde_json;

use serde_json::*;

pub struct Event {
	_value: serde_json::Value,
	_propagate: bool,
	_empty: bool,
	_name: String,
}

impl Event {

	pub fn new(name: &str) -> Event {
		Event {
			_value: json!(null),
			_propagate: true,
			_empty: true,
			_name: name.to_string(),
 		}
	}
	
	pub fn value(&self) -> &serde_json::Value {
		&self._value
	}
	
	pub fn empty(&self) -> bool {
		self._empty
	}
	
	pub fn name(&self) -> String {
		self._name.to_string()
	}
	
	pub fn propagate(&self) -> bool {
		self._propagate
	}
	
	pub fn stop_propagation(&mut self) {
		self._propagate = false;
	}
	
	pub fn reset(&mut self) {
		self._propagate = true;
	}
}
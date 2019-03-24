extern crate serde_json;

use std::*;
use serde_json::*;

pub struct Client {
	_handler: String,
	_get_aux: ptr::Shared<T>,
	// _responses: Vec<tuple<serde_json::Value, &fn(serde_json::Value)>>,
	_use_ssl: bool,
	_mtx: sync::Mutex<T>,
}

impl Client {
	pub fn new(name: &str) -> Client {
		Client {
			_handler: name.to_string(),
			_get_aux: ptr::Shared::null,
			// _responses: Vec<tuple<serde_json::Value, &fn(serde_json::Value)>>,
			_use_ssl: false,
			_mtx: sync::Mutex::new(0),
 		}
	}

}
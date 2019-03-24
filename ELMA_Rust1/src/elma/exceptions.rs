use std::*;

pub struct Exception {
	_what: String,
}

impl Exception {
	pub fn new(what: &str) -> Exception {
		Exception {
			_what: what.to_string(),
 		}
	}
	
	pub fn what(&self) -> String {
		self._what.to_string()
	}

}
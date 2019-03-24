// mod state;

use super::state::*;

pub struct Transition<'a> {
	_from: &'a State,
	_to: &'a State,
	_event_name: String,
	
}

impl<'a> Transition<'a> {
	pub fn new(event_name: &str, from: &'a State, to: &'a State) -> Transition<'a> {
		Transition {
			_from: from, 
			_to: to,
			_event_name: event_name.to_string(),
 		}
	}
	
	pub fn from(&self) -> &State {
		self._from
	}
	
	pub fn to(&self) -> &State {
		self._to
	}
	
	pub fn event_name(&self) -> String {
		self._event_name.to_string()
	}

}
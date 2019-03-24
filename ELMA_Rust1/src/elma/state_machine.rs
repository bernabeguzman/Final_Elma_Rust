// mod transition;

use super::transition::*;
use super::state::*;
use std::*;

pub struct StateMachine {
	// _transitions: Vec<Transition>,
	_initial: State,
	_current: State,
	_propagate: bool,
}

impl StateMachine {
	pub fn new() -> StateMachine {
		StateMachine {
			// _transitions: Vec::new(),
			_initial: State::new("null"),
			_current: State::new("null"),
			_propagate: false,
 		}
	}

	pub fn propagate(&self) -> bool {
		self._propagate
	}
}
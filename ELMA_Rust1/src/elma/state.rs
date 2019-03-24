// mod state_machine;

use super::state_machine::*;
use std::*;

static _id_counter: i32 = 0;

pub struct State {
	_name: String,
	_id: i32,
	//_id_counter: i32, 
	// _state_machine_ptr: StateMachine,
}

impl State {

	pub fn new(name: &str) -> State {
		State {
			_name: name.to_string(), 
			_id: _id_counter + 1,
			// _state_machine_ptr: StateMachine::new("null"),
 		}
	}
	
	pub fn name(&self) -> String {
		self._name.to_string()
	}
	
	pub fn id(&self) -> i32 {
		self._id
	}
	
	pub fn during() {}
	
	// pub fn entry(e: &Event) {}
	
	// pub fn exit(e: &Event) {}
	
	// pub fn emit(e: &Event) {}
	
	// pub fn state_machine(&self) -> &StateMachine {
		// //let sm : StateMachine = &self._state_machine_ptr;
		// &self._state_machine_ptr
	// }

}
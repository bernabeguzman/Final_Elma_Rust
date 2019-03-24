extern crate serde_json;

// mod channel;
// mod client;
// mod event;
// mod exceptions;
// mod literals;
// mod manager;
// mod process;
// pub mod state_machine;
// pub mod state;
// pub mod transition;
mod elma;


//use process::*;
//use bindgen::builder;
use elma::channel::*;
use elma::state_machine::*;
use elma::state::*;
use elma::transition::*;
use elma::literals::*;
use elma::event::*;
use elma::process::*;
use elma::manager::*;
use elma::exceptions::*;
use serde_json::*;
use std::*;

fn main() {
    // Tell cargo to tell rustc to link the system bzip2
    // shared library.
    println!("cargo:rustc-link-lib=bz2");
}

#[test] 
fn channel_test() {
	let mut chan = Channel::new("chan0", 100i32);
	let v1: Value = json!({"name" : "Joe"});
	let v2: Value = json!({"name" : "Joe"});
	let v3: Value = json!({"name" : "Joe"});
	assert!(chan.empty());
	chan.send(v1);
	assert!(chan.nonempty());
	assert_eq!(1, chan.size());
	chan.latest();
	assert!(chan.empty());
}

#[test] 
fn state_test() {
	let mut st1 = State::new("state1");
	assert_eq!(1, st1.id());
	assert_eq!("state1", st1.name());
	let mut st2 = State::new("state2");
	assert_eq!(1, st2.id());
	assert_eq!("state2", st2.name());
}

#[test] 
fn state_machine_test() {
	let mut st1 = State::new("state1");
	assert_eq!("state1", st1.name());
	let mut st2 = State::new("state2");
	assert_eq!("state2", st2.name());
	let mut sm1 = StateMachine::new();
	assert!(!sm1.propagate());
}

#[test] 
fn transition_test() {
	let mut st1 = State::new("state1");
	let mut st2 = State::new("state2");
	let mut tran = Transition::new("tran1", &st1, &st2);
	assert_eq!("tran1", tran.event_name());
}

#[test] 
fn literals_test() {
	assert_eq!(time::Duration::from_secs(1000u64), _s(1000u64));
	assert_eq!(time::Duration::from_millis(1000u64), _ms(1000u64));
	assert_eq!(time::Duration::from_micros(1000u64), _us(1000u64));
}

#[test] 
fn event_test() {
	let mut event1 = Event::new("event1");
	assert_eq!(event1.name(), "event1");
	assert!(event1.propagate());
	assert!(event1.empty());
	event1.stop_propagation();
	assert!(!event1.propagate());
}

#[test] 
fn manager_test() {
	let mut mng1 = Manager::new();
}

#[test] 
fn process_test() {
	let mut p1 = Process::new("p1");
	assert_eq!(p1.name(), "p1");
	//assert_eq!((*p1.status()) as i32, status_type::UNINITIALIZED as i32);
}

#[test] 
fn exceptions_test() {
	let mut exp1 = Exception::new("exp1");
	assert_eq!(exp1.what(), "exp1");
	//assert_eq!((*p1.status()) as i32, status_type::UNINITIALIZED as i32);
}
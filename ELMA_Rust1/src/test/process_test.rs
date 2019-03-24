pub trait Quack {
    fn quack(&self);
}

pub struct Process {
	pub is_parrot: bool

}

impl Quack for Process {
	fn quack(&self) {
		if ! self.is_parrot {
			println!("quack");
		} else {
			println!("sqwak");
		}
	}


}
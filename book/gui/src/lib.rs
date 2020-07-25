// gui implementation sample using trait system (from chapter 17)

pub trait Draw {
	fn draw(&self);
}

pub struct Screen {
	pub components: Vec<Box<dyn Draw>>  // can hold everything implements `Draw` trait
}

impl Screen {
	pub fn run(&self) {
		for component in self.components.iter() {
			component.draw();
		}
	}
}

pub struct Button {
	pub width: u32,
	pub height: u32,
	pub label: String
}

impl Draw for Button {
	fn draw(&self) {
		// ...
	}
}


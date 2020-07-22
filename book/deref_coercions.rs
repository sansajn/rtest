// deref coercions sample (chapter 15)
use std::ops::Deref;

struct MyBox<T>(T);  // tuple struct

impl<T> MyBox<T> {
	fn new(x: T) -> MyBox<T> {
		MyBox(x)
	}
}

impl<T> Deref for MyBox<T> {
	type Target = T;

	fn deref(&self) -> &T {
		&self.0
	}
}

fn hello(name: &str) {
	println!("hello, {}!", name);
}

fn main() {
	let m = MyBox::new(String::from("Rust"));
	hello(&m);  // working because String deref implementation returns &str
	println!("done!");
}

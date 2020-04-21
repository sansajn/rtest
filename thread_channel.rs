// thread channel comunication sample

use std::thread;
use std::sync::mpsc;  // Multiple Producer Single Consumer

fn main() {
	let (tx, rx) = mpsc::channel();

	thread::spawn(move || {
		let val = String::from("hi");
		tx.send(val).unwrap();
	});

	let received = rx.recv().unwrap();
	println!("received: {}", received);

	println!("done!");
}

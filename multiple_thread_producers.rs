// multiple thread producers by cloning transmitter sample 

use std::thread;
use std::sync::mpsc;  // Multiple Producer Single Consumer
use std::time::Duration;

fn main() {
	let (tx, rx) = mpsc::channel();

	let tx1 = mpsc::Sender::clone(&tx);
	thread::spawn(move || {
		let vals = vec![
			String::from("hi"),
			String::from("from"),
			String::from("the"),
			String::from("thread")
		];

		for val in vals {
			tx1.send(val).unwrap();
			thread::sleep(Duration::from_millis(250));
		}
	});

	thread::spawn(move || {
		let vals = vec![
			String::from("more"),
			String::from("messages"),
			String::from("for"),
			String::from("you")
		];

		for val in vals {
			tx.send(val).unwrap();
			thread::sleep(Duration::from_millis(250));
		}
	});

	for received in rx {
		println!("received: {}", received);
	}

	println!("done!");
}

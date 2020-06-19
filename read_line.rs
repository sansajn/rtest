use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;

fn main() {
	let f = File::open("book/user.txt").unwrap();
	let mut reader = BufReader::new(f);

	let mut line = String::new();
	reader.read_line(&mut line).unwrap();
	println!("user id '{}'", line);

	println!("done!");
}

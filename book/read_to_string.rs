// read file to string
use std::fs;
use std::io;

fn main() {
	println!("user name is {}", read_username_from_file().unwrap());
}

fn read_username_from_file() -> Result<String, io::Error> {
	fs::read_to_string("user.txt")
}

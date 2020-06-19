use std::fs::File;
use std::io;
use std::io::Read;

fn main() {
	let content = read_username_from_file().unwrap();
	println!("content is {} characters long", content.len());
}

fn read_username_from_file() -> Result<String, io::Error> {
	let mut s = String::new();
	File::open("user.txt")?.read_to_string(&mut s)?;
	Ok(s)
}

use std::fs::File;
use std::io;
use std::io::Read;

fn main() {
	let f = File::open("user.txt");
	let mut f = match f {
		Ok(file) => file,
		Err(e) => panic!("Can't open file")
	};

	let mut s = String::new();
	
	f.read_to_string(&mut s);

	println!("user name is {}", s);
}

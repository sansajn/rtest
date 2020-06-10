// slice sample
// compile with `rustc first_world.rs` command

fn main() {
	let s = String::from("hello world");
	let beg = first_world(&s);
	println!("{}", beg);

	let word = first_world_slice(&s);
	println!("{}", word);
}

fn first_world(s: &String) -> usize {
	for (i, &item) in s.as_bytes().iter().enumerate() {
		if item == b' ' {
			return i;
		}
	}

	s.len()
}

fn first_world_slice(s: &String) -> &str {
	for (i, &item) in s.as_bytes().iter().enumerate() {
		if item == b' ' {
			return &s[0..i];
		}
	}

	&s[..]
}

// lifetime sample from chapter 10

fn main() {
	let s1 = String::from("abcd");
	let s2 = "xyz";
	let result = longest(s1.as_str(), s2);
	println!("The longest strng is {}", result);
}

fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
	if a.len() > b.len() {
		a
	} else {
		b
	}
}

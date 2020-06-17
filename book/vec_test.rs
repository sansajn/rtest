fn main() {
	let mut v = vec![1, 2, 3];
	v.push(4);
	v.push(5);
	v.push(6);

	let first = v[0];
	let first_ref = &v[0];

	println!("first={}", first);
	println!("first_ref={}", first_ref);

	// iteration
	for i in &v {  // i is immutable element reference
		println!("{}", i);
	}

	for i in &mut v {  // i is mutable element reference
		*i += 50;
	}
}

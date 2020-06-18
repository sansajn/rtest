use std::collections::HashMap;

fn main() {
	// insert
	let mut scores = HashMap::new();
	scores.insert(String::from("Blue"), 10);
	scores.insert(String::from("Yellow"), 50);
	println!("{:?}", scores);

	// collect & zip
	let teams = vec![String::from("Blue"), String::from("Yellow")];
	let initial_scores = vec![10, 50];
	let mut scores2: HashMap<_, _> =
		teams.into_iter().zip(initial_scores.into_iter()).collect();
	// teams was moved to scores2 so they are no longer valid there
	println!("{:?}", scores2);

	// access
	let mut scores3 = HashMap::new();
	scores3.insert(String::from("Blue"), 11);
	scores3.insert(String::from("Yellow"), 55);
	for (key, value) in &scores3 {
		println!("{}: {}", key, value);
	}

	// access with get() -> Option<&V>
	match scores3.get("Blue") {
		Some(x) => println!("{}", x),
		_ => ()
	}

	// update
	let mut scores4 = HashMap::new();
	scores4.insert(String::from("Blue"), 10);
	scores4.insert(String::from("Blue"), 25);
	println!("{:?}", scores4);

	// isert if not exists
	scores4.entry(String::from("Yellow")).or_insert(50);
	scores4.entry(String::from("Blue")).or_insert(20);
	println!("{:?}", scores4);

	// insert (if not exists) or update
	let text = "hello world wonderful world";
	let mut map = HashMap::new();
	for word in text.split_whitespace() {
		let count = map.entry(word).or_insert(0);
		*count += 1;
	}
	println!("{:?}", map);
}

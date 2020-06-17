enum SpreadsheetCell {
	Int(i32),
	Float(f64),
	Text(String)
}

fn main() {
	let row = vec![
		SpreadsheetCell::Int(3),
		SpreadsheetCell::Text(String::from("blue")),
		SpreadsheetCell::Float(10.12)
	];


	for cell in &row {
		match cell {
			SpreadsheetCell::Int(x) => println!("{}", x),
			SpreadsheetCell::Float(x) => println!("{}", x),
			SpreadsheetCell::Text(s) => println!("{}", s)
		}
	}

	println!("done");
}

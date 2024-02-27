use rust_sqlite_wrapper::SqliteConnection;

/* `test.db` looks this way

```sqlite
sqlite> .schema test
CREATE TABLE test (number integer primary key, text text);
sqlite> select * from test;
1|one
2|two
3|three
4|four
5|five
``` */

fn main() {
	let conn = SqliteConnection::open("test.db").expect("Failed to open database");
	// Do something with the connection
	println!("Database opened successfully");
	
	// Query and existing rows
	let rows = conn.query_test().expect("Failed to query test table");

	if let Some((last_number, _)) = rows.last() {
		println!("The last number in the database is: {}", last_number);
		if *last_number < 6 {
			// Insert a new row
			conn.insert_into_test(6, "six").expect("Failed to insert into test table");
		}
	}

	// Print existing rows
	for (number, text) in &rows {
		println!("Number: {}, Text: {}", number, text);
	}

	// Connection will be closed automatically when `conn` goes out of scope
}

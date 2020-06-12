#[derive(Debug)]
enum IpAddr {
	V4(u8, u8, u8, u8),
	V6(String)
}

fn main() {
	let home = IpAddr::V4(10, 4, 127, 199);
	let loopback = IpAddr::V6(String::from("::1"));
	println!("{:?}", loopback);
}

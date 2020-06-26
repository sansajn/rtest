#[derive(Debug)]
struct Point<T> {
	x: T, y: T
}

impl<T> Point<T> {
	fn x(&self) -> &T {
		&self.x
	}
}

impl Point<f32> {  // only for f32 Points !
	fn distance_from_origin(&self) -> f32 {
		(self.x.powi(2) + self.y.powi(2)).sqrt()
	}
}

fn main() {
	let p = Point {x:10, y:15};
	println!("{:?}", p);
	println!("x={}", p.x());

	let p2 = Point {x:10.0, y:15.0};
	p2.distance_from_origin();
}

// sample from chapter 13
use std::thread;
use std::time::Duration;
use std::collections::HashMap;

// momeoization or lazy initialization pattern sample
struct Catcher<T>
where
	T: Fn(u32) -> u32
{
	calculation: T,
	value: HashMap<u32, u32>
}

impl<T> Catcher<T>
where
	T: Fn(u32) -> u32
{
	fn new(calculation: T) -> Catcher<T> {
		Catcher {
			calculation,
			value: HashMap::new()
		}
	}

	// make this function generic
	fn value(&mut self, arg: u32) -> u32 {
		match self.value.get(&arg) {
			Some(v) => v.clone(),
			None => {
				let v = (self.calculation)(arg);
				self.value.insert(arg, v);
				v
			}
		}
	}
}

fn main() {
	let simulated_user_specified_value = 10;
	let simulated_random_number = 7;

	generate_workout(simulated_user_specified_value, simulated_random_number);
}

fn generate_workout(intensity: u32, random_number: u32) {
	let mut expensive_result = Catcher::new(|num| {
		println!("calculating slowly...");
		thread::sleep(Duration::from_secs(2));
		num
	});

	if intensity < 25 {
		println!("Today, do {} pushups!", expensive_result.value(intensity));
		println!("Next, do {} situps!", expensive_result.value(intensity));
	} else {
		if random_number == 3 {
			println!("Take a break today! Remember to stay hydrated!");
		} else {
			println!("Today, run for {} minutes!", expensive_result.value(intensity));
		}
	}
}

#[test]
fn call_with_different_values() {
	let mut c = Catcher::new(|a| a);
	let v1 = c.value(1);
	let v2 = c.value(2);
	assert_eq!(v2, 2);
}

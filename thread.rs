use std::thread;

fn main() {
    let t = thread::spawn(|| {
        for i in 1..10 {
            println!("hello {}", i);
        }
    });

    t.join().unwrap();
}
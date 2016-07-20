use std::iter::Iterator;

struct Fibo {

	preprev: i32,
	prev: i32,

}

impl Fibo {
	
	fn new() -> Self {
		Fibo {preprev: 0, prev: 1}
	}
}

impl Iterator for Fibo {

	type Item = i32;

	fn next(&mut self) -> Option<Self::Item> {
		
		let x = self.preprev + self.prev;
		self.preprev = self.prev;
		self.prev = x;

		Some(x)
	}
}

fn main() {

	for i in Fibo::new().take(20) {
		println!("{}", i);
	}
}

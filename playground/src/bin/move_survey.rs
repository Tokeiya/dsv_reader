pub struct Foo<T>(T);

impl<T> Foo<T> {
	pub fn new(value: T) -> Self {
		Self(value)
	}
	
	pub fn move_value(self) -> T {
		self.0
	}
}

fn main() {}
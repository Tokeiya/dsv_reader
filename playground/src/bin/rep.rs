struct Value(i32);

fn main() {
	let mut a = Value(42);

	let b = std::mem::replace(&mut a, Value(0));
}

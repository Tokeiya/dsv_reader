pub trait MyFnOnce {
	fn call_once(self);
}

pub trait MyFnMut: MyFnOnce {
	fn call_mut(&mut self);
}


pub trait MyFn: MyFnMut {
	fn call(&self);
}

impl<T: MyFnMut> MyFnOnce for T {
	fn call_once(mut self) {
		self.call_mut()
	}
}

impl<T: MyFn> MyFnMut for T {
	fn call_mut(&mut self) {
		self.call()
	}
}


fn main() {
	let s = String::from_utf8(Vec::new());
}



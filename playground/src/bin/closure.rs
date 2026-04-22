
pub trait MyFnOnce{
	fn call_once(self);
}

pub trait MyFnMut:MyFnOnce{
	fn call_mut(&mut self);
}


pub trait MyFn:MyFnMut{
	fn call(&self);
}

impl<T: MyFnMut> MyFnOnce for T {
	fn call_once(mut self) {
		self.call_mut()
	}
}

impl<T:MyFn> MyFnMut for T {
	fn call_mut(&mut self) {
		self.call()
	}
}


fn main() {
	immutable(&||println!("hello"));
	mutable(&mut ||println!("hello"));
	once(||println!("hello"))
}



fn immutable<F:Fn()>(f:&F){
	
	for _ in 0..10{
		f();
	}
}

fn mutable<F:FnMut()>(f:&mut F){
	for _ in 0..10{
		f();
	}
}

fn once<F:FnOnce()>(f:F){
	f()
}
use std::error::Error;

pub trait MutConvert<T,E:Error>{
	fn mut_convert(&self,value:&str)->Result<T,E>;
}


pub trait ImmutConvert<T,E:Error>:MutConvert<T,E>{
	fn immut_convert(&self,value:&str)->Result<T,E>;
}

impl<T,E:Error,C:ImmutConvert<T,E>> MutConvert<T,E> for C {
	fn mut_convert(&self, value: &str) -> Result<T, E> {
		self.immut_convert(value)
	}
}

pub struct IntegerConverter;

impl ImmutConvert<i32, std::num::ParseIntError> for IntegerConverter {
	fn immut_convert(&self, value: &str) -> Result<i32, std::num::ParseIntError> {
		value.parse()
	}
}

fn main() {}
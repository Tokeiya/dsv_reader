use std::error::Error;

pub trait Converter:Sized{
	type T;
	type E:Error;
	
	fn convert(value:&str)->Result<Self::T,Self::E>;
}


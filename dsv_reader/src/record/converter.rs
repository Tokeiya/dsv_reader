use std::error::Error;

pub trait Converter
{
	type T;
	type E: Error;
	
	fn convert(&mut self, source: &str) -> Result<Self::T, Self::E>;
	fn optional_convert(&mut self, source: &str) -> Option<Result<Self::T, Self::E>>;
}


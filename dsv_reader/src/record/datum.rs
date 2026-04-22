use crate::record::converter::Converter;

pub struct Datum(String);

impl From<&str> for Datum {
	fn from(source: &str) -> Self {
		todo!()
	}
}

impl From<String> for Datum {
	fn from(value: String) -> Self {
		todo!()
	}
}

impl Datum {
	pub fn source(&self) -> &str {
		todo!()
	}
	
	pub fn convert<C: Converter>(&self, converter: &mut C) -> Result<C::T, C::E> {
		todo!()
	}
	
	pub fn convert_optional<C: Converter>(&self, converter: &mut C) -> Option<Result<C::T, C::E>> {
		todo!()
	}
}
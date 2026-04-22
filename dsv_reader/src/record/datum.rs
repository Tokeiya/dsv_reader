use super::convert::converter::Converter;

pub struct Datum(String);

impl From<&str> for Datum {
	fn from(source: &str) -> Self {
		Datum(source.to_string())
	}
}

impl From<String> for Datum {
	fn from(value: String) -> Self {
		Datum(value)
	}
}

impl Datum {
	pub fn source(&self) -> &str {
		&self.0
	}

	pub fn convert<C: Converter>(&self) -> Result<C::T, C::E> {
		C::convert(&self.0)
	}
}

#[cfg(test)]
mod test {
	use super::*;
	use crate::record::convert::converter::Converter;
	use mockall::{mock, predicate::*};
	use std::num::ParseIntError;

	struct Mock;

	impl Converter for Mock {
		type T = i32;
		type E = ParseIntError;

		fn convert(value: &str) -> Result<Self::T, Self::E> {
			value.parse::<i32>()
		}
	}

	#[test]
	fn from_str() {
		let fixture = Datum::from("123");
		assert_eq!(fixture.0, "123");
	}

	#[test]
	fn from_string() {
		let fixture = Datum::from(String::from("123"));
		assert_eq!(fixture.0, "123");
	}

	#[test]
	fn convert() {
		let fixture = Datum::from("123");
		assert_eq!(fixture.convert::<Mock>().unwrap(), 123);

		let fixture = Datum::from("456.00");
		assert!(fixture.convert::<Mock>().is_err());
	}

	#[test]
	fn source() {
		let fixture = Datum::from("hello");
		assert_eq!(fixture.source(), "hello");
	}
}

use std::str::FromStr;

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

	pub fn convert<C: FromStr>(&self) -> Result<C, C::Err> {
		C::from_str(&self.0)
	}
}

#[cfg(test)]
pub mod test_util {
	use super::*;

	impl Datum {
		pub fn assert(&self, expected: &str) {
			assert_eq!(self.source(), expected);
		}
	}
}

#[cfg(test)]
mod test {
	use super::*;

	struct Mock;

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
		assert_eq!(fixture.convert::<i32>().unwrap(), 123);

		let fixture = Datum::from("456.00");
		assert!(fixture.convert::<i32>().is_err());
	}

	#[test]
	fn source() {
		let fixture = Datum::from("hello");
		assert_eq!(fixture.source(), "hello");
	}
}

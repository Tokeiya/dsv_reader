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
}
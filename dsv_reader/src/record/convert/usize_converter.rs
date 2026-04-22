use super::converter::Converter;

struct UsizeConverter;

impl Converter for UsizeConverter {
	type T = usize;
	type E = std::num::ParseIntError;

	fn convert(value: &str) -> Result<Self::T, Self::E> {
		usize::from_str_radix(value, 10)
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn convert() {
		let fixture = UsizeConverter::convert("123");
		assert_eq!(fixture.unwrap(), 123);

		let fixture = UsizeConverter::convert("456.00");
		assert!(fixture.is_err());
	}
}

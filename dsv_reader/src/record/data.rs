use crate::record::datum::Datum;

pub struct Data(Vec<Datum>);

impl Data {
	pub fn len(&self) -> usize {
		self.0.len()
	}

	pub fn iter(&self) -> std::slice::Iter<Datum> {
		self.0.iter()
	}

	pub fn get(&self, index: usize) -> Option<&Datum> {
		self.0.get(index)
	}
}

impl From<&[String]> for Data {
	fn from(value: &[String]) -> Self {
		Data(value.iter().map(|s| Datum::from(s.to_string())).collect())
	}
}

impl std::ops::Index<usize> for Data {
	type Output = Datum;

	fn index(&self, index: usize) -> &Self::Output {
		&(&self.0)[index]
	}
}

#[cfg(test)]
mod test {
	use super::*;
	use std::mem::forget;
	use std::ops::Index;

	#[test]
	fn from_string() {
		let vec: Vec<_> = vec!["hello".into(), "world".into()];
		let fixture = Data::from(vec.as_slice());

		assert_eq!(fixture.0.len(), 2);
		fixture.0[0].assert("hello");
		fixture.0[1].assert("world");
	}

	#[test]
	fn len() {
		let mut vec: Vec<String> = Vec::new();

		for i in 0..10 {
			vec.push(i.to_string());
		}

		let fixture = Data::from(vec.as_slice());
		assert_eq!(fixture.len(), 10);
	}

	#[test]
	fn index() {
		let vec: Vec<String> = vec!["hello".into(), "world".into()];
		let fixture = Data::from(vec.as_slice());

		fixture[0].assert("hello");
		fixture[1].assert("world");
	}

	#[test]
	#[should_panic(expected = "index out of bounds")]
	fn out_index() {
		let fixture = Data::from(vec!["hello".into(), "world".into()].as_slice());

		fixture[2].assert("hello");
	}

	#[test]
	fn get() {
		let fixture = Data::from(vec!["hello".into(), "world".into()].as_slice());
		fixture.get(0).unwrap().assert("hello");
		fixture.get(1).unwrap().assert("world");
		assert!(fixture.get(2).is_none());
	}

	#[test]
	fn iter() {
		let fixture = Data::from(vec!["hello".into(), "world".into()].as_slice());

		for (i, datum) in fixture.iter().enumerate() {
			match i {
				0 => datum.assert("hello"),
				1 => datum.assert("world"),
				_ => panic!("unexpected index: {}", i),
			}
		}
	}
}

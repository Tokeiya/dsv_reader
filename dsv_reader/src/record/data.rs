use crate::record::datum::Datum;

pub struct Data(Vec<Datum>);

impl Data {
	pub fn len(&self) -> usize {
		todo!()
	}

	pub fn iter(&self) -> std::slice::Iter<Datum> {
		todo!()
	}

	pub fn get(&self, index: usize) -> Option<&Datum> {
		todo!()
	}
}

impl From<&[String]> for Data {
	fn from(value: &[String]) -> Self {
		todo!()
	}
}

impl std::ops::Index<usize> for Data {
	type Output = Datum;

	fn index(&self, index: usize) -> &Self::Output {
		todo!()
	}
}

#[cfg(test)]
mod test {
	use super::*;
}

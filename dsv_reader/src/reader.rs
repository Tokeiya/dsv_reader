use super::buffer::Buffer;
use std::error::Error;

pub trait Reader {
	type E: Error;
	fn read(&mut self) -> Result<Option<(String, bool)>, Self::E>;
	fn read_line(&mut self, buffer: &mut Vec<String>) -> Result<(), Self::E> {
		loop {
			match self.read()? {
				None => {}
				Some((v, b)) => {
					buffer.push(v);
					if b {
						return Ok(());
					}
				}
			}
		}
	}
}

#[cfg(test)]
mod test {
	use super::*;
	use mockall::mock;
	use mockall::predicate::*;

	mock! {
		Reader{}

		impl super::Reader for Reader {
			type E = std::io::Error;
			fn read(&mut self) -> Result<Option<(String, bool)>, std::io::Error> ;
		}
	}

	#[test]
	fn read_line() {
		let mut cnt = 0;

		let mut mock = MockReader::new();
		mock.expect_read().times(3).returning(move || {
			dbg!(cnt);
			let ret = cnt;
			cnt += 1;

			Ok(Some((ret.to_string(), ret == 2)))
		});

		let mut vec = Vec::new();
		mock.read_line(&mut vec).unwrap();
	}
}

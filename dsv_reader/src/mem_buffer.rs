use crate::buffer::Buffer;
use std::io::Read;

const SIZE: usize = 8_192;

pub struct MemBuffer<R> {
	backend: R,
	storage: [u8; SIZE],
	index: usize,
	length: Option<usize>,
}

impl<R> MemBuffer<R> {
	pub fn new(backend: R) -> Self {
		Self {
			backend,
			storage: [0; SIZE],
			index: 0,
			length: None,
		}
	}
}

impl<R: Read> MemBuffer<R> {
	fn load(&mut self) -> Result<(), std::io::Error> {
		if self.length.is_none() || self.index == self.length.unwrap() {
			self.length = Some(self.backend.read(&mut self.storage)?);
			self.index = 0;
		} else {
			let mut idx = 0;

			for i in self.index..self.length.unwrap() {
				self.storage[idx] = self.storage[i];
				idx += 1;
			}

			self.length = Some(self.backend.read(&mut self.storage[idx..])? + idx);
			self.index = 0;
		}

		dbg!(format!("idx:{} len{}", self.index, self.length.unwrap()));

		Ok(())
	}
}

impl<R: std::io::Read> Buffer for MemBuffer<R> {
	type Error = std::io::Error;

	fn read(&mut self) -> Result<Option<u8>, Self::Error> {
		let ret = self.peek()?;

		if ret.is_some() {
			self.index += 1;
		}
		Ok(ret)
	}

	fn peek(&mut self) -> Result<Option<u8>, Self::Error> {
		if self.length.is_none() {
			self.load()?;
		}

		let len = match self.length {
			None => unreachable!(),
			Some(len) => len,
		};

		if len == 0 {
			return Ok(None);
		}

		if len == self.index {
			self.load()?;
		}

		if self.length == Some(0) {
			Ok(None)
		} else {
			Ok(Some(self.storage[self.index]))
		}
	}

	fn look_ahead_1(&mut self) -> Result<Option<u8>, Self::Error> {
		if self.length.is_none() {
			self.load()?;
		}

		let len = match self.length {
			None => unreachable!(),
			Some(len) => len,
		};

		if len == 0 {
			return Ok(None);
		}

		if len == self.index + 1 {
			self.load()?;
		}

		if self.length <= Some(1) {
			Ok(None)
		} else {
			Ok(Some(self.storage[self.index + 1]))
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use mockall::mock;
	use std::io::Read;

	mock! {
		Reader{}
		impl Read for Reader{
			fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> ;
		}
	}

	#[test]
	fn new() {
		let fixture = MemBuffer::new(MockReader::new());
		assert_eq!(fixture.index, 0);
		assert!(fixture.storage.iter().all(|&x| x == 0));
		assert_eq!(fixture.storage.len(), SIZE);
		assert_eq!(fixture.length, None);
	}

	#[test]
	fn read() {
		let mut mock = MockReader::new();
		mock.expect_read().times(1).returning(|arr| {
			for i in 0..=0xff {
				arr[i as usize] = i;
			}
			Ok(256)
		});

		mock.expect_read().times(1).returning(|_| Ok(0));

		let mut fixture = MemBuffer::new(mock);

		for i in 0..=0xff {
			dbg!(i);
			assert_eq!(fixture.read().unwrap().unwrap(), i);
		}

		assert!(fixture.read().unwrap().is_none());
		assert!(fixture.read().unwrap().is_none());
		assert!(fixture.read().unwrap().is_none());
	}

	#[test]
	fn peek() {
		let mut mock = MockReader::new();

		mock.expect_read().times(1).returning(|arr| {
			for i in 0..=0xff {
				arr[i as usize] = i;
			}
			Ok(256)
		});

		mock.expect_read().times(1).returning(|_| Ok(0));

		let mut fixture = MemBuffer::new(mock);
		assert_eq!(fixture.peek().unwrap().unwrap(), 0x00);
		fixture.index = 0xff;
		assert_eq!(fixture.peek().unwrap(), Some(0xff));

		_ = fixture.read().unwrap().unwrap();
		assert_eq!(fixture.peek().unwrap(), None);
		assert_eq!(fixture.peek().unwrap(), None);
	}

	#[test]
	fn look_ahead_1() {
		let mut mock = MockReader::new();

		mock.expect_read().times(1).returning(|arr| {
			arr[0] = 0x01;
			Ok(1)
		});

		mock.expect_read().times(1).returning(|arr| {
			arr[0] = 0x02;
			Ok(1)
		});

		mock.expect_read().times(1).returning(|_| Ok(0));

		let mut fixture = MemBuffer::new(mock);
		assert_eq!(fixture.look_ahead_1().unwrap(), Some(0x02));
		fixture.read().unwrap();

		assert_eq!(fixture.look_ahead_1().unwrap(), None);
	}
}

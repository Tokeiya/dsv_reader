use crate::buffer::Buffer;
use std::arch::x86_64::_mm_undefined_si128;
use std::io::Read;

const SIZE: usize = 8_192;

pub struct MemBuffer<R> {
	backend: R,
	storage: [u8; SIZE],
	index: usize,
	length: usize,
}

impl<R> MemBuffer<R> {
	pub fn new(backend: R) -> Self {
		Self {
			backend,
			storage: [0; SIZE],
			index: 0,
			length: 0,
		}
	}
}

impl<R: Read> MemBuffer<R> {
	fn load(&mut self) -> Result<(), std::io::Error> {
		if self.index == self.length {
			self.length = self.backend.read(&mut self.storage)?;
			self.index = 0;
		} else {
			let mut idx = 0;

			for i in self.index..self.length {
				self.storage[idx] = self.storage[i];
				idx += 1;
			}

			self.length = self.backend.read(&mut self.storage[idx..])? + idx;
			self.index = 0;
		}

		Ok(())
	}
}

impl<R: std::io::Read> Buffer for MemBuffer<R> {
	type Error = std::io::Error;

	fn read(&mut self) -> Result<Option<u8>, Self::Error> {
		self.index += 1;
		if self.index >= self.length {
			self.load()?;
		};

		if self.length == 0 {
			Ok(None);
		} else {
		}

		todo!()
	}

	fn peek(&self) -> Result<Option<u8>, Self::Error> {
		todo!()
	}

	fn look_ahead_1(&self) -> Result<Option<u8>, Self::Error> {
		todo!()
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
		assert_eq!(fixture.index, usize::MAX);
		assert!(fixture.storage.iter().all(|&x| x == 0));
		assert_eq!(fixture.storage.len(), SIZE);
		assert_eq!(fixture.length, 0);
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
		fixture.index = 0xff;
		assert_eq!(fixture.peek().unwrap(), Some(0xff));

		fixture.read();
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

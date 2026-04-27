use super::buffer::Buffer;
use super::reader::Reader;
use crate::mem_buffer::MemBuffer;
use std::io::Read;

const CR: u8 = b'\r';
const LF: u8 = b'\n';
const QUOTE: u8 = b'"';
const TAB: u8 = b'\t';

pub struct TsvReader<B> {
	buffer: B,
}

impl<R: Read> TsvReader<MemBuffer<R>> {
	pub fn new(read: R) -> Self {
		Self {
			buffer: MemBuffer::new(read),
		}
	}
}

impl<R: Read> Reader for TsvReader<MemBuffer<R>> {
	type E = std::io::Error;
	fn read(&mut self) -> Result<Option<(String, bool)>, Self::E> {
		let piv = self.buffer.peek()?;

		if piv.is_none() {
			return Ok(None);
		}

		todo!()
	}
}

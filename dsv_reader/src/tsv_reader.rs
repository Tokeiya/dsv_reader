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
	
	fn read_unquoted(&mut self) -> Result<Option<(String, bool)>, std::io::Error> {
		todo!()
	}
	
	fn read_quoted(&mut self) -> Result<Option<(String, bool)>, std::io::Error> {
		todo!()
	}
	
	fn consume_new_line(&mut self) -> Result<(), std::io::Error> {
		match self.buffer.peek()?.unwrap() {
			CR => {
				_ = self.buffer.read()?;
				if matches!(self.buffer.peek()?, Some(c) if c == LF) {
					_ = self.buffer.read()?;
					Ok(())
				} else {
					Ok(())
				}
			}
			LF => {
				_ = self.buffer.read()?;
				Ok(())
			}
			_ => unreachable!()
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
		
		match piv.unwrap() {
			CR => {
				self.consume_new_line()?;
				Ok(Some((String::new(), true)))
			}
			LF => {
				self.consume_new_line()?;
				Ok(Some((String::new(), true)))
			}
			TAB => {
				_ = self.buffer.read()?;
				Ok(Some((String::new(), false)))
			}
			QUOTE => {
				self.read_quoted()
			}
			_ => {
				self.read_unquoted()
			}
		}
	}
}


#[cfg(test)]
mod test {
	use super::*;
	use std::io::Cursor;
	#[test]
	fn consume_new_line_cr_lf() {
		let cursor = Cursor::new("\r\n".to_string());
		let mut fixture = TsvReader { buffer: MemBuffer::new(cursor) };
		
		_ = fixture.consume_new_line().unwrap();
		assert_eq!(fixture.buffer.peek().unwrap(), None);
	}
	
	#[test]
	fn consume_new_line_cr() {
		let cursor = Cursor::new("\r".to_string());
		let mut fixture = TsvReader { buffer: MemBuffer::new(cursor) };
		
		_ = fixture.consume_new_line().unwrap();
		assert_eq!(fixture.buffer.peek().unwrap(), None);
	}
	
	#[test]
	fn consume_new_line_lf() {
		let cursor = Cursor::new("\n".to_string());
		let mut fixture = TsvReader { buffer: MemBuffer::new(cursor) };
		
		_ = fixture.consume_new_line().unwrap();
		assert_eq!(fixture.buffer.peek().unwrap(), None);
	}
}
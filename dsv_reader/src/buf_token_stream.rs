use crate::token::Token;
use crate::token_stream::TokenStream;
use std::io::{BufRead, Read, Result as IoResult};

const LENGTH: usize = 8_192;
const CR: u8 = b'\r';
const LF: u8 = b'\n';
const QUOTE: u8 = b'"';
pub struct BufStream<R, const D: u8> {
	read: R,
	buffer: [u8; LENGTH],
	len: usize,
	index: usize,
	current: IoResult<Token>,
	next: IoResult<Token>,
}

impl<R: Read, const D: u8> BufStream<R, D> {
	pub fn try_new(read: R) -> IoResult<Self> {
		let mut ret = Self {
			read,
			buffer: [0; LENGTH],
			len: 0,
			index: 0,
			current: Ok(Token::EOF),
			next: Ok(Token::EOF),
		};
		
		ret.initial_process()?;
		Ok(ret)
	}
	
	fn initial_process(&mut self) -> IoResult<()> {
		debug_assert!(
			matches!(&self.current, Ok(t) if t==&Token::EOF)
				&& matches!(&self.next,Ok(t) if t==&Token::EOF)
				&& self.buffer.iter().all(|&x| x == 0),
			"Once Initialized"
		);
		
		self.len = self.read.read(&mut self.buffer)?;
		
		self.current = self.read_token();
		self.next = self.read_token();
		
		Ok(())
	}
	
	fn fill_buffer(&mut self) -> IoResult<()> {
		debug_assert_ne!(self.len, 0);
		debug_assert_eq!(self.index, self.len);
		self.len = self.read.read(&mut self.buffer)?;
		self.index = 0;
		Ok(())
	}
	
	fn move_index(&mut self) -> IoResult<()> {
		debug_assert_ne!(self.len, 0);
		
		self.index += 1;
		if self.index >= self.len {
			self.fill_buffer()
		} else {
			Ok(())
		}
	}
	
	fn peek_buffer(&mut self) -> Option<u8> {
		if self.len == 0 {
			None
		} else {
			Some(self.buffer[self.index])
		}
	}
	
	fn cr_process(&mut self) -> IoResult<Token> {
		self.move_index()?;
		
		let pivot = self.peek_buffer();
		
		if matches!(pivot,Some(c) if c==LF) {
			self.move_index()?;
			Ok(Token::CRLF)
		} else {
			Ok(Token::CR)
		}
	}
	
	fn other_process(&mut self) -> IoResult<Token> {
		let mut vec = Vec::new();
		
		while let Some(c) = self.peek_buffer() {
			if c == QUOTE || c == LF || c == CR || c == D {
				break;
			} else {
				vec.push(c);
				self.move_index()?;
			}
		}
		
		Ok(Token::Value(vec))
	}
	
	fn read_token(&mut self) -> IoResult<Token> {
		let cursor = self.peek_buffer();
		
		if cursor.is_none() {
			return Ok(Token::EOF);
		};
		
		match cursor.unwrap() {
			CR => self.cr_process(),
			LF => {
				self.move_index()?;
				Ok(Token::LF)
			}
			QUOTE => {
				self.move_index()?;
				Ok(Token::Quoted)
			}
			x if x == D => {
				self.move_index()?;
				Ok(Token::Delimiter(D))
			}
			_ => self.other_process(),
		}
	}
}

impl<R: BufRead, const D: u8> TokenStream for BufStream<R, D> {
	type Error = std::io::Error;
	
	fn advance_token(&mut self) -> Result<Token, Self::Error> {
		let tmp = self.read_token();
		let tmp = std::mem::replace(&mut self.next, tmp);
		let tmp = std::mem::replace(&mut self.current, tmp);
		
		tmp
	}
	
	fn current_token(&mut self) -> Result<&Token, &Self::Error> {
		self.current.as_ref()
	}
	
	fn ahead_token(&mut self) -> Result<&Token, &Self::Error> {
		self.next.as_ref()
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use mockall::mock;
	use rand::prelude::{IndexedRandom, SliceRandom};
	use rand::RngExt;
	use std::io::Cursor;
	use std::io::{Error as IoError, ErrorKind, Result as IoResult};
	
	
	mock! {
		Read{}

		impl std::io::Read for Read {
			fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> ;
		}
	}
	
	
	
	#[test]
	fn try_new_error() {
		let mut mock = MockRead::new();
		mock.expect_read()
			.times(1)
			.returning(|_| IoResult::Err(IoError::new(ErrorKind::Other, "test")));
		
		let fixture = BufStream::<_, b'\t'>::try_new(mock);
		
		match fixture {
			Ok(_) => unreachable!(),
			Err(e) => {
				assert_eq!(e.kind(), ErrorKind::Other);
				assert_eq!(e.to_string(), "test");
			}
		}
	}
	#[test]
	fn try_new() {
		let cursor = Cursor::new("hello\tworld".to_string());
		let fixture = BufStream::<_, b'\t'>::try_new(cursor).unwrap();
		
		assert!(matches!(fixture.current,Ok(Token::Value(vec)) if vec==b"hello"));
		assert!(matches!(fixture.next,Ok(Token::Delimiter(t)) if t==b'\t'));
		
		assert_eq!(fixture.len, 11);
		assert_eq!(fixture.index, 6);
		assert_eq!(&fixture.buffer[..11], b"hello\tworld");
	}
	
	fn assert(scr: &str) {
		assert!(scr.len() >= LENGTH);
		let mut fixture = BufStream::<_, b'\t'>::try_new(Cursor::new(scr.to_string())).unwrap();
		let mut accum: Vec<u8> = Vec::new();
		
		loop {
			let peek = fixture.current_token().unwrap().clone();
			let ahead = fixture.ahead_token().unwrap().clone();
			
			assert_eq!(fixture.current_token().unwrap(), &peek);
			assert_eq!(fixture.ahead_token().unwrap(), &ahead);
			
			let advance = fixture.advance_token().unwrap();
			
			assert_eq!(advance, peek);
			
			match &advance {
				Token::Delimiter(d) => accum.push(*d),
				Token::Quoted => accum.push(QUOTE),
				Token::CR => accum.push(CR),
				Token::LF => accum.push(LF),
				Token::CRLF => accum.extend_from_slice(b"\r\n"),
				Token::Value(v) => accum.extend_from_slice(&v),
				Token::EOF => {}
			}
			
			if advance == Token::EOF {
				break;
			}
		}
		
		assert_eq!(String::from_utf8(accum).unwrap().as_str(), scr);
	}
	
	#[test]
	fn simple_read() {
		let expected = rand::rng()
			.random_iter::<i64>()
			.take(10_000)
			.collect::<Vec<_>>();
		
		let mut scr = String::new();
		
		for i in expected.iter() {
			scr.push_str(&i.to_string());
			scr.push('\t');
		}
		scr.push_str("42");
		
		assert(&scr);
	}
	
	#[test]
	fn complex_read() {
		let scr = (0u8..=0x7f).collect::<Vec<_>>();
		let mut rng = rand::rng();
		
		let mut exp = (0..10_000)
			.map(|_| *scr.choose(&mut rng).unwrap())
			.collect::<Vec<_>>();
		
		for i in scr.iter() {
			exp.push(*i);
		}
		
		exp.shuffle(&mut rng);
		
		exp.insert(400, b'\"');
		exp.insert(401, b'\"');
		
		exp.insert(500, b'\r');
		exp.insert(501, b'\n');
		
		let str = String::from_utf8(exp).unwrap();
		
		assert(&str);
	}
}

use crate::token::Token;
use crate::token_stream::TokenStream;
use std::io::{BufRead, Error as IoError, Result as IoResult};

const LENGTH: usize = 8_192;
const CR: u8 = b'\r';
const LF: u8 = b'\n';
const QUOTE: u8 = b'"';

pub struct BufStream<R, const D: u8> {
	read: R,
	buffer: [u8; LENGTH],
	len: usize,
	index: usize,
	current: Token,
	next: Token,
}

impl<R: BufRead, const D: u8> BufStream<R, D> {
	pub fn try_new(read: R) -> IoResult<Self> {
		let mut ret = Self {
			read,
			buffer: [0; LENGTH],
			len: 0,
			index: 0,
			current: Token::EOF,
			next: Token::EOF,
		};

		ret.initial_process()?;
		Ok(ret)
	}

	fn initial_process(&mut self) -> IoResult<()> {
		debug_assert!(
			self.current == Token::EOF
				&& self.next == Token::EOF
				&& self.buffer.iter().all(|&x| x == 0),
			"Once Initialized"
		);

		self.len = self.read.read(&mut self.buffer)?;

		self.current = self.read_token()?;
		self.next = self.read_token()?;

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

	fn quote_process(&mut self) -> IoResult<Token> {
		self.move_index()?;

		if matches!(self.peek_buffer(),Some(c) if c==QUOTE) {
			self.move_index()?;
			Ok(Token::EscapedQuoted)
		} else {
			Ok(Token::Quoted)
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

		let cursor = cursor.unwrap();

		match cursor {
			CR => self.cr_process(),
			LF => {
				self.move_index()?;
				Ok(Token::LF)
			}
			QUOTE => self.quote_process(),
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
	fn next_token(&mut self) -> Result<Token, Self::Error> {
		todo!()
	}

	fn peek_token(&mut self) -> Result<&Token, Self::Error> {
		todo!()
	}

	fn ahead_token(&mut self) -> Result<&Token, Self::Error> {
		todo!()
	}
}

#[cfg(test)]
mod tests {
	use super::*;
}

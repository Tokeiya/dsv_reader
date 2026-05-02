use super::dsv_read::{DsvRead, Error};
use crate::token::Token;
use crate::token_stream::TokenStream;
pub struct DsvReader<R: TokenStream, const D: u8> {
	token_stream: R,
}

impl<R: TokenStream, const D: u8> DsvReader<R, D> {
	pub fn new(token_stream: R) -> Self {
		Self { token_stream }
	}

	fn read(&mut self, buff: Option<&Vec<u8>>) -> Result<Token, Error<R::Error>> {
		self.token_stream
			.advance_token()
			.map_err(|err| Error::TokenStreamError {
				source: err,
				remaining: if let Some(b) = buff {
					b.clone()
				} else {
					Vec::new()
				},
			})
	}

	fn peek(&mut self, buff: Option<&Vec<u8>>) -> Result<&Token, Error<R::Error>> {
		if self.token_stream.current_token().is_err() {
			Err(self.read(buff).unwrap_err())
		} else {
			Ok(self.token_stream.current_token().unwrap())
		}
	}

	fn read_normal(
		&mut self,
		mut buff: Vec<u8>,
	) -> Result<Option<(String, bool)>, Error<R::Error>> {
		let is_eol;

		loop {
			match self.read(Some(&buff))? {
				Token::Delimiter(_) => {
					is_eol = false;
					break;
				}
				Token::Quoted => return Err(Error::UnexpectedQuote),
				Token::CR => {
					is_eol = true;
					break;
				}
				Token::LF => {
					is_eol = true;
					break;
				}
				Token::CRLF => {
					is_eol = true;
					break;
				}
				Token::Value(v) => buff.extend_from_slice(v.as_slice()),
				Token::EOF => return Err(Error::UnexpectedEOF),
			}
		}

		let str = String::from_utf8(buff).map_err(Error::FromUtf8Error)?;
		Ok(Some((str, is_eol)))
	}

	fn read_quoted(&mut self) -> Result<Option<(String, bool)>, Error<R::Error>> {
		let mut buff = Vec::new();
		loop {
			match self.read(Some(&buff))? {
				Token::Delimiter(_) => return Err(Error::UnexpectedDelimiter),
				Token::Quoted => match self.peek(Some(&buff))? {
					Token::Delimiter(_) => {
						_ = self.read(Some(&buff))?;
						let str = String::from_utf8(buff).map_err(Error::FromUtf8Error)?;
						return Ok(Some((str, false)));
					}
					Token::Quoted => buff.push(b'"'),
					Token::CR => {
						_ = self.read(Some(&buff))?;
						let str = String::from_utf8(buff).map_err(Error::FromUtf8Error)?;
						return Ok(Some((str, true)));
					}
					Token::LF => {
						_ = self.read(Some(&buff))?;
						let str = String::from_utf8(buff).map_err(Error::FromUtf8Error)?;
						return Ok(Some((str, true)));
					}
					Token::CRLF => {
						_ = self.read(Some(&buff))?;
						let str = String::from_utf8(buff).map_err(Error::FromUtf8Error)?;
						return Ok(Some((str, true)));
					}
					Token::Value(_) => return Err(Error::UnexpectedQuote),
					Token::EOF => {
						_ = self.read(Some(&buff))?;
						let str = String::from_utf8(buff).map_err(Error::FromUtf8Error)?;
						return Ok(Some((str, true)));
					}
				},
				Token::CR => buff.push(b'\r'),
				Token::LF => buff.push(b'\n'),
				Token::CRLF => buff.extend_from_slice(b"\r\n"),
				Token::Value(vec) => buff.extend_from_slice(vec.as_slice()),
				Token::EOF => return Err(Error::UnexpectedEOF),
			}
		}
	}
}

impl<R: TokenStream, const D: u8> DsvRead<R, D> for DsvReader<R, D> {
	fn read(&mut self) -> Result<Option<(String, bool)>, Error<R::Error>> {
		let piv = self.read(None)?;

		match piv {
			Token::Delimiter(d) => Ok(Some(("".to_string(), false))),
			Token::Quoted => self.read_quoted(),
			Token::CR => Ok(Some(("".to_string(), true))),
			Token::LF => Ok(Some(("".to_string(), true))),
			Token::CRLF => Ok(Some(("".to_string(), true))),
			Token::Value(v) => self.read_normal(v),
			Token::EOF => Ok(None),
		}
	}
}

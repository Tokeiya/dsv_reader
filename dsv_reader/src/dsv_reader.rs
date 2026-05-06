use super::dsv_read::DsvRead;
use crate::read_error::Error;
use crate::token::Token;
use crate::token_stream::TokenStream;
pub struct DsvReader<R: TokenStream> {
	token_stream: R,
}

impl<R: TokenStream> DsvReader<R> {
	pub fn new(token_stream: R) -> Self {
		Self { token_stream }
	}
	
	fn read_token(&mut self, buff: Option<&Vec<u8>>) -> Result<Token, Error<R::Error>> {
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
	
	fn peek_token(&mut self, buff: Option<&Vec<u8>>) -> Result<&Token, Error<R::Error>> {
		if self.token_stream.current_token().is_err() {
			Err(self.read_token(buff).unwrap_err())
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
			match self.read_token(Some(&buff))? {
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
				Token::EOF => {
					is_eol = true;
					break;
				}
			}
		}
		
		let str = String::from_utf8(buff).map_err(Error::FromUtf8Error)?;
		Ok(Some((str, is_eol)))
	}
	
	fn read_quoted(&mut self) -> Result<Option<(String, bool)>, Error<R::Error>> {
		let mut buff = Vec::new();
		loop {
			match self.read_token(Some(&buff))? {
				Token::Delimiter(d) => buff.push(d),
				Token::Quoted => match self.peek_token(Some(&buff))? {
					Token::Delimiter(_) => {
						_ = self.read_token(Some(&buff))?;
						let str = String::from_utf8(buff).map_err(Error::FromUtf8Error)?;
						return Ok(Some((str, false)));
					}
					Token::Quoted => {
						_ = self.read_token(Some(&buff))?;
						buff.push(b'"')
					}
					Token::CR => {
						_ = self.read_token(Some(&buff))?;
						let str = String::from_utf8(buff).map_err(Error::FromUtf8Error)?;
						return Ok(Some((str, true)));
					}
					Token::LF => {
						_ = self.read_token(Some(&buff))?;
						let str = String::from_utf8(buff).map_err(Error::FromUtf8Error)?;
						return Ok(Some((str, true)));
					}
					Token::CRLF => {
						_ = self.read_token(Some(&buff))?;
						let str = String::from_utf8(buff).map_err(Error::FromUtf8Error)?;
						return Ok(Some((str, true)));
					}
					Token::Value(v) => {
						_ = dbg!(String::from_utf8(v.clone()));
						return Err(Error::UnexpectedQuote);
					}
					Token::EOF => {
						_ = self.read_token(Some(&buff))?;
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

impl<R: TokenStream> DsvRead<R> for DsvReader<R> {
	fn read(&mut self) -> Result<Option<(String, bool)>, Error<R::Error>> {
		let piv = self.read_token(None)?;
		
		match piv {
			Token::Delimiter(_) => Ok(Some(("".to_string(), false))),
			Token::Quoted => self.read_quoted(),
			Token::CR => Ok(Some(("".to_string(), true))),
			Token::LF => Ok(Some(("".to_string(), true))),
			Token::CRLF => Ok(Some(("".to_string(), true))),
			Token::Value(v) => self.read_normal(v),
			Token::EOF => Ok(None),
		}
	}
}

#[cfg(test)]
mod test {
	use super::*;
	use crate::BufStream;
	use std::io::Cursor;
	
	fn assert(
		fixture: &mut DsvReader<BufStream<Cursor<&'static str>, b','>>,
		scr: &str,
		eol: bool,
	) {
		let (v, b) = fixture.read().unwrap().unwrap();
		assert_eq!((v, b), (scr.to_string(), eol));
	}
	
	fn create(scr: &'static str) -> DsvReader<BufStream<Cursor<&'static str>, b','>> {
		let cursor = Cursor::new(scr);
		let stream = BufStream::try_new(cursor).unwrap();
		DsvReader::new(stream)
	}
	
	#[test]
	fn zero_byte() {
		let mut fixture = create("");
		assert_eq!(fixture.read().unwrap(), None);
	}
	
	#[test]
	fn quoted_empty() {
		let mut fixture = create(r##""","""##);
		assert(&mut fixture, "", false);
		assert(&mut fixture, "", true);
		assert_eq!(fixture.read().unwrap(), None);
		
		let mut fixture = create(r##","""##);
		assert(&mut fixture, "", false);
		assert(&mut fixture, "", true);
		assert_eq!(fixture.read().unwrap(), None);
	}
	
	#[test]
	fn simple_normal() {
		let mut fixture = create("a,b,c\nd,e,f\rg,h,i\r\nj,k,l");
		
		assert(&mut fixture, "a", false);
		assert(&mut fixture, "b", false);
		assert(&mut fixture, "c", true);
		
		assert(&mut fixture, "d", false);
		assert(&mut fixture, "e", false);
		assert(&mut fixture, "f", true);
		
		assert(&mut fixture, "g", false);
		assert(&mut fixture, "h", false);
		assert(&mut fixture, "i", true);
		
		assert(&mut fixture, "j", false);
		assert(&mut fixture, "k", false);
		assert(&mut fixture, "l", true);
		assert_eq!(fixture.read().unwrap(), None);
	}
	
	#[test]
	fn quoted() {
		let mut fixture = create(
			r##""a","b","c"
"d""e""##,
		);
		
		assert(&mut fixture, "a", false);
		assert(&mut fixture, "b", false);
		assert(&mut fixture, "c", true);
		
		assert(&mut fixture, r##"d"e"##, true);
		assert_eq!(fixture.read().unwrap(), None);
		
		let mut fixture = create(
			r##""a
b
c
d
""##,
		);
		
		assert(&mut fixture, "a\nb\nc\nd\n", true);
		
		let mut fixture = create(r##""a,b,c""##);
		assert(&mut fixture, "a,b,c", true);
	}
	
	#[test]
	fn unexpected() {
		let mut fixture = create(r##"ho"ge"##);
		assert!(matches!(fixture.read_quoted(), Err(Error::UnexpectedQuote)));
		
		let mut fixture = create(
			r##""a
		b
		"##,
		);
		
		assert!(matches!(fixture.read(), Err(Error::UnexpectedEOF)));
	}
}

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
	
	fn read(&mut self, buff: &Vec<u8>) -> Result<Token, Error<R::Error>> {
		self.token_stream.advance_token()
			.map_err(|err| Error::TokenStreamError { source: err, remaining: buff.clone() })
	}
	
	fn peek(&mut self, buff: &Vec<u8>) -> Result<&Token, Error<R::Error>> {
		match self.read(&buff) {
			Ok(_) => { unreachable!() }
			Err(err) => {
				Err(err)
			}
		}
	}
}


impl<R: TokenStream, const D: u8> DsvRead<R, D> for DsvReader<R, D> {
	fn read(&mut self) -> Result<Option<(String, bool)>, Error<R::Error>> {
		let mut is_quoted = false;
		let mut buff = Vec::<u8>::new();
		
		let piv = self.read(&mut buff)?;
		
		match piv {
			Token::Delimiter(d) => {
				return Ok(Some(("".to_string(), false)));
			}
			Token::Quoted => {
				is_quoted = true;
			}
			Token::CR => {
				return Ok(Some(("".to_string(), true)));
			}
			Token::LF => {
				return Ok(Some(("".to_string(), true)));
			}
			Token::CRLF => {
				return Ok(Some(("".to_string(), true)));
			}
			Token::Value(v) => {}
			Token::EOF => {
				return Ok(None)
			}
		};
		
		todo!()
	}
}


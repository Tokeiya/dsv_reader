use super::dsv_read::{DsvRead, Error};
use crate::token::Token;
use crate::token_stream::TokenStream;
pub struct DsvReader<R: TokenStream, const D: u8> {
	token_stream: R,
}

impl<R: TokenStream, const D: u8> DsvRead<R, D> for DsvReader<R, D> {
	fn read(&mut self) -> Result<Option<(String, bool)>, Error<R::Error>> {
		let mut buff: Vec<u8> = Vec::new();
		let current = self
			.token_stream.advance_token()
			.map_err(|error| Error::TokenStreamError {
				error,
				remaining: "".to_string(),
			})?;
		
		let mut is_quoted = false;
		
		match current {
			Token::Delimiter(_) => return Ok(Some(("".to_string(), false))),
			Token::Quoted => is_quoted = true,
			Token::EscapedQuoted => return Err(Error::UnexpectedQuote),
			Token::CR | Token::LF | Token::CRLF => return Ok(Some(("".to_string(), true))),
			Token::Value(s) => buff = s,
			Token::EOF => return Ok(None),
		};
		
		buff.push(b'a');
		
		todo!()
	}
}


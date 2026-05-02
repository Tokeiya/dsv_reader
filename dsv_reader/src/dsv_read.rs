use super::token_stream::TokenStream;
use std::string::FromUtf8Error;
use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
pub enum Error<E: std::error::Error> {
	#[error("Unexpected end of file")]
	UnexpectedQuote,
	#[error("Unexpected end of file")]
	UnexpectedEOF,
	#[error("Unexpected delimiter")]
	UnexpectedDelimiter,
	#[error("Token stream error: {source}")]
	TokenStreamError { source: E, remaining: Vec<u8> },
	#[error(transparent)]
	FromUtf8Error(#[from] FromUtf8Error),
	#[error("Unexpected new line")]
	UnexpectedNewLine,
}

pub type Result<T, E> = std::result::Result<T, Error<E>>;

pub trait DsvRead<R: TokenStream> {
	fn read(&mut self) -> Result<Option<(String, bool)>, R::Error>;
	fn fill(&mut self, buffer: &mut Vec<String>) -> Result<Option<usize>, R::Error> {
		let mut cnt = 0;

		loop {
			let piv = self.read()?;

			match piv {
				None => break,
				Some((s, b)) => {
					buffer.push(s);
					cnt += 1;
					if b {
						break;
					}
				}
			}
		}

		Ok(Some(cnt))
	}
}

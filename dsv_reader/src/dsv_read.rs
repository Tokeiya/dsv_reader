use super::token_stream::TokenStream;
use thiserror::Error as ThisError;


#[derive(ThisError)]
pub enum Error<E: std::error::Error> {
	#[error("Unexpected end of file")]
	UnexpectedQuote,
	#[error("Unexpected end of file")]
	UnexpectedEOF,
	#[error("Token stream error: {error}")]
	TokenStreamError {
		error: E,
		remaining: String,
	},
}

pub trait DsvRead<R: TokenStream, const D: u8> {
	fn read(&mut self) -> Result<Option<(String, bool)>, Error<R::Error>>;
	fn fill(&mut self, buffer: &mut Vec<String>) -> Result<Option<usize>, Error<R::Error>> {
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
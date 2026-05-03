use super::token_stream::TokenStream;
use crate::read_error::Error;

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

use super::dsv_read::{DsvRead, Error};
use crate::token_stream::TokenStream;
pub struct DsvReader<R: TokenStream, const D: u8> {
	token_stream: R,
}

impl<R: TokenStream, const D: u8> DsvRead<R, D> for DsvReader<R, D> {
	fn read(&mut self) -> Result<Option<(String, bool)>, Error<R::Error>> {
		todo!()
	}
}


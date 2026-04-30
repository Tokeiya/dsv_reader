use crate::token::Token;
use std::error::Error as StdError;

pub trait TokenStream {
	type Error: StdError;
	fn next_token(&mut self) -> Result<Token, Self::Error>;
	fn peek_token(&mut self) -> Result<Token, Self::Error>;
	fn ahead_token(&mut self) -> Result<Token, Self::Error>;
}



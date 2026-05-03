use crate::token::Token;
use std::error::Error as StdError;

pub trait AsyncTokenStream {
	type Error: StdError;
	fn advance_token(&mut self) -> impl Future<Output = Result<Token, Self::Error>> + '_;
	fn current_token(&mut self) -> Result<&Token, &Self::Error>;
	fn ahead_token(&mut self) -> Result<&Token, &Self::Error>;
}
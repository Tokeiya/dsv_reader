pub trait Buffer {
	type Error: std::fmt::Debug;
	fn read(&mut self) -> Result<Option<u8>, Self::Error>;
	fn peek(&self) -> Result<Option<u8>, Self::Error>;
	fn look_ahead_1(&self) -> Result<Option<u8>, Self::Error>;
}

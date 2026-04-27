pub trait Buffer {
	type Error: std::fmt::Debug;
	fn read(&mut self) -> Result<Option<u8>, Self::Error>;
	fn peek(&mut self) -> Result<Option<u8>, Self::Error>;
	fn look_ahead_1(&mut self) -> Result<Option<u8>, Self::Error>;
	fn look_ahead_2(&mut self) -> Result<Option<u8>, Self::Error>;
}

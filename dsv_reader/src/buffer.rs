pub trait Buffer {
	fn read(&mut self) -> u8;
	fn peek(&self) -> u8;
	fn look_ahead_1(&self) -> u8;
}
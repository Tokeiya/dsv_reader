use crate::buffer::Buffer;

const SIZE: usize = 8_192;

pub struct MemBuffer<R> {
	backend: R,
	storage: [u8; SIZE],
	index: usize,
}

impl<R> MemBuffer<R> {
	pub fn new(backend: R) -> Self {
		Self {
			backend,
			storage: [0; SIZE],
			index: 0,
		}
	}
}

impl<R: std::io::Read> Buffer for MemBuffer<R> {
	fn read(&mut self) -> u8 {
		todo!()
	}
	
	fn peek(&self) -> u8 {
		todo!()
	}
	
	fn look_ahead_1(&self) -> u8 {
		todo!()
	}
}

#[cfg(test)]
mod tests {}
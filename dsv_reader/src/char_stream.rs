use std::io::{BufRead, Cursor, Read};

pub struct CharStream<T> {
	buffer: Vec<char>,
	reader: T,
}

impl<R: BufRead> CharStream<R> {
	pub fn new(reader: R) -> Self {
		Self {
			buffer: Vec::new(),
			reader,
		}
	}

	pub fn pop(&mut self) -> Option<char> {
		todo!()
	}

	pub fn peek(&self) -> Option<char> {
		todo!()
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn new() {
		let cursor = Cursor::new("a\nb\nc");
		let mut fixture = CharStream::new(cursor);

		assert_eq!(fixture.buffer.len(), 0)
	}

	#[test]
	fn pop() {
		let cursor = Cursor::new("a\nb\nc");
		let mut fixture = CharStream::new(cursor);

		assert_eq!(fixture.pop(), Some('a'));
		assert_eq!(fixture.pop(), Some('\n'));
		assert_eq!(fixture.pop(), Some('b'));
		assert_eq!(fixture.pop(), Some('\n'));
		assert_eq!(fixture.pop(), Some('c'));
		assert_eq!(fixture.pop(), None);
	}

	#[test]
	fn peek() {
		let cursor = Cursor::new("a\nb\nc");
		let mut fixture = CharStream::new(cursor);

		assert_eq!(fixture.peek(), Some('a'));
		assert_eq!(fixture.peek(), Some('a'));
		fixture.pop();

		assert_eq!(fixture.peek(), Some('\n'));
		fixture.pop();

		assert_eq!(fixture.peek(), Some('b'));
		fixture.pop();

		assert_eq!(fixture.peek(), Some('\n'));
		fixture.pop();

		assert_eq!(fixture.peek(), None);
	}
}

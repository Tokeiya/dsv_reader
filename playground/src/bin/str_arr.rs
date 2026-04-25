use std::io::{Cursor, Read};

fn main() {
	let mut cur = Cursor::new("HelloWorld");
	let mut buf = [0u8; 4];
	
	println!("Buffer size: {}", cur.read(buf.as_mut_slice()).unwrap());
	
	let a = String::from_utf8_lossy(buf.as_slice());
	
	
	println!("{}", a);
	
	for (i, c) in buf.as_slice().iter().enumerate() {
		println!("Index: {}, Char: {}", i, c);
	}
}
use std::fs::File;
use std::io::BufReader;
use dsv_reader::{BufStream, DsvRead, DsvReader};

fn main() {
	let f = File::open("../bingo_simulator/data/sample.tsv").unwrap();
	let token = BufStream::<_, b'\t'>::try_new(f).unwrap();
	
	let mut reader = DsvReader::new(token);
	
	let (s,b)=reader.read().unwrap().unwrap();
	
	
	println!("{:?}",s);
	println!("{:?}",b);
}

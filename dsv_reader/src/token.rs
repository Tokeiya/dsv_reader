#[derive(Debug, PartialEq, Clone)]
pub enum Token {
	Delimiter(u8),
	Quoted,
	CR,
	LF,
	CRLF,
	Value(Vec<u8>),
	EOF,
}

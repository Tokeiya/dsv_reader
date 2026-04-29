#[derive(Debug, PartialEq)]
pub enum Token {
	Delimiter(u8),
	Quoted,
	EscapedQuoted,
	CR,
	LF,
	CRLF,
	Value(Vec<u8>),
	EOF,
}

use std::string::FromUtf8Error;
use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
pub enum Error<E: std::error::Error> {
	#[error("Unexpected quote")]
	UnexpectedQuote,
	#[error("Unexpected end of file")]
	UnexpectedEOF,
	#[error("Unexpected delimiter")]
	UnexpectedDelimiter,
	#[error("Token stream error: {source}")]
	TokenStreamError { source: E, remaining: Vec<u8> },
	#[error(transparent)]
	FromUtf8Error(#[from] FromUtf8Error),
	#[error("Unexpected new line")]
	UnexpectedNewLine,
}
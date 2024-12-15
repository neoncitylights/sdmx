use sdmx::data::DataMessage;
use std::error::Error;
use std::fmt::Display;
use std::fs::read_to_string;
use std::str::FromStr;

#[derive(Debug)]
pub enum ReadJsonError {
	IO(std::io::Error),
	Serde(serde_json::Error),
}

impl Error for ReadJsonError {}
impl Display for ReadJsonError {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match self {
			ReadJsonError::IO(e) => write!(f, "IO error while reading: {}", e),
			ReadJsonError::Serde(e) => write!(f, "Serde error while parsing: {}", e),
		}
	}
}

pub fn read_json(path: &str) -> Result<DataMessage, ReadJsonError> {
	match read_to_string(path) {
		Ok(s) => DataMessage::from_str(&s).map_err(ReadJsonError::Serde),
		Err(e) => Err(ReadJsonError::IO(e)),
	}
}

#[macro_export]
macro_rules! fixture {
	($fname:expr) => {
		concat!(env!("CARGO_MANIFEST_DIR"), "/files/", $fname) // assumes Linux ('/')!
	};
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	#[cfg_attr(miri, ignore)]
	fn test_sample01() {
		let file = read_json(fixture!("sample01.json"));
		assert!(file.is_ok());
	}

	#[test]
	#[cfg_attr(miri, ignore)]
	fn test_sample02() {
		let file = read_json(fixture!("sample02.json"));
		println!("{:?}", file);
	}
}

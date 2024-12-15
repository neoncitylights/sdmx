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

pub fn read_json<T>(path: &str) -> Result<T, ReadJsonError>
where
	T: FromStr<Err = serde_json::Error>,
{
	match read_to_string(path) {
		Ok(s) => T::from_str(&s).map_err(ReadJsonError::Serde),
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
	use sdmx::data::DataMessage;
	use sdmx::metadata::MetadataMessage;

	#[test]
	#[cfg_attr(miri, ignore)]
	fn test_data_sample01() {
		let file = read_json::<DataMessage>(fixture!("data_sample01.json"));
		assert!(file.is_ok());
	}

	#[test]
	#[cfg_attr(miri, ignore)]
	fn test_data_sample02() {
		let file = read_json::<DataMessage>(fixture!("data_sample02.json"));
		assert!(file.is_ok());
	}

	#[test]
	#[cfg_attr(miri, ignore)]
	fn test_metadata_sample01() {
		let file = read_json::<MetadataMessage>(fixture!("metadata_sample01.json"));
		assert!(file.is_ok());
	}

	#[test]
	#[cfg_attr(miri, ignore)]
	fn test_metadata_sample02() {
		let file = read_json::<MetadataMessage>(fixture!("metadata_sample02.json"));
		assert!(file.is_ok());
	}

	#[test]
	#[cfg_attr(miri, ignore)]
	fn test_metadata_sample03() {
		let file = read_json::<MetadataMessage>(fixture!("metadata_sample03.json"));
		assert!(file.is_ok());
	}

	// #[test]
	// #[cfg_attr(miri, ignore)]
	// fn test_structure_sample01() {
	// 	let file = read_json::<MetadataMessage>(fixture!("structure_sample01.json"));
	// 	println!("{:?}", file);
	// }
}

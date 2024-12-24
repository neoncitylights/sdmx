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
mod tests_data_message {
	use super::*;
	use sdmx_json::data::DataMessage;

	#[test]
	#[cfg_attr(miri, ignore)]
	fn test_constructed_sample_full() {
		let file = read_json::<DataMessage>(fixture!("data/twg-constructed-sample-full.json"));
		assert!(file.is_ok(), "{:?}", file);
	}

	#[test]
	#[cfg_attr(miri, ignore)]
	fn test_generated_sample() {
		let file = read_json::<DataMessage>(fixture!("data/twg-generated-sample.json"));
		assert!(file.is_ok(), "{:?}", file);
	}
}

#[cfg(test)]
mod tests_metadata_message {
	use super::*;
	use sdmx_json::metadata::MetadataMessage;

	#[test]
	#[cfg_attr(miri, ignore)]
	fn test_constructed_sample() {
		let file = read_json::<MetadataMessage>(fixture!("metadata/twg-constructed-sample.json"));
		assert!(file.is_ok(), "{:?}", file);
	}

	#[test]
	#[cfg_attr(miri, ignore)]
	fn test_constructed_sample2() {
		let file = read_json::<MetadataMessage>(fixture!("metadata/twg-constructed-sample2.json"));
		assert!(file.is_ok(), "{:?}", file);
	}

	#[test]
	#[cfg_attr(miri, ignore)]
	fn test_generated_sample() {
		let file = read_json::<MetadataMessage>(fixture!("metadata/twg-generated-sample.json"));
		assert!(file.is_ok(), "{:?}", file);
	}
}

#[cfg(test)]
mod tests_structure_message {
	use super::*;
	use sdmx_json::structure::StructureMessage;

	#[test]
	#[cfg_attr(miri, ignore)]
	fn test_constructed_sample() {
		let file = read_json::<StructureMessage>(fixture!("structure/twg-constructed-sample.json"));
		assert!(file.is_ok(), "{:?}", file);
	}

	#[test]
	#[cfg_attr(miri, ignore)]
	fn test_generated_sample() {
		let file = read_json::<StructureMessage>(fixture!("structure/twg-generated-sample.json"));
		assert!(file.is_ok(), "{:?}", file);
	}
}

use crate::primitives::{
	Action, Annotation, DataType, Link, LocalizedText, Meta, NumberOrString, SdmxMessage,
	SdmxValue, StatusMessage,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::str::FromStr;

/// The top-level type of a JSON file that conforms to the
/// SDMX-JSON Metadata Message format.
///
/// # Deserializing
/// ## From a string
/// ```no_run
/// use std::str::FromStr;
/// use std::fs::read_to_string;
/// use sdmx_json::metadata::MetadataMessage;
///
/// fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
///     let file = read_to_string("sdmx-data.json")?;
///     let message = MetadataMessage::from_str(file.as_str())?;
///     Ok(())
/// }
/// ```
///
/// ## From a byte slice
/// ```no_run
/// use std::fs::read;
/// use sdmx_json::metadata::MetadataMessage;
///
/// fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
///     let file = read("sdmx-data.json")?;
///     let message = MetadataMessage::try_from(file.as_slice())?;
///     Ok(())
/// }
/// ```
///
/// ## From a `serde_json::Value`
/// ```no_run
/// use serde_json::json;
/// use sdmx_json::metadata::MetadataMessage;
///
/// fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
///     let value = json!({}); // assuming this has content
///     let message = MetadataMessage::try_from(value);
///     Ok(())
/// }
/// ```
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
pub struct MetadataMessage {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub meta: Option<Meta>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub data: Option<Data>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub errors: Option<Vec<StatusMessage>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(flatten)]
	pub other: Option<HashMap<String, Value>>,
}

impl SdmxMessage for MetadataMessage {
	type Data = Data;
	fn meta(&self) -> Option<&Meta> {
		self.meta.as_ref()
	}

	fn data(&self) -> Option<&Self::Data> {
		self.data.as_ref()
	}

	fn errors(&self) -> Option<&Vec<StatusMessage>> {
		self.errors.as_ref()
	}
}

impl<'a> TryFrom<&'a [u8]> for MetadataMessage {
	type Error = serde_json::Error;
	fn try_from(slice: &'a [u8]) -> Result<Self, Self::Error> {
		serde_json::from_slice(slice)
	}
}

impl FromStr for MetadataMessage {
	type Err = serde_json::Error;
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		serde_json::from_str(s)
	}
}

impl TryFrom<Value> for MetadataMessage {
	type Error = serde_json::Error;
	fn try_from(value: Value) -> Result<Self, Self::Error> {
		serde_json::from_value(value)
	}
}

/// The associated data with a metadata message.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Data {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub metadata_sets: Option<Vec<MetadataSet>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(flatten)]
	pub other: Option<HashMap<String, Value>>,
}

/// A collection of reported metadata against a set of values
/// for a given full or partial target identifier,
/// as described in a metadata structure definition.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct MetadataSet {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub action: Option<Action>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub publication_period: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub publication_year: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub reporting_begin: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub reporting_year: Option<String>,
	pub id: String,
	#[serde(rename = "agencyID")]
	pub agency_id: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub version: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub is_external_reference: Option<bool>,
	pub metadataflow: Option<String>,
	pub metadata_provision_agreement: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub valid_from: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub valid_to: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub annotations: Option<Vec<Annotation>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub links: Option<Vec<Link>>,
	pub name: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub names: Option<LocalizedText>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub descriptions: Option<LocalizedText>,
	pub targets: Vec<String>,
	pub attributes: Vec<Attribute>,
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(flatten)]
	pub other: Option<HashMap<String, Value>>,
}

/// A reported metadata attribute value.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
pub struct Attribute {
	pub id: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub annotations: Option<Vec<Annotation>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub format: Option<Format>,
	pub value: Option<SdmxValue>,
	pub attributes: Option<Vec<Attribute>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(flatten)]
	pub other: Option<HashMap<String, Value>>,
}

/// The representation for a component which describes
/// the possible content for component values.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
pub struct Format {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub data_type: Option<DataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub is_sequence: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub interval: Option<isize>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub start_time: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub end_time: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub min_length: Option<usize>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub max_length: Option<usize>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub min_value: Option<isize>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub max_value: Option<isize>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub decimals: Option<usize>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub is_multilingual: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub sentinel_values: Option<Vec<NumberOrString>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(flatten)]
	pub other: Option<HashMap<String, Value>>,
}

impl_extendable!(MetadataMessage, Data, MetadataSet, Attribute, Format);

use crate::primitives::{
	Action, Annotation, DataType, Link, LocalizedText, Meta, NumberOrString, SdmxValue,
	StatusMessage,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::str::FromStr;

/// The top-level type of a JSON file that conforms to the
/// SDMX-JSON Data Message format.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
pub struct DataMessage {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub meta: Option<Meta>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub data: Option<Data>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub error: Option<StatusMessage>,
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(flatten)]
	pub other: Option<HashMap<String, Value>>,
}

impl<'a> TryFrom<&'a [u8]> for DataMessage {
	type Error = serde_json::Error;
	fn try_from(slice: &'a [u8]) -> Result<Self, Self::Error> {
		serde_json::from_slice(slice)
	}
}

impl FromStr for DataMessage {
	type Err = serde_json::Error;
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		serde_json::from_str(s)
	}
}

impl TryFrom<Value> for DataMessage {
	type Error = serde_json::Error;
	fn try_from(value: Value) -> Result<Self, Self::Error> {
		serde_json::from_value(value)
	}
}

/// The associated data with a data message.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Data {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub structures: Option<Vec<Structure>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub data_sets: Option<Vec<DataSet>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(flatten)]
	pub other: Option<HashMap<String, Value>>,
}

/// The structural metadata for interpreting the data contained
/// in the message.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Structure {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub links: Option<Vec<Link>>,
	pub dimensions: Dimensions,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub measures: Option<Measures>,
	pub attributes: Attributes,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub annotations: Option<Vec<Annotation>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub dataset: Option<DataSet>,
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(flatten)]
	pub other: Option<HashMap<String, Value>>,
}

/// A short, convenient type alias to [`DimsMeasuresAttributes`].
pub type Dimensions = DimsMeasuresAttributes;
/// A short, convenient type alias to [`DimsMeasuresAttributes`].
pub type Measures = DimsMeasuresAttributes;
/// A short, convenient type alias to [`DimsMeasuresAttributes`].
pub type Attributes = DimsMeasuresAttributes;

/// An object which either represents multiple dimensions,
/// multiple measures, or multiple attributes.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct DimsMeasuresAttributes {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub data_set: Option<Vec<Component>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub dimension_group: Option<Vec<Component>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub series: Option<Vec<Component>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub observation: Option<Vec<Component>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(flatten)]
	pub other: Option<HashMap<String, Value>>,
}

/// A dimension, measure, or attribute used in the message.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Component {
	pub id: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub names: Option<LocalizedText>,
	pub description: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub descriptions: Option<LocalizedText>,
	pub key_position: Option<usize>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub roles: Option<Vec<String>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub is_mandatory: Option<bool>,
	pub relationship: Option<AttributeRelationship>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub format: Option<Format>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub default_value: Option<NumberOrString>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub links: Option<Vec<Link>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub annotations: Option<Vec<usize>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub values: Option<Vec<Option<ComponentValue>>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(flatten)]
	pub other: Option<HashMap<String, Value>>,
}

/// The relationship between an attribute and other data structure
/// definition components as defined in the data structure definition.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct AttributeRelationship {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub dataflow: Option<Value>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub dimensions: Option<Vec<String>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub observation: Option<Value>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub primary_measure: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub measures: Option<Vec<String>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(flatten)]
	pub other: Option<HashMap<String, Value>>,
}

/// The representation for a component.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Format {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub min_occurs: Option<usize>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub max_occurs: Option<usize>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub data_type: Option<DataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub is_sequence: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub interval: Option<String>,
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

/// A particular value for a component in a message.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
pub struct ComponentValue {
	pub id: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub names: Option<LocalizedText>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub values: Option<Vec<SdmxValue>>,
	pub description: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub descriptions: Option<LocalizedText>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub start: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub end: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub parent: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub order: Option<isize>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub links: Option<Vec<Link>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub annotations: Option<Vec<usize>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(flatten)]
	pub other: Option<HashMap<String, Value>>,
}

/// A collection of observations with meta-information
/// about the dataset (when it was published, reported,
/// how long the dataset is valid, etc).
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct DataSet {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub structure: Option<isize>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub action: Option<Action>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub reporting_begin: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub reporting_end: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub valid_from: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub valid_to: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub publication_year: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub publication_period: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub links: Option<Vec<Link>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub annotations: Option<Vec<usize>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub attributes: Option<Vec<SdmxValue>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub dimension_group_attributes: Option<HashMap<String, Vec<SdmxValue>>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub series: Option<HashMap<String, Series>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub observations: Option<HashMap<String, Vec<SdmxValue>>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(flatten)]
	pub other: Option<HashMap<String, Value>>,
}

/// A set of data points.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
pub struct Series {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub annotations: Option<Vec<usize>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub attributes: Option<Vec<SdmxValue>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub observations: Option<HashMap<String, Vec<SdmxValue>>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(flatten)]
	pub other: Option<HashMap<String, Value>>,
}

impl_extendable!(
	DataMessage,
	Data,
	Structure,
	DimsMeasuresAttributes,
	Component,
	AttributeRelationship,
	Format,
	ComponentValue,
	DataSet,
	Series,
);

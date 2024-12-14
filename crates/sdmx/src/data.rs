use crate::{
	Action, DataType, Links, LocalizedText, MetaManyReceivers, NumberOrString, SdmxObject,
	SdmxValue,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
pub struct DataMessage {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub meta: Option<MetaManyReceivers>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub data: Option<Data>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub error: Option<Error>,
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(flatten)]
	pub other: Option<HashMap<String, Value>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Data {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub structures: Option<Structure>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub data_sets: Option<DataSet>,
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(flatten)]
	pub other: Option<HashMap<String, Value>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Structure {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub links: Option<Links>,
	pub dimensions: Dimensions,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub measures: Option<Measures>,
	pub attributes: Attributes,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub annotations: Option<Vec<Annotation>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub datasets: Option<DataSet>,
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(flatten)]
	pub other: Option<HashMap<String, Value>>,
}

pub type Dimensions = DimsMeasuresAttributes;
pub type Measures = DimsMeasuresAttributes;
pub type Attributes = DimsMeasuresAttributes;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
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

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Component {
	pub id: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub names: Option<LocalizedText>,
	pub description: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub descriptions: Option<LocalizedText>,
	pub key_position: usize,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub roles: Option<Vec<String>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub is_mandatory: Option<bool>,
	pub relationship: AttributeRelationship,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub format: Option<Format>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub default_value: Option<NumberOrString>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub links: Option<Links>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub annotations: Option<Vec<String>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub values: Option<Vec<ComponentValue>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(flatten)]
	pub other: Option<HashMap<String, Value>>,
}

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

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct SentinelValue {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub value: Option<NumberOrString>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub names: Option<LocalizedText>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub descriptions: Option<LocalizedText>,
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(flatten)]
	pub other: Option<HashMap<String, Value>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
pub struct ComponentValue {
	pub id: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub names: Option<LocalizedText>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub values: Option<Vec<SdmxValue>>,
	pub description: String,
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
	pub links: Option<Links>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub annotations: Option<Vec<Annotation>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(flatten)]
	pub other: Option<HashMap<String, Value>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
pub struct Annotation {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub id: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub title: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "type")]
	pub type_: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub value: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub text: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub texts: Option<LocalizedText>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub links: Option<Links>,
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(flatten)]
	pub other: Option<HashMap<String, Value>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
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
	pub links: Option<Links>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub annotations: Option<Vec<Annotation>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub attributes: Option<Vec<Attributes>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub dimension_group_attributes: Option<SdmxObject>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub series: Option<Vec<Series>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub observations: Option<SdmxObject>,
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(flatten)]
	pub other: Option<HashMap<String, Value>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
pub struct Series {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub annotations: Option<Vec<Annotation>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub attributes: Option<Vec<SdmxValue>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub observations: Option<SdmxObject>,
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(flatten)]
	pub other: Option<HashMap<String, Value>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
pub struct Error {
	pub code: usize,
	pub title: String,
	pub titles: LocalizedText,
	pub detail: String,
	pub details: LocalizedText,
	pub links: Links,
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(flatten)]
	pub other: Option<HashMap<String, Value>>,
}

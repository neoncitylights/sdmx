use crate::data::Annotation;
use crate::{Action, DataType, Links, LocalizedText, MetaManyReceivers, NumberOrString, SdmxValue};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Default, Clone, PartialEq, Eq)]
pub struct MetadataMessage {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub meta: Option<MetaManyReceivers>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub data: Option<Data>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub errors: Option<Vec<StatusMessage>>,
}

#[derive(Serialize, Deserialize, Default, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Data {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub metadata_sets: Option<Vec<MetadataSet>>,
}

#[derive(Serialize, Deserialize, Default, Clone, PartialEq, Eq)]
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
	pub agency_id: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub version: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub is_external_reference: Option<bool>,
	pub metadataflow: String,
	pub metadata_provision_agreement: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub valid_from: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub valid_to: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub annotations: Option<Vec<Annotation>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub links: Option<Links>,
	pub name: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub names: Option<LocalizedText>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub descriptions: Option<LocalizedText>,
	pub targets: Vec<String>,
	pub attributes: Vec<Attributes>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Attributes {
	pub id: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub annotations: Option<Vec<Annotation>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub format: Option<Format>,
	pub value: SdmxValue,
	pub attributes: Vec<Attributes>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Format {
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

#[derive(Serialize, Deserialize, Default, Clone, PartialEq, Eq)]
pub struct StatusMessage {
	pub code: usize,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub title: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub titles: Option<LocalizedText>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub detail: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub details: Option<LocalizedText>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub links: Option<Links>,
}

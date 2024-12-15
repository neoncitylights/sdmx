use crate::primitives::{Annotation, Link};
use crate::structure::CommonArtefactType;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DataConstraint {
	#[serde(flatten)]
	pub artefact: CommonArtefactType,
	pub role: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub constraint_attachment: Option<ConstraintAttachment>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub cube_regions: Option<Vec<CubeRegion>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub data_key_sets: Option<Vec<DataKeySet>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub release_calendar: Option<ReleaseCalendar>,
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(flatten)]
	pub other: Option<HashMap<String, Value>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct MetadataConstraint {
	#[serde(flatten)]
	pub artefact: CommonArtefactType,
	pub role: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub constraint_attachment: Option<MetadataConstraintAttachment>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub metadata_target_regions: Option<Vec<MetadataTargetRegion>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub release_calendar: Option<ReleaseCalendar>,
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(flatten)]
	pub other: Option<HashMap<String, Value>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ConstraintAttachment {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub data_provider: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub data_structures: Option<Vec<String>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub dataflows: Option<Vec<String>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub provision_agreements: Option<Vec<String>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub simple_data_sources: Option<Vec<String>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub queryable_data_sources: Option<QueryableDataSource>,
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(flatten)]
	pub other: Option<HashMap<String, Value>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct MetadataConstraintAttachment {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub metadata_provider: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub metadata_sets: Option<Vec<String>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub metadata_structures: Option<Vec<String>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub metadataflows: Option<Vec<String>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub metadata_provision_agreements: Option<Vec<String>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub simple_data_sources: Option<Vec<String>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub queryable_data_sources: Option<QueryableDataSource>,
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(flatten)]
	pub other: Option<HashMap<String, Value>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct QueryableDataSource {
	pub is_rest_datasource: bool,
	pub is_web_service_datasource: bool,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub data_url: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub wadl_url: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub wsdl_url: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(flatten)]
	pub other: Option<HashMap<String, Value>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CubeRegion {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub annotations: Option<Vec<Annotation>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub links: Option<Vec<Link>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub include: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub components: Option<Vec<ComponentValueSet>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub key_values: Option<Vec<CubeRegionKey>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(flatten)]
	pub other: Option<HashMap<String, Value>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ComponentValueSet {
	pub id: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub include: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub remove_prefix: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub time_range: Option<TimeRangeValue>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub values: Option<Vec<StringOrScv>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(flatten)]
	pub other: Option<HashMap<String, Value>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum StringOrScv {
	String(String),
	SimpleComponent(SimpleComponentValue),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct TimeRangeValue {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub after_period: Option<TimePeriodRange>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub before_period: Option<TimePeriodRange>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub end_period: Option<TimePeriodRange>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub start_period: Option<TimePeriodRange>,
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(flatten)]
	pub other: Option<HashMap<String, Value>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct TimePeriodRange {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub period: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub is_inclusive: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(flatten)]
	pub other: Option<HashMap<String, Value>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SimpleComponentValue {
	pub value: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub lang: Option<String>,
	pub cascade_values: CascadeValues,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub valid_from: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub valid_to: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(flatten)]
	pub other: Option<HashMap<String, Value>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum CascadeValues {
	String(String),
	#[serde(rename = "excluderoot")]
	ExcludeRoot,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CubeRegionKey {
	pub id: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub include: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub remove_prefix: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub valid_from: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub valid_to: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub time_range: Option<TimeRangeValue>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub values: Option<Vec<StringOrScv>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(flatten)]
	pub other: Option<HashMap<String, Value>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DataKeySet {
	pub is_included: bool,
	pub keys: Vec<DataKey>,
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(flatten)]
	pub other: Option<HashMap<String, Value>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DataKey {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub annotations: Option<Vec<Annotation>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub links: Option<Vec<Link>>,
	pub include: bool,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub valid_from: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub valid_to: Option<String>,
	pub key_values: Vec<DataKeyValue>,
	pub components: Vec<DataComponentValueSet>,
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(flatten)]
	pub other: Option<HashMap<String, Value>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DataKeyValue {
	pub id: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub include: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub remove_prefix: Option<bool>,
	pub value: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(flatten)]
	pub other: Option<HashMap<String, Value>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DataComponentValueSet {
	pub id: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub include: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub remove_prefix: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub time_range: Option<TimeRangeValue>,
	pub values: Vec<StringOrDcv>,
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(flatten)]
	pub other: Option<HashMap<String, Value>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum StringOrDcv {
	String(String),
	Dcv(DataComponentValue),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DataComponentValue {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub cascade_values: Option<CascadeValues>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub lang: Option<String>,
	pub value: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(flatten)]
	pub other: Option<HashMap<String, Value>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct MetadataTargetRegion {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub annotations: Option<Vec<Annotation>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub links: Option<Vec<Link>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub include: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub components: Option<Vec<MetadataAttributeValueSet>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub valid_from: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub valid_to: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(flatten)]
	pub other: Option<HashMap<String, Value>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct MetadataAttributeValueSet {
	pub id: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub include: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub remove_prefix: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub time_range: Option<TimeRangeValue>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub values: Option<Vec<StringOrScv>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(flatten)]
	pub other: Option<HashMap<String, Value>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct ReleaseCalendar {
	pub offset: String,
	pub periodicity: String,
	pub tolerance: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(flatten)]
	pub other: Option<HashMap<String, Value>>,
}

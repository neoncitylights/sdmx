use crate::{Annotation, Links};
use crate::structure::CommonArtefactType;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DataConstraint {
	#[serde(flatten)]
	pub artefact: CommonArtefactType,
	pub role: String,
	pub constraint_attachment: Option<ConstraintAttachment>,
	pub cube_regions: Option<Vec<CubeRegion>>,
	pub data_key_sets: Option<Vec<DataKeySet>>,
	pub release_calendar: Option<ReleaseCalendar>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct MetadataConstraint {
	#[serde(flatten)]
	pub artefact: CommonArtefactType,
	pub role: String,
	pub constraint_attachment: Option<MetadataConstraintAttachment>,
	pub metadata_target_regions: Option<Vec<MetadataTargetRegion>>,
	pub release_calendar: Option<ReleaseCalendar>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ConstraintAttachment {
	pub data_provider: Option<String>,
	pub data_structures: Option<Vec<String>>,
	pub dataflows: Option<Vec<String>>,
	pub provision_agreements: Option<Vec<String>>,
	pub simple_data_sources: Option<Vec<String>>,
	pub queryable_data_sources: Option<QueryableDataSource>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct MetadataConstraintAttachment {
	pub metadata_provider: Option<String>,
	pub metadata_sets: Option<Vec<String>>,
	pub metadata_structures: Option<Vec<String>>,
	pub metadataflows: Option<Vec<String>>,
	pub metadata_provision_agreements: Option<Vec<String>>,
	pub simple_data_sources: Option<Vec<String>>,
	pub queryable_data_sources: Option<QueryableDataSource>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct QueryableDataSource {
	pub is_rest_datasource: bool,
	pub is_web_service_datasource: bool,
	pub data_url: Option<String>,
	pub wadl_url: Option<String>,
	pub wsdl_url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CubeRegion {
	pub annotations: Option<Vec<Annotation>>,
	pub links: Option<Links>,
	pub include: Option<bool>,
	pub components: Option<Vec<ComponentValueSet>>,
	pub key_values: Option<Vec<CubeRegionKey>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ComponentValueSet {
	pub id: String,
	pub include: Option<bool>,
	pub remove_prefix: Option<bool>,
	pub time_range: Option<TimeRangeValue>,
	pub values: Option<Vec<StringOrScv>>,
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
	pub after_period: Option<TimePeriodRange>,
	pub before_period: Option<TimePeriodRange>,
	pub end_period: Option<TimePeriodRange>,
	pub start_period: Option<TimePeriodRange>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct TimePeriodRange {
	pub period: Option<String>,
	pub is_inclusive: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SimpleComponentValue {
	pub value: String,
	pub lang: Option<String>,
	pub cascade_values: CascadeValues,
	pub valid_from: Option<String>,
	pub valid_to: Option<String>,
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
	pub include: Option<bool>,
	pub remove_prefix: Option<bool>,
	pub valid_from: Option<String>,
	pub valid_to: Option<String>,
	pub time_range: Option<TimeRangeValue>,
	pub values: Option<Vec<StringOrScv>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DataKeySet {
	pub is_included: bool,
	pub keys: Vec<DataKey>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DataKey {
	pub annotations: Option<Vec<Annotation>>,
	pub links: Option<Links>,
	pub include: bool,
	pub valid_from: Option<String>,
	pub valid_to: Option<String>,
	pub key_values: Vec<DataKeyValue>,
	pub components: Vec<DataComponentValueSet>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DataKeyValue {
	pub id: String,
	pub include: Option<bool>,
	pub remove_prefix: Option<bool>,
	pub value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DataComponentValueSet {
	pub id: String,
	pub include: Option<bool>,
	pub remove_prefix: Option<bool>,
	pub time_range: Option<TimeRangeValue>,
	pub values: Vec<StringOrDcv>,
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
	pub cascade_values: Option<CascadeValues>,
	pub lang: Option<String>,
	pub value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct MetadataTargetRegion {
	pub annotations: Option<Vec<Annotation>>,
	pub links: Option<Links>,
	pub include: Option<bool>,
	pub components: Option<Vec<MetadataAttributeValueSet>>,
	pub valid_from: Option<String>,
	pub valid_to: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct MetadataAttributeValueSet {
	pub id: String,
	pub include: Option<bool>,
	pub remove_prefix: Option<bool>,
	pub time_range: Option<TimeRangeValue>,
	pub values: Option<Vec<StringOrScv>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct ReleaseCalendar {
	pub offset: String,
	pub periodicity: String,
	pub tolerance: String,
}

use crate::primitives::{Annotation, Link};
use crate::structure::CommonArtefactType;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::convert::Infallible;
use std::str::FromStr;

/// A subset of the definition of the allowable (or available)
/// content of a dataset.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DataConstraint {
	#[serde(flatten)]
	pub artefact: CommonArtefactType,
	pub role: Role,
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

/// A subset of the definition of the allowable (or available)
/// content of a metadata set.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct MetadataConstraint {
	#[serde(flatten)]
	pub artefact: CommonArtefactType,
	pub role: Role,
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

/// The purpose of a constraint, which informs the constraint
/// if the data is actually present, or if it defines what
/// data is allowed.
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Role {
	Actual,
	Allowed,
}

/// A collection of references to data-constrainable artefacts.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
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

/// A collection of references to metadata-constrainable artefacts.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
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

/// A data source which accepts a standard SDMX query message
/// and responds appropriately.
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

/// A subset of data within multi-dimensional data.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
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

/// The structure for providing values for data attributes,
/// measures, or metadata attributes.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
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

/// A string or simple component value.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum StringOrScv {
	String(String),
	SimpleComponent(SimpleComponentValue),
}

impl From<String> for StringOrScv {
	fn from(value: String) -> Self {
		Self::String(value)
	}
}

impl From<SimpleComponentValue> for StringOrScv {
	fn from(value: SimpleComponentValue) -> Self {
		Self::SimpleComponent(value)
	}
}

impl From<&str> for StringOrScv {
	fn from(value: &str) -> Self {
		Self::String(value.to_owned())
	}
}

impl FromStr for StringOrScv {
	type Err = Infallible;
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		Ok(Self::String(s.to_owned()))
	}
}

/// A time period value expressed as a range.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum TimeRangeValue {
	After(TimeAfterPeriod),
	Before(TimeBeforePeriod),
	Between(TimeBetweenPeriod),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct TimeAfterPeriod {
	pub after_period: TimePeriodRange,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct TimeBeforePeriod {
	pub before_period: TimePeriodRange,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct TimeBetweenPeriod {
	pub end_period: TimePeriodRange,
	pub start_period: TimePeriodRange,
}

/// A time period that describes whether a period
/// is inclusive in a range.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
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

/// A simple value for a component.
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

/// Indicates whether a value should be cascaded.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum CascadeValues {
	Boolean(bool),
	#[serde(rename = "excluderoot")]
	ExcludeRoot,
}

impl From<bool> for CascadeValues {
	fn from(value: bool) -> Self {
		Self::Boolean(value)
	}
}

/// A set of values for a dimension that defines
/// a data cube region.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
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

/// A collection of full or partial data keys (dimension values).
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct DataKeySet {
	pub is_included: bool,
	pub keys: Vec<DataKey>,
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(flatten)]
	pub other: Option<HashMap<String, Value>>,
}

/// A region which defines a distinct full or partial data key.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
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

/// A dimension value for the purpose of defining a distinct data key.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
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

/// The structure for providing values for data attributes,
/// measures, or metadata attributes.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
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

/// A string or data component value.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum StringOrDcv {
	String(String),
	Dcv(DataComponentValue),
}

impl From<String> for StringOrDcv {
	fn from(value: String) -> Self {
		Self::String(value)
	}
}

impl From<DataComponentValue> for StringOrDcv {
	fn from(value: DataComponentValue) -> Self {
		Self::Dcv(value)
	}
}

impl From<&str> for StringOrDcv {
	fn from(value: &str) -> Self {
		Self::String(value.to_owned())
	}
}

impl FromStr for StringOrDcv {
	type Err = Infallible;
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		Ok(Self::String(s.to_owned()))
	}
}

/// A simple value for a component.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
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

/// Describes the report structure and the metadata target
/// from that structure on which the region is based.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
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

/// Describes the vaues provided for a metadata attribute.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
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

/// The timing of releases of the constrained data.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
pub struct ReleaseCalendar {
	pub offset: String,
	pub periodicity: String,
	pub tolerance: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(flatten)]
	pub other: Option<HashMap<String, Value>>,
}

impl_artefact!(DataConstraint, MetadataConstraint);

impl_extendable!(
	DataConstraint,
	MetadataConstraint,
	ConstraintAttachment,
	MetadataConstraintAttachment,
	QueryableDataSource,
	CubeRegion,
	ComponentValueSet,
	TimePeriodRange,
	SimpleComponentValue,
	CubeRegionKey,
	DataKeySet,
	DataKey,
	DataKeyValue,
	DataComponentValueSet,
	DataComponentValue,
	MetadataTargetRegion,
	MetadataAttributeValueSet,
	ReleaseCalendar,
);

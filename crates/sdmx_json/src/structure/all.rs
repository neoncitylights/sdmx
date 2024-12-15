use crate::primitives::{
	Annotation, DataType, Error, Link, LocalizedText, MetaSingleReceiver, SentinelValue,
};
use crate::structure::{CommonArtefactType, DataConstraint, MetadataConstraint};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct StructureMessage {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub meta: Option<MetaSingleReceiver>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub data: Option<Data>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub errors: Option<Vec<Error>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(flatten)]
	pub other: Option<HashMap<String, Value>>,
}

impl<'a> TryFrom<&'a [u8]> for StructureMessage {
	type Error = serde_json::Error;
	fn try_from(slice: &'a [u8]) -> Result<Self, Self::Error> {
		serde_json::from_slice(slice)
	}
}

impl FromStr for StructureMessage {
	type Err = serde_json::Error;
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		serde_json::from_str(s)
	}
}

impl TryFrom<Value> for StructureMessage {
	type Error = serde_json::Error;
	fn try_from(value: Value) -> Result<Self, Self::Error> {
		serde_json::from_value(value)
	}
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Data {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub data_structures: Option<Vec<DataStructure>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub metadata_structures: Option<Vec<MetadataStructure>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub category_schemas: Option<Vec<CategoryScheme>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub concept_schemas: Option<Vec<ConceptScheme>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub concepts: Option<Vec<Codelist>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub geographic_codelists: Option<Vec<Codelist>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub geo_grid_codelists: Option<Vec<Codelist>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub value_lists: Option<Vec<CommonArtefactType>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub hierarchies: Option<Vec<CommonArtefactType>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub hierarchy_associations: Option<Vec<CommonArtefactType>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub agency_schemes: Option<Vec<AgencyScheme>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub data_provider_schemes: Option<Vec<DataProviderScheme>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub data_consumer_schemes: Option<Vec<DataConsumerScheme>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub metadata_provider_schemes: Option<Vec<MetadataProviderScheme>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub organisation_unit_schemes: Option<Vec<OrganizationUnitScheme>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub dataflows: Option<Vec<Dataflow>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub metadataflows: Option<Vec<CommonArtefactType>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub reporting_taxonomies: Option<Vec<ReportingTaxonomy>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub provision_agreements: Option<Vec<CommonArtefactType>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub metadata_provision_agreements: Option<Vec<CommonArtefactType>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub structure_maps: Option<Vec<CommonArtefactType>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub representation_maps: Option<Vec<CommonArtefactType>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub concept_scheme_maps: Option<Vec<CommonArtefactType>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub category_scheme_maps: Option<Vec<CommonArtefactType>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub organisation_scheme_maps: Option<Vec<CommonArtefactType>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub reporting_taxonomy_maps: Option<Vec<CommonArtefactType>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub processes: Option<Vec<CommonArtefactType>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub categorisations: Option<Vec<Categorization>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub data_constraints: Option<Vec<DataConstraint>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub metadata_constraints: Option<Vec<MetadataConstraint>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub custom_type_schemes: Option<Vec<CustomTypeScheme>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub vtl_mapping_schemes: Option<Vec<VtlMappingScheme>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name_personalisation_schemes: Option<Vec<NamePersonalizationScheme>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ruleset_schemes: Option<Vec<RulesetScheme>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub transformation_schemes: Option<Vec<TransformationScheme>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub user_defined_operator_schemes: Option<Vec<UserDefinedOperatorsScheme>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(flatten)]
	pub other: Option<HashMap<String, Value>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum ArtefactType {
	DataStructures(Box<DataStructure>),
	MetadataStructures(Box<MetadataStructure>),
	CategorySchemes(Box<CategoryScheme>),
	ConceptSchemes(Box<ConceptScheme>),
	Codelists(Box<Codelist>),
	GeographicCodelists(Box<GeographyCodelist>),
	GeoGridCodelists(Box<GeoGridCodelist>),
	ValueLists(Box<CommonArtefactType>),
	Hierarchies(Box<CommonArtefactType>),
	HierarchyAssociations(Box<CommonArtefactType>),
	AgencySchemes(Box<AgencyScheme>),
	DataProviderSchemes(Box<DataProviderScheme>),
	DataConsumerSchemes(Box<DataConsumerScheme>),
	MetadataProviderSchemes(Box<MetadataProviderScheme>),
	OrganisationUnitSchemes(Box<OrganizationUnitScheme>),
	Dataflows(Box<Dataflow>),
	Metadataflows(Box<CommonArtefactType>),
	ReportingTaxonomies(Box<ReportingTaxonomy>),
	ProvisionAgreements(Box<CommonArtefactType>),
	MetadataProvisionAgreements(Box<CommonArtefactType>),
	StructureMaps(Box<CommonArtefactType>),
	RepresentationMaps(Box<CommonArtefactType>),
	ConceptSchemeMaps(Box<CommonArtefactType>),
	CategorySchemeMaps(Box<CommonArtefactType>),
	OrganisationSchemeMaps(Box<CommonArtefactType>),
	ReportingTaxonomyMaps(Box<CommonArtefactType>),
	Processes(Box<CommonArtefactType>),
	Categorisations(Box<Categorization>),
	DataConstraints(Box<DataConstraint>),
	MetadataConstraints(Box<MetadataConstraint>),
	CustomTypeSchemes(Box<CustomTypeScheme>),
	VtlMappingSchemes(Box<VtlMappingScheme>),
	NamePersonalisationSchemes(Box<NamePersonalizationScheme>),
	RulesetSchemes(Box<RulesetScheme>),
	TransformationSchemes(Box<TransformationScheme>),
	UserDefinedOperatorSchemes(Box<UserDefinedOperatorsScheme>),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Item {
	pub id: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub names: Option<LocalizedText>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub descriptions: Option<LocalizedText>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub annotations: Option<Vec<Annotation>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub links: Option<Vec<Link>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(flatten)]
	pub other: Option<HashMap<String, Value>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DataStructure {
	#[serde(flatten)]
	pub common: CommonArtefactType,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub data_structure_components: Option<DataStructureComponents>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub metadata: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(flatten)]
	pub other: Option<HashMap<String, Value>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DataStructureComponents {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub attribute_list: Option<AttributeList>,
	pub dimension_list: DimensionList,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub groups: Option<Vec<Group>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub measure_list: Option<MeasureList>,
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(flatten)]
	pub other: Option<HashMap<String, Value>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AttributeList {
	pub id: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub annotations: Option<Vec<Annotation>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub links: Option<Vec<Link>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub attributes: Option<Vec<Attribute>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub metadata_attribute_usages: Option<Vec<MetadataAttributeUsage>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(flatten)]
	pub other: Option<HashMap<String, Value>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Attribute {
	pub id: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub annotations: Option<Vec<Annotation>>,
	pub links: Option<Vec<Link>>,
	pub usage: Usage,
	pub attribute_relationship: AttributeRelationship,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub measure_relationship: Option<Vec<String>>,
	pub concept_identity: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub concept_roles: Option<Vec<String>>,
	pub local_representation: LocalRepresentation,
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(flatten)]
	pub other: Option<HashMap<String, Value>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Usage {
	Mandatory,
	Optional,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AttributeRelationship {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub dataflow: Option<()>, // TODO fix this
	#[serde(skip_serializing_if = "Option::is_none")]
	pub dimensions: Option<Vec<String>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub are_dimensions_optional: Option<Vec<bool>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub group: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub observation: Option<()>, // TODO fix this
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(flatten)]
	pub other: Option<HashMap<String, Value>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct LocalRepresentation {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub enumeration: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub enumeration_format: Option<EnumerationFormat>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub format: Option<Format>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub min_occurs: Option<usize>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub max_occurs: Option<MaxOccurs>,
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(flatten)]
	pub other: Option<HashMap<String, Value>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum MaxOccurs {
	Signed(usize),
	Unbounded,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct EnumerationFormat {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub data_type: Option<DataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub is_sequence: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub interval: Option<isize>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub start_value: Option<isize>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub end_value: Option<isize>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub time_interval: Option<String>,
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
	pub pattern: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(flatten)]
	pub other: Option<HashMap<String, Value>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Format {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub data_type: Option<DataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub is_sequence: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub interval: Option<isize>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub start_value: Option<isize>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub end_value: Option<isize>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub time_interval: Option<String>,
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
	pub is_multilingual: bool,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub sentinel_value: Option<Vec<SentinelValue>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(flatten)]
	pub other: Option<HashMap<String, Value>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct MetadataAttributeUsage {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub annotations: Option<Vec<Annotation>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub links: Option<Vec<Link>>,
	pub metadata_attribute_reference: String,
	pub attribute_relationship: AttributeRelationship,
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(flatten)]
	pub other: Option<HashMap<String, Value>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DimensionList {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub id: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub annotations: Option<Vec<String>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub links: Option<Vec<Link>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub dimensions: Option<Vec<Dimension>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub time_dimensions: Option<TimeDimension>,
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(flatten)]
	pub other: Option<HashMap<String, Value>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Dimension {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub id: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub annotations: Option<Vec<Annotation>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub links: Option<Vec<Link>>,
	pub position: usize,
	pub concept_identity: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub concept_roles: Option<Vec<String>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub local_representation: Option<LocalRepresentation>,
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(flatten)]
	pub other: Option<HashMap<String, Value>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct TimeDimension {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub id: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub annotations: Option<Vec<Annotation>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub links: Option<Vec<Link>>,
	pub concept_identity: String,
	pub local_representation: LocalRepresentation,
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(flatten)]
	pub other: Option<HashMap<String, Value>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct TimeDimensionFormat {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub start_time: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub end_time: Option<String>,
	pub data_type: TimeDimensionDataType,
	pub sentinel_values: Option<Vec<SentinelValue>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(flatten)]
	pub other: Option<HashMap<String, Value>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimeDimensionDataType {
	ObservationalTimePeriod,
	StandardTimePeriod,
	BasicTimePeriod,
	GregorianTimePeriod,
	GregorianYear,
	GregorianYearMonth,
	GregorianDay,
	ReportingTimePeriod,
	ReportingYear,
	ReportingSemester,
	ReportingTrimester,
	ReportingQuarter,
	ReportingMonth,
	ReportingWeek,
	ReportingDay,
	DateTime,
	TimeRange,
}

impl TimeDimensionDataType {
	pub const fn is_reporting(&self) -> bool {
		matches!(
			self,
			Self::ReportingDay
				| Self::ReportingWeek
				| Self::ReportingMonth
				| Self::ReportingYear
				| Self::ReportingQuarter
				| Self::ReportingSemester
				| Self::ReportingTrimester
				| Self::ReportingTimePeriod
		)
	}

	pub const fn is_gregorian(&self) -> bool {
		matches!(
			self,
			Self::GregorianTimePeriod
				| Self::GregorianYear
				| Self::GregorianYearMonth
				| Self::GregorianDay
		)
	}
}

impl TryFrom<DataType> for TimeDimensionDataType {
	type Error = ();
	fn try_from(value: DataType) -> Result<Self, Self::Error> {
		match value {
			DataType::ObservationalTimePeriod => Ok(Self::ObservationalTimePeriod),
			DataType::StandardTimePeriod => Ok(Self::StandardTimePeriod),
			DataType::BasicTimePeriod => Ok(Self::BasicTimePeriod),
			DataType::GregorianTimePeriod => Ok(Self::GregorianTimePeriod),
			DataType::GregorianYear => Ok(Self::GregorianYear),
			DataType::GregorianYearMonth => Ok(Self::GregorianYearMonth),
			DataType::GregorianDay => Ok(Self::GregorianDay),
			DataType::ReportingTimePeriod => Ok(Self::ReportingTimePeriod),
			DataType::ReportingYear => Ok(Self::ReportingYear),
			DataType::ReportingSemester => Ok(Self::ReportingSemester),
			DataType::ReportingTrimester => Ok(Self::ReportingTrimester),
			DataType::ReportingQuarter => Ok(Self::ReportingQuarter),
			DataType::ReportingMonth => Ok(Self::ReportingMonth),
			DataType::ReportingWeek => Ok(Self::ReportingWeek),
			DataType::ReportingDay => Ok(Self::ReportingDay),
			DataType::DateTime => Ok(Self::DateTime),
			DataType::TimeRange => Ok(Self::TimeRange),
			_ => Err(()),
		}
	}
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Group {
	pub id: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub annotations: Option<Vec<Annotation>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub links: Option<Vec<Link>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub group_dimensions: Option<Vec<String>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(flatten)]
	pub other: Option<HashMap<String, Value>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct MeasureList {
	pub id: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub annotations: Option<Vec<Annotation>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub links: Option<Vec<Link>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub measures: Option<Vec<Measure>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(flatten)]
	pub other: Option<HashMap<String, Value>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Measure {
	pub id: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub annotations: Option<Vec<Annotation>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub links: Option<Vec<Link>>,
	pub concept_identity: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub concept_roles: Option<Vec<String>>,
	pub local_representation: LocalRepresentation,
	pub usage: Usage,
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(flatten)]
	pub other: Option<HashMap<String, Value>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct MetadataStructure {
	#[serde(flatten)]
	pub common: CommonArtefactType,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub metadata_structure_components: Option<MetadataStructureComponents>,
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(flatten)]
	pub other: Option<HashMap<String, Value>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct MetadataStructureComponents {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub metadata_attribute_list: Option<MetadataAttributeList>,
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(flatten)]
	pub other: Option<HashMap<String, Value>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct MetadataAttributeList {
	pub id: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub annotations: Option<Vec<Annotation>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub links: Option<Vec<Link>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub metadata_attributes: Option<Vec<MetadataAttribute>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(flatten)]
	pub other: Option<HashMap<String, Value>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct MetadataAttribute {
	pub id: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub annotations: Option<Vec<Annotation>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub links: Option<Vec<Link>>,
	pub concept_identity: String,
	// TODO: this local repr can't have min_occurs/max_occurs
	pub local_representation: LocalRepresentation,
	pub min_occurs: usize,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub max_occurs: Option<MaxOccurs>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub is_presentational: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub metadata_attributes: Option<Vec<MetadataAttribute>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(flatten)]
	pub other: Option<HashMap<String, Value>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CategoryScheme {
	#[serde(flatten)]
	pub artefact: CommonArtefactType,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub is_partial: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub categories: Option<Vec<Item>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(flatten)]
	pub other: Option<HashMap<String, Value>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ConceptScheme {
	#[serde(flatten)]
	pub artefact: CommonArtefactType,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub is_partial: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub concepts: Option<Vec<Item>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub core_representation: Option<CoreRepresentation>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub iso_concept_reference: Option<IsoConceptReference>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub parent: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(flatten)]
	pub other: Option<HashMap<String, Value>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CoreRepresentation {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub enumeration: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub enumeration_format: Option<EnumerationFormat>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub format: Option<Format>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub min_occurs: Option<usize>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub max_occurs: Option<MaxOccurs>,
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(flatten)]
	pub other: Option<HashMap<String, Value>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct IsoConceptReference {
	pub concept_agency: String,
	pub concept_id: String,
	pub concept_scheme_id: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(flatten)]
	pub other: Option<HashMap<String, Value>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Codelist {
	#[serde(flatten)]
	pub artefact: CommonArtefactType,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub is_partial: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub codes: Option<Vec<Item>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub parent: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(flatten)]
	pub other: Option<HashMap<String, Value>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct GeographyCodelist {
	#[serde(flatten)]
	pub artefact: CommonArtefactType,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub is_partial: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub geo_feature_set_codes: Option<Vec<Item>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(flatten)]
	pub other: Option<HashMap<String, Value>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct GeoGridCodelist {
	#[serde(flatten)]
	pub artefact: CommonArtefactType,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub is_partial: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub geo_grid_codes: Option<Vec<Item>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(flatten)]
	pub other: Option<HashMap<String, Value>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AgencyScheme {
	#[serde(flatten)]
	pub artefact: CommonArtefactType,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub is_partial: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub agencies: Option<Vec<Item>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(flatten)]
	pub other: Option<HashMap<String, Value>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DataProviderScheme {
	#[serde(flatten)]
	pub artefact: CommonArtefactType,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub is_partial: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub data_providers: Option<Vec<Item>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(flatten)]
	pub other: Option<HashMap<String, Value>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DataConsumerScheme {
	#[serde(flatten)]
	pub artefact: CommonArtefactType,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub is_partial: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub data_consumers: Option<Vec<Item>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(flatten)]
	pub other: Option<HashMap<String, Value>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct MetadataProviderScheme {
	#[serde(flatten)]
	pub artefact: CommonArtefactType,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub is_partial: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub metadata_providers: Option<Vec<Item>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(flatten)]
	pub other: Option<HashMap<String, Value>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct OrganizationUnitScheme {
	#[serde(flatten)]
	pub artefact: CommonArtefactType,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub is_partial: Option<bool>,
	#[serde(alias = "organisationUnits")]
	pub organization_units: Option<Vec<Item>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(flatten)]
	pub other: Option<HashMap<String, Value>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Dataflow {
	#[serde(flatten)]
	pub artefact: CommonArtefactType,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub structure: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(flatten)]
	pub other: Option<HashMap<String, Value>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct NamePersonalizationScheme {
	#[serde(flatten)]
	pub artefact: CommonArtefactType,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub is_partial: Option<bool>,
	#[serde(alias = "namePersonalisations")]
	pub name_personalizations: Option<Vec<Item>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(flatten)]
	pub other: Option<HashMap<String, Value>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ReportingTaxonomy {
	#[serde(flatten)]
	pub artefact: CommonArtefactType,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub is_partial: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub reporting_categories: Option<Vec<Item>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(flatten)]
	pub other: Option<HashMap<String, Value>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Categorization {
	#[serde(flatten)]
	pub artefact: CommonArtefactType,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub source: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub target: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(flatten)]
	pub other: Option<HashMap<String, Value>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CustomTypeScheme {
	#[serde(flatten)]
	pub artefact: CommonArtefactType,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub is_partial: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub custom_types: Option<Vec<Item>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(flatten)]
	pub other: Option<HashMap<String, Value>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct VtlMappingScheme {
	#[serde(flatten)]
	pub artefact: CommonArtefactType,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub is_partial: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub vtl_mappings: Option<Vec<Item>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(flatten)]
	pub other: Option<HashMap<String, Value>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RulesetScheme {
	#[serde(flatten)]
	pub artefact: CommonArtefactType,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub is_partial: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub rulesets: Option<Vec<Item>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(flatten)]
	pub other: Option<HashMap<String, Value>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct TransformationScheme {
	#[serde(flatten)]
	pub artefact: CommonArtefactType,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub is_partial: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub transformations: Option<Vec<Item>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(flatten)]
	pub other: Option<HashMap<String, Value>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct UserDefinedOperatorsScheme {
	#[serde(flatten)]
	pub artefact: CommonArtefactType,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub is_partial: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub user_defined_operators: Option<Vec<Item>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(flatten)]
	pub other: Option<HashMap<String, Value>>,
}

impl_extendable!(
	StructureMessage,
	Data,
	Item,
	DataStructure,
	DataStructureComponents,
	AttributeList,
	Attribute,
	AttributeRelationship,
	EnumerationFormat,
	Format,
	MetadataAttributeUsage,
	Dimension,
	TimeDimension,
	TimeDimensionFormat,
	Group,
	MeasureList,
	Measure,
	MetadataStructure,
	MetadataAttributeList,
	MetadataAttribute,
	CategoryScheme,
	ConceptScheme,
	CoreRepresentation,
	IsoConceptReference,
	Codelist,
	GeographyCodelist,
	GeoGridCodelist,
	AgencyScheme,
	DataProviderScheme,
	DataConsumerScheme,
	MetadataProviderScheme,
	OrganizationUnitScheme,
	Dataflow,
	NamePersonalizationScheme,
	ReportingTaxonomy,
	Categorization,
	CustomTypeScheme,
	VtlMappingScheme,
	RulesetScheme,
	TransformationScheme,
	UserDefinedOperatorsScheme,
);

impl_artefact! {
	CategoryScheme,
	ConceptScheme,
	Codelist,
	GeographyCodelist,
	GeoGridCodelist,
	AgencyScheme,
	DataProviderScheme,
	DataConsumerScheme,
	MetadataProviderScheme,
	OrganizationUnitScheme,
	Dataflow,
	NamePersonalizationScheme,
	ReportingTaxonomy,
	Categorization,
	CustomTypeScheme,
	VtlMappingScheme,
	RulesetScheme,
	TransformationScheme,
	UserDefinedOperatorsScheme,
}

impl_item_scheme! {
	(ConceptScheme, concepts),
	(CategoryScheme, categories),
	(Codelist, codes),
	(GeographyCodelist, geo_feature_set_codes),
	(GeoGridCodelist, geo_grid_codes),
	(AgencyScheme, agencies),
	(DataProviderScheme, data_providers),
	(DataConsumerScheme, data_consumers),
	(MetadataProviderScheme, metadata_providers),
	(OrganizationUnitScheme, organization_units),
	(NamePersonalizationScheme, name_personalizations),
	(ReportingTaxonomy, reporting_categories),
	(CustomTypeScheme, custom_types),
	(VtlMappingScheme, vtl_mappings),
	(RulesetScheme, rulesets),
	(TransformationScheme, transformations),
	(UserDefinedOperatorsScheme, user_defined_operators),
}

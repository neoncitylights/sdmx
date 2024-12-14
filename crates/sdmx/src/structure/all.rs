use crate::structure::{CommonArtefactType, DataConstraint, MetadataConstraint};
use crate::{Annotation, DataType, Links, LocalizedText, SentinelValue};
use serde::{Deserialize, Serialize};

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
	pub links: Option<Links>,
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
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AttributeList {
	pub id: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub annotations: Option<Vec<Annotation>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub links: Option<Links>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub attributes: Option<Vec<Attribute>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub metadata_attribute_usages: Option<Vec<MetadataAttributeUsage>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Attribute {
	pub id: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub annotations: Option<Vec<Annotation>>,
	pub links: Option<Links>,
	pub usage: Usage,
	pub attribute_relationship: AttributeRelationship,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub measure_relationship: Option<Vec<String>>,
	pub concept_identity: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub concept_roles: Option<Vec<String>>,
	pub local_representation: LocalRepresentation,
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
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct MetadataAttributeUsage {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub annotations: Option<Vec<Annotation>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub links: Option<Links>,
	pub metadata_attribute_reference: String,
	pub attribute_relationship: AttributeRelationship,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DimensionList {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub id: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub annotations: Option<Vec<String>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub links: Option<Links>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub dimensions: Option<Vec<Dimension>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub time_dimensions: Option<TimeDimension>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Dimension {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub id: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub annotations: Option<Vec<Annotation>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub links: Option<Links>,
	pub position: usize,
	pub concept_identity: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub concept_roles: Option<Vec<String>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub local_representation: Option<LocalRepresentation>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct TimeDimension {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub id: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub annotations: Option<Vec<Annotation>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub links: Option<Links>,
	pub concept_identity: String,
	pub local_representation: LocalRepresentation,
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

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Group {
	pub id: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub annotations: Option<Vec<Annotation>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub links: Option<Links>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub group_dimensions: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct MeasureList {
	pub id: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub annotations: Option<Vec<Annotation>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub links: Option<Links>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub measures: Option<Vec<Measure>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Measure {
	pub id: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub annotations: Option<Vec<Annotation>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub links: Option<Links>,
	pub concept_identity: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub concept_roles: Option<Vec<String>>,
	pub local_representation: LocalRepresentation,
	pub usage: Usage,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct MetadataStructure {
	#[serde(flatten)]
	pub common: CommonArtefactType,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub metadata_structure_components: Option<MetadataStructureComponents>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct MetadataStructureComponents {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub metadata_attribute_list: Option<MetadataAttributeList>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct MetadataAttributeList {
	pub id: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub annotations: Option<Vec<Annotation>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub links: Option<Links>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub metadata_attributes: Option<Vec<MetadataAttribute>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct MetadataAttribute {
	pub id: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub annotations: Option<Vec<Annotation>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub links: Option<Links>,
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
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct IsoConceptReference {
	pub concept_agency: String,
	pub concept_id: String,
	pub concept_scheme_id: String,
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
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Dataflow {
	#[serde(flatten)]
	pub artefact: CommonArtefactType,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub structure: Option<String>,
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
}

# Changelog

## v0.4.0 (Unreleased)

### Breaking changes
This version includes changes to align with the original JSON schema.
- structure: The enum type `MaxOccurs` was renamed to `Occurrence`.
- structure: The enum type `TimeDimensionDataType` was renamed to `TimeDataType`.
- structure: The following changes have been made to `MetadataAttribute` type:
  - The `local_representation` field is now an optional type (aka `Option`).
  - The inner type of `local_representation` field is now its own type, `MetadataAttributeRepresentation` (instead of `LocalRepresentation`.

## v0.3.0 (2024-12-22)

### Breaking changes
- structure: The definition of the `AttributeRelationship` type is now more closely aligned with the original JSON schema for structure messages, which can only be a single variant of something at a time.

### Features
- The following types now implement the `Default` trait:
  - metadata: `Attribute`, `Format`
  - primitives: `MetaManyReceivers`, `MetaSingleReceiver`
  - structure: `AgencyScheme`, `AttributeList`, `CommonArtefactType`, `ComponentValueSet`, `ConstraintAttachment`, `CoreRepresentation`, `CubeRegionKey`, `CubeRegion`, `CustomTypeScheme`, `DataComponentValueSet`, `DataComponentValue`, `DataKeyValue`, `DataKey`, `DataProviderScheme`, `DataStructureComponents`, `DataStructure`, `Data`, `Dataflow`, `DimensionList`, `EnumerationFormat`, `Group`, `IsoConceptReference`, `Item`, `LocalRepresentation`, `MeasureList`, `MetadataAttributeList`, `MetadataAttributeValueSet`, `MetadataAttribute`, `MetadataConstraintAttachment`, `MetadataTargetRegion`, `OrganizationUnitScheme`, `ReleaseCalendar`, `TimeDimension`, `TimePeriodRange`, `TimeRangeValue`, `TransformationScheme`, `UserDefinedOperatorsScheme`, `VtlMappingScheme`

## v0.2.0 (2024-12-16)

### Breaking changes
- metadata: The type `Attributes` is now correctly named `Attribute`.
- primitives: The types `crate::metadata::StatusMessage` and `crate::primitives::Error` were consolidated into one type, `crate::primitives::StatusMessage`.
- structure: The `CascadeValues` variant `String` was replaced with a `Boolean` variant.

### Bugfixes
- data: In `DataSet`, the fields `dimension_group_attributes`, `series`, and `observations` were fixed to have a less generic type.
- data: In `Series`, the field type `annotations` was corrected to be a collection of indices that reference annotations (from `Option<Vec<Annotation>>` to `Option<Vec<usize>>`).
- data: In `Series`, the field type `observations` was corrected to be less generic, from `Option<SdmxObject>` to `Option<HashMap<String, Vec<SdmxValue>>>`.
- primitives: In `StatusMessage`, the `code` field is changed from `f64` to `u16` (smallest size possible for representing HTTP status codes). It assumes this is an error in the original specification / JSON schema, which specified `number` (which can be an integer or floating point), instead of `integer`. **Note** that this also patches the original sample files, which are presumed to have been caused by having been partially or fully generated from the original JSON schema.
- primitives: In `MetaManyReceivers`, the fields `name` and `links` will no longer serialize if it is a `None` variant.
- primitives: In `MetaSingleReceiver`, the field `name` will no longer serialize if it is a `None` variant.

### Documentation
- All modules and symbols now have top-level documentation. Some symbols have a somewhat basic description, and will be improved upon in the future.

## v0.1.0 (2024-12-15)

- Initial release of the sdmx_json library

# Changelog

## v0.6.0 (2024-12-23)

### Breaking changes
- primitives: The types `MetaSingleReceiver` and `MetaManyReceivers` are now merged into a single type, `Meta`.
  - When deserializing, a single receiver will become a `Some(Vec<Receiver>)` with 1 item. If multiple receivers are found, it will still become a `Some(Vec<Receiver>)`. If there are no receivers, it will become `None`.
  - When serializing, a single receiver will become a JSON string, and multiple receivers will become a JSON array.
- data: The `DataMessage` type's field `error` is now `errors`, and is now of type `Option<Vec<StatusMessage>>`.

### Features
- structure: The `Artefact` trait now has an `artefact()` method, which returns a reference to `CommonArtefactType`.
- primitives: There is a new trait called `SdmxMessage`. This is implemented by `DataMessage`, `MetadataMessage`, and `StructureMessage`.

### Documentation
- The crate-level documentation now elaborates on how the message types can be deserialized.

### Internal changes and notes
- The `sdmx_json` crate now depends on `serde_with`.
- **Note**: The JSON samples files in the `sdmx_json_de_tests` crate (for deserialization) now closely match their original names from the `sdmx-twg/sdmx-json` repository. The only difference now is that they are prefixed with `twg-`, to indicate where they came from.
  - More JSON files from other sources will be introduced later to ensure the correctness of the `sdmx_json` crate.
- **Note** (data): This fixes a supposed discrepancy within SDMX-JSON Data Message. It is presumed that one of the JSON sample files had a discrepancy according to the following notes:
  - The original file [`1-sdmx-json-field-guide.md`](https://github.com/sdmx-twg/sdmx-json/blob/71fe5eaa9fcd29e3c15f2f0216a19b9b650b1dbd/data-message/docs/1-sdmx-json-field-guide.md) says that the meta object has a `receivers` property, which accepts an optional array of the Receiver  type.
  - The original file [`schemas/2.0.0/sdmx-json-data-schema.json`](https://github.com/sdmx-twg/sdmx-json/blob/71fe5eaa9fcd29e3c15f2f0216a19b9b650b1dbd/data-message/tools/schemas/2.0.0/sdmx-json-data-schema.json) also says the same above as the markdown file.
  - However, the original file `constructed-sample-full.json` file had a meta object where a property is named `receiver`, which  contains an object of Receiver type. This is fixed by making it an array.
  - Note that `generated-sample.json` is correct, as it contains a property named "receivers" which is an array of the Receiver type.
- **Note** (structure): This fixes a supposed discrepancy within SDMX-JSON Structure Message. It is presumed that the original markdown file had a discrepancy according to the following notes:
  - The original file [`1-sdmx-json-field-guide.md`](https://github.com/sdmx-twg/sdmx-json/blob/71fe5eaa9fcd29e3c15f2f0216a19b9b650b1dbd/structure-message/docs/1-sdmx-json-field-guide.md) says that the meta object has a `receiver` property of a Receiver type.
  - The original file [`schemas/2.0.0/sdmx-json-structure-schema.json`](https://github.com/sdmx-twg/sdmx-json/blob/master/structure-message/tools/schemas/2.0.0/sdmx-json-structure-schema.json) however, says that the meta object has a `receivers` property which is an array of the Receiver type.
  - Both original sample JSON files from `sdmx-twg/json` have a `receivers` property which accept an array of Receiver type.

## v0.5.0 (2024-12-23)

### Breaking changes
- structure: The type `AttributeRelationshipDataFlow` was renamed to `AttributeRelationshipDataflow` (lowercased F in dataflow) to be consistent with the `Dataflow` type.

### Documentation
- structure: The following types now have top-level documentation:
  - `AttributeRelationshipDataflow`
  - `AttributeRelationshipDimensions`
  - `AttributeRelationshipGroups`
  - `AttributeRelationshipObservations`
- structure: The type `AttributeRelationshipDataflow` now has a top-level comment for the field `dataflow`.
- structure: The type `AttributeRelationshipObservations` now has a top-level comment for the field `observation`.

## v0.4.0 (2024-12-23)

### Breaking changes
This version includes changes to align with the original JSON schema.
- structure: The enum type `MaxOccurs` was renamed to `Occurrence`.
- structure: The enum type `TimeDimensionDataType` was renamed to `TimeDataType`.
- structure: The following changes have been made to `MetadataAttribute` type:
  - The `local_representation` field is now an optional type (aka `Option`).
  - The inner type of `local_representation` field is now its own type, `MetadataAttributeRepresentation` (instead of `LocalRepresentation`).

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

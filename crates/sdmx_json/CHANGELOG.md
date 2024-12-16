# Changelog

## v0.2.0 (Unreleased)

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
- All modules and symbols now have top-level documentation.

## v0.1.0 (2024-12-15)

- Initial release of the sdmx_json library

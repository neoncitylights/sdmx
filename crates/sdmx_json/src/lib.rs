#![cfg_attr(docsrs, feature(doc_cfg))]

//! A Rust implementation of SDMX-JSON (Statistical Data and Metadata eXchange)
//! using Serde.
//!
//! All JSON files are implemented with a top-level type, e.g
//! [`DataMesssage`][crate::data::DataMessage],
//! [`MetadataMessage`][crate::metadata::MetadataMessage],
//! and [`StructureMessage`][crate::structure::StructureMessage]. Likewise,
//! all of these types can be deserialized from the following types:
//!
//! - a [`&str`] through implementing the [`FromStr`][std::str::FromStr] trait,
//!   which internally calls [`serde_json::from_str()`][serde_json::from_str]
//! - a [`&[u8]`][slice] through implementing `TryFrom<&u8>`,
//!   which internally calls [`serde_json::from_slice()`][serde_json::from_slice]
//! - a [`Value`][serde_json::Value] through implementing `TryFrom<Value>`,
//!   which internally calls [`serde_json::from_value()`][serde_json::from_value]
//! - any type that implements [`Read`](`std::io::Read`) by directly calling
//!   [`serde_json::from_reader()`][serde_json::from_reader]

#[macro_use]
mod macros;

/// SDMX-JSON Data Message format, 2.0.0 (aligned with SDMX 3.0.0)
///
/// This module implements [SDMX-JSON Data Message 2.0.0][data].
/// The Data Message format allows for exchanging contextual statistics
/// data via the JSON file format.
///
/// JSON files in this format are implemented in the top-level root type,
/// [`DataMessage`][crate::data::DataMessage].
/// They can be deserialized from a [`&str`], a [`&[u8]`][slice],
/// and a [`Value`][serde_json::Value].
///
/// [data]: <https://github.com/sdmx-twg/sdmx-json/tree/master/data-message>
pub mod data;

/// SDMX-JSON Metadata Message format, 2.0.0 (aligned with SDMX 3.0.0)
///
/// This module implements [SDMX-JSON Metadata Message 2.0.0][metadata].
/// The Metadata Message format includes information that describes
/// the statistical data itself.
///
/// JSON files in this format are implemented in the top-level root type,
/// [`MetadataMessage`][crate::metadata::MetadataMessage].
/// They can be deserialized from a [`&str`], a [`&[u8]`][slice],
/// and a [`Value`][serde_json::Value].
///
/// [metadata]: <https://github.com/sdmx-twg/sdmx-json/tree/master/metadata-message>
pub mod metadata;

/// Common foundational types shared between the message formats
pub mod primitives;

/// SDMX-JSON Structure Message format, 2.0.0 (aligned with SDMX 3.0.0)
///
/// This module implements [SDMX-JSON Structure Message 2.0.0][structure].
/// The Structure Message format is used for describing objects in
/// RESTful API services to make data more easily discoverable
/// and consumable.
///
/// JSON files in this format are implemented in the top-level root type,
/// [`StructureMessage`][crate::structure::StructureMessage].
/// They can be deserialized from a [`&str`], a [`&[u8]`][slice],
/// and a [`Value`][serde_json::Value].
///
/// [structure]: https://github.com/sdmx-twg/sdmx-json/tree/master/structure-message
pub mod structure;

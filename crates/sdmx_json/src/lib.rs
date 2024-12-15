#![cfg_attr(docsrs, feature(doc_cfg))]

//! A Rust implementation of SDMX-JSON (Statistical Data and Metadata eXchange)
//! using Serde.

#[macro_use]
mod macros;

/// Common foundational types shared between the message formats
pub mod primitives;
/// SDMX-JSON data message format
pub mod data;
/// SDMX-JSON metadata message format
pub mod metadata;
/// SDMX-JSON structure message format
pub mod structure;
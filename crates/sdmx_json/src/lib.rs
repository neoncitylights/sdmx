#![cfg_attr(docsrs, feature(doc_cfg))]

//! A Rust implementation of SDMX-JSON (Statistical Data and Metadata eXchange)
//! using Serde.

#[macro_use]
mod macros;

/// SDMX-JSON data message format
pub mod data;
/// SDMX-JSON metadata message format
pub mod metadata;
/// Common foundational types shared between the message formats
pub mod primitives;
/// SDMX-JSON structure message format
pub mod structure;

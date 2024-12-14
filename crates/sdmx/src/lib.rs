#![cfg_attr(docsrs, feature(doc_cfg))]

//! A Rust implementation of SDMX-JSON (Statistical Data and Metadata eXchange)
//! using Serde.

mod common;
pub use common::*;

pub mod data;
pub mod metadata;
pub mod structure;

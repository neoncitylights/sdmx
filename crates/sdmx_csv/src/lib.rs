use std::collections::HashMap;

/// An object for efficiently storing dynamic column
/// names with indices.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct InternedHeaders<'a>(pub HashMap<usize, &'a str>);

/// A CSV record for an SDMX-CSV Data Message.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DataRecord<'a> {
	pub structure: Structure,
	pub structure_id: StructureId<'a>,
	// only if option labels=name
	pub structure_name: Option<StructureName<'a>>,
	// if column is not present, assume to be Information variant
	pub action: Action,
	// if option key=series|obs|both (or: ky != None)
	pub series_key: Option<&'a str>,
	pub obs_key: Option<&'a str>,
	pub components: HashMap<usize, &'a str>,
	pub other: HashMap<usize, &'a str>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MetadataRecord<'a> {
	pub md_structure: Structure,
	pub md_structure_id: StructureId<'a>,
	// column only exists if labels=name
	pub md_structure_name: Option<StructureName<'a>>,
	pub metadataset_id: &'a str,
	// column only exists if labels=name
	pub metadataset_name: Option<&'a str>,
	// If column is not present, assume to be Information variant
	pub action: Action,
	pub target_types: Vec<&'a str>,
	pub target_ids: Vec<&'a str>,
	// column only exists if labels=name
	pub target_names: Option<&'a str>,
	pub components: HashMap<usize, &'a str>,
	pub others: HashMap<usize, &'a str>,
}

/// The type of structure for the given CSV record.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Structure {
	DataFlow,
	DataStructure,
	DataProvision,
}

/// A unique identifier for an SDMX artefact.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct StructureId<'a> {
	agency: &'a str,
	artefact_id: &'a str,
	version: &'a str,
}

impl<'a> StructureId<'a> {
	pub const fn agency(&self) -> &'a str {
		self.agency
	}

	pub const fn artefact_id(&self) -> &'a str {
		self.artefact_id
	}

	pub const fn version(&self) -> &'a str {
		self.version
	}
}

/// A unique identifier and localized name for an SDMX artefact.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct StructureName<'a> {
	pub id: StructureId<'a>,
	pub localized: &'a str,
}

// TODO: Have some sort of sdmx_core/sdmx_im crate
// for sharing common types between standard implementations.
/// An action which describes how or why the data is being transmitted
/// from the sender's side.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Action {
	Append,
	Replace,
	Delete,
	#[default]
	Information,
}

impl TryFrom<char> for Action {
	type Error = ();
	fn try_from(value: char) -> Result<Self, Self::Error> {
		match value {
			'A' => Ok(Self::Append),
			'R' => Ok(Self::Replace),
			'D' => Ok(Self::Delete),
			'I' => Ok(Self::Information),
			_ => Err(()),
		}
	}
}

/// Options to configure an SDMX-CSV Data Message.
/// These options may also be given from an HTTP Accept header.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct DataOptions {
	pub labels: Labels,
	pub time_format: TimeFormat,
	pub keys: Keys,
}

/// Options to configure an SDMX-CSV Metadata Message.
/// These options may also be given from an HTTP Accept header.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct MetadataOptions {
	pub labels: Labels,
}

/// Configures if a label contains an ID, a localized name,
/// or both (in the format of `<id>: <localized name>`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Labels {
	#[default]
	Id,
	Name,
	Both,
}

/// Configures if a time format should be stored as originally
/// recorded, or as a normalized ISO 8601 format.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TimeFormat {
	#[default]
	Original,
	Normalized,
}

/// Configures whether additional key-related columns
/// may appear (no extra, an observation key, a series key,
/// or both).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Keys {
	#[default]
	None,
	Obs,
	Series,
	Both,
}

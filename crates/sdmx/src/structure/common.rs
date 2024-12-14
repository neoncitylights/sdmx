use crate::{Annotation, Links, LocalizedText};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CommonArtefactType {
	pub id: String,
	pub agency_id: Option<String>,
	pub version: Option<String>,
	pub name: Option<String>,
	pub names: Option<LocalizedText>,
	pub valid_from: Option<String>,
	pub valid_to: Option<String>,
	pub is_external_reference: Option<bool>,
	pub annotations: Option<Vec<Annotation>>,
	pub links: Option<Links>,
}

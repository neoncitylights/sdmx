use crate::structure::Item;
use crate::primitives::{Annotation, Link, LocalizedText};

pub trait Artefact {
	fn id(&self) -> &String;
	fn agency_id(&self) -> Option<&String>;
	fn version(&self) -> Option<&String>;
	fn name(&self) -> Option<&String>;
	fn names(&self) -> Option<&LocalizedText>;
	fn valid_from(&self) -> Option<&String>;
	fn valid_to(&self) -> Option<&String>;
	fn is_external_reference(&self) -> Option<bool>;
	fn annotations(&self) -> Option<&Vec<Annotation>>;
	fn links(&self) -> Option<&Vec<Link>>;
}

pub trait ItemScheme {
	fn is_partial(&self) -> Option<bool>;
	fn items(&self) -> Option<&Vec<Item>>;
}

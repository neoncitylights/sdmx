macro_rules! impl_artefact {
	($($struct:ty),+ $(,)?) => {
		$(
			impl $crate::structure::Artefact for $struct {
				fn id(&self) -> &String {
					&(self.artefact.id)
				}

				fn agency_id(&self) -> Option<&String> {
					self.artefact.agency_id.as_ref()
				}

				fn version(&self) -> Option<&String> {
					self.artefact.version.as_ref()
				}

				fn name(&self) -> Option<&String> {
					self.artefact.name.as_ref()
				}

				fn names(&self) -> Option<&$crate::primitives::LocalizedText> {
					self.artefact.names.as_ref()
				}

				fn valid_from(&self) -> Option<&String> {
					self.artefact.valid_from.as_ref()
				}

				fn valid_to(&self) -> Option<&String> {
					self.artefact.valid_to.as_ref()
				}

				fn is_external_reference(&self) -> Option<bool> {
					self.artefact.is_external_reference
				}

				fn annotations(&self) -> Option<&Vec<$crate::primitives::Annotation>> {
					self.artefact.annotations.as_ref()
				}

				fn links(&self) -> Option<&Vec<$crate::primitives::Link>> {
					self.artefact.links.as_ref()
				}
			}
		)+
	};
}

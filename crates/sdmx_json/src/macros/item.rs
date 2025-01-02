macro_rules! impl_item {
	($($struct: ty),+ $(,)?) => {
		$(
			impl $crate::structure::Item for $struct {
				fn id(&self) -> &String {
					&self.item.id
				}
				fn name(&self) -> Option<&String> {
					self.item.name.as_ref()
				}
				fn names(&self) -> Option<&LocalizedText> {
					self.item.names.as_ref()
				}
				fn description(&self) -> Option<&String> {
					self.item.description.as_ref()
				}
				fn descriptions(&self) -> Option<&LocalizedText> {
					self.item.descriptions.as_ref()
				}
				fn annotations(&self) -> Option<&Vec<Annotation>> {
					self.item.annotations.as_ref()
				}
				fn links(&self) -> Option<&Vec<Link>> {
					self.item.links.as_ref()
				}
			}
		)+
	};
	($(geo:$struct: ty),+ $(,)?) => {
		$(
			impl $crate::structure::Item for $struct {
				fn id(&self) -> &String {
					&self.code.item.id
				}
				fn name(&self) -> Option<&String> {
					self.code.item.name.as_ref()
				}
				fn names(&self) -> Option<&LocalizedText> {
					self.code.item.names.as_ref()
				}
				fn description(&self) -> Option<&String> {
					self.code.item.description.as_ref()
				}
				fn descriptions(&self) -> Option<&LocalizedText> {
					self.code.item.descriptions.as_ref()
				}
				fn annotations(&self) -> Option<&Vec<Annotation>> {
					self.code.item.annotations.as_ref()
				}
				fn links(&self) -> Option<&Vec<Link>> {
					self.code.item.links.as_ref()
				}
			}
		)+
	};
}

macro_rules! impl_item_scheme {
	($(($struct: ty, $field: ident)),+ $(,)?) => {
		$(
			impl $crate::structure::ItemScheme for $struct {
				fn is_partial(&self) -> Option<bool> {
					self.is_partial
				}

				fn items(&self) -> Option<&Vec<$crate::structure::Item>> {
					self.$field.as_ref()
				}
			}
		)+
	};
}

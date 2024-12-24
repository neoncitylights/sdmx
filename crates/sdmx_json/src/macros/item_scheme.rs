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

				fn set_items(&mut self, items: Option<Vec<$crate::structure::Item>>) {
					self.$field = items;
				}

				fn clear_items(&mut self) {
					self.$field = None;
				}

				fn contains_items(&self) -> bool {
					match &self.$field {
						Some(items) => !items.is_empty(),
						None => false,
					}
				}
			}
		)+
	};
}

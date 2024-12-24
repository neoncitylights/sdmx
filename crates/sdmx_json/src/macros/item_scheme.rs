macro_rules! impl_item_scheme {
	($(($struct: ty, $field: ident, $item: ty)),+ $(,)?) => {
		$(
			impl $crate::structure::ItemScheme for $struct {
				type Item = $item;

				fn is_partial(&self) -> Option<bool> {
					self.is_partial
				}

				fn items(&self) -> Option<&Vec<Self::Item>> {
					self.$field.as_ref()
				}

				fn set_items(&mut self, items: Option<Vec<Self::Item>>) {
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

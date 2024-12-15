macro_rules! impl_extendable {
	($($struct:ty),+ $(,)?) => {
		$(
			impl $crate::primitives::Extendable for $struct {
				fn other(&self) -> Option<&HashMap<String, Value>> {
					self.other.as_ref()
				}
			}
		)+
	}
}

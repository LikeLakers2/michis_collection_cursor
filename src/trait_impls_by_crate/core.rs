use crate::{IndexableCollection, IndexableCollectionMut};

impl<T, const N: usize> IndexableCollection for [T; N] {
	type Item = T;

	forward_indexable!(get_item);

	fn len(&self) -> usize {
		N
	}
}

impl<T, const N: usize> IndexableCollectionMut for [T; N] {
	forward_mutable!();
}

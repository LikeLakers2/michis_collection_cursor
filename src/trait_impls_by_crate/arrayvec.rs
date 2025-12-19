use arrayvec::ArrayVec;

use crate::{IndexableCollection, IndexableCollectionMut, IndexableCollectionResizable};

impl<T, const CAP: usize> IndexableCollection for ArrayVec<T, CAP> {
	type Item = T;
	forward_indexable!();
}

impl<T, const CAP: usize> IndexableCollectionMut for ArrayVec<T, CAP> {
	forward_mutable!();
}

impl<T, const CAP: usize> IndexableCollectionResizable for ArrayVec<T, CAP> {
	forward_resizable!(check_len_on_remove = true);
}

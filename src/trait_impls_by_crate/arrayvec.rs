use arrayvec::ArrayVec;

use crate::{IndexableCollection, IndexableCollectionMut};

impl<T, const CAP: usize> IndexableCollection for ArrayVec<T, CAP> {
	type Item = T;
	forward_ref!();
}

impl<T, const CAP: usize> IndexableCollectionMut for ArrayVec<T, CAP> {
	forward_mut!(check_len_on_remove = true);
}

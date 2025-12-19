extern crate alloc;

use alloc::{collections::VecDeque, vec::Vec};

use crate::{IndexableCollection, IndexableCollectionMut, IndexableCollectionResizable};

impl<T> IndexableCollection for Vec<T> {
	type Item = T;
	forward_indexable!();
}

impl<T> IndexableCollectionMut for Vec<T> {
	forward_mutable!();
}

impl<T> IndexableCollectionResizable for Vec<T> {
	forward_resizable!(check_len_on_remove = true);
}

impl<T> IndexableCollection for VecDeque<T> {
	type Item = T;
	forward_indexable!();
}

impl<T> IndexableCollectionMut for VecDeque<T> {
	forward_mutable!();
}

impl<T> IndexableCollectionResizable for VecDeque<T> {
	forward_resizable!(check_len_on_remove = false);
}

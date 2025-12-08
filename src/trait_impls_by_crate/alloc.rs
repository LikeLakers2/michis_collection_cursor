extern crate alloc;

use alloc::{collections::VecDeque, vec::Vec};

use crate::{IndexableCollection, IndexableCollectionMut};

impl<T> IndexableCollection for Vec<T> {
	type Item = T;
	forward_ref!();
}

impl<T> IndexableCollectionMut for Vec<T> {
	forward_mut!(check_len_on_remove = true);
}

impl<T> IndexableCollection for VecDeque<T> {
	type Item = T;
	forward_ref!();
}

impl<T> IndexableCollectionMut for VecDeque<T> {
	forward_mut!(check_len_on_remove = false);
}

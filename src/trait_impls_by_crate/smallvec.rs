use smallvec::{Array, SmallVec};

use crate::{IndexableCollection, IndexableCollectionMut};

impl<A: Array> IndexableCollection for SmallVec<A> {
	type Item = <A as Array>::Item;
	forward_ref!();
}

impl<A: Array> IndexableCollectionMut for SmallVec<A> {
	forward_mut!(check_len_on_remove = true);
}

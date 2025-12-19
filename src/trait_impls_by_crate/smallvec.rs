use smallvec::{Array, SmallVec};

use crate::{IndexableCollection, IndexableCollectionMut, IndexableCollectionResizable};

impl<A: Array> IndexableCollection for SmallVec<A> {
	type Item = <A as Array>::Item;
	forward_indexable!();
}

impl<A: Array> IndexableCollectionMut for SmallVec<A> {
	forward_mutable!();
}

impl<A: Array> IndexableCollectionResizable for SmallVec<A> {
	forward_resizable!(check_len_on_remove = true);
}

#[cfg(feature = "alloc")]
use tinyvec::TinyVec;
use tinyvec::{Array, ArrayVec, SliceVec};

use crate::{IndexableCollection, IndexableCollectionMut, IndexableCollectionResizable};

impl<A: Array> IndexableCollection for ArrayVec<A> {
	type Item = <A as Array>::Item;
	forward_indexable!();
}

impl<A: Array> IndexableCollectionMut for ArrayVec<A> {
	forward_mutable!();
}

impl<A: Array> IndexableCollectionResizable for ArrayVec<A> {
	forward_resizable!(check_len_on_remove = true);
}

impl<'s, T> IndexableCollection for SliceVec<'s, T> {
	type Item = T;
	forward_indexable!();
}

impl<'s, T> IndexableCollectionMut for SliceVec<'s, T> {
	forward_mutable!();
}

impl<'s, T: Default> IndexableCollectionResizable for SliceVec<'s, T> {
	forward_resizable!(check_len_on_remove = true);
}

#[cfg(feature = "alloc")]
impl<A: Array> IndexableCollection for TinyVec<A> {
	type Item = <A as Array>::Item;
	forward_indexable!();
}

#[cfg(feature = "alloc")]
impl<A: Array> IndexableCollectionMut for TinyVec<A> {
	forward_mutable!();
}

#[cfg(feature = "alloc")]
impl<A: Array> IndexableCollectionResizable for TinyVec<A> {
	forward_resizable!(check_len_on_remove = true);
}

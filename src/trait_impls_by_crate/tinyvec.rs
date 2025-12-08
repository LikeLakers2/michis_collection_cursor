#[cfg(feature = "alloc")]
use tinyvec::TinyVec;
use tinyvec::{Array, ArrayVec, SliceVec};

use crate::{IndexableCollection, IndexableCollectionMut};

impl<A: Array> IndexableCollection for ArrayVec<A> {
	type Item = <A as Array>::Item;
	forward_ref!();
}

impl<A: Array> IndexableCollectionMut for ArrayVec<A> {
	forward_mut!(check_len_on_remove = true);
}

impl<'s, T> IndexableCollection for SliceVec<'s, T> {
	type Item = T;
	forward_ref!();
}

impl<'s, T: Default> IndexableCollectionMut for SliceVec<'s, T> {
	forward_mut!(check_len_on_remove = true);
}

#[cfg(feature = "alloc")]
impl<A: Array> IndexableCollection for TinyVec<A> {
	type Item = <A as Array>::Item;
	forward_ref!();
}

#[cfg(feature = "alloc")]
impl<A: Array> IndexableCollectionMut for TinyVec<A> {
	forward_mut!(check_len_on_remove = true);
}

use tinyvec::{Array, ArrayVec, SliceVec};
#[cfg(feature = "alloc")]
use tinyvec::TinyVec;

use crate::{IndexableCollection, IndexableCollectionMut};

impl<A: Array> IndexableCollection for ArrayVec<A> {
	type Item = <A as Array>::Item;

	fn len(&self) -> usize {
		self.len()
	}

	fn get_item(&self, index: usize) -> Option<&Self::Item> {
		self.get(index)
	}
}

impl<A: Array> IndexableCollectionMut for ArrayVec<A> {
	fn get_item_mut(&mut self, index: usize) -> Option<&mut Self::Item> {
		self.get_mut(index)
	}

	fn set_item(&mut self, index: usize, item: Self::Item) {
		self.insert(index, item);
	}

	fn remove_item(&mut self, index: usize) -> Option<Self::Item> {
		(index < self.len()).then(|| self.remove(index))
	}

	fn clear(&mut self) {
		self.clear();
	}
}

impl<'s, T> IndexableCollection for SliceVec<'s, T> {
	type Item = T;

	fn len(&self) -> usize {
		self.len()
	}

	fn get_item(&self, index: usize) -> Option<&Self::Item> {
		self.get(index)
	}
}

impl<'s, T: Default> IndexableCollectionMut for SliceVec<'s, T> {
	fn get_item_mut(&mut self, index: usize) -> Option<&mut Self::Item> {
		self.get_mut(index)
	}

	fn set_item(&mut self, index: usize, item: Self::Item) {
		self.insert(index, item);
	}

	fn remove_item(&mut self, index: usize) -> Option<Self::Item> {
		(index < self.len()).then(|| self.remove(index))
	}

	fn clear(&mut self) {
		self.clear();
	}
}

#[cfg(feature = "alloc")]
impl<A: Array> IndexableCollection for TinyVec<A> {
	type Item = <A as Array>::Item;

	fn len(&self) -> usize {
		self.len()
	}

	fn get_item(&self, index: usize) -> Option<&Self::Item> {
		self.get(index)
	}
}

#[cfg(feature = "alloc")]
impl<A: Array> IndexableCollectionMut for TinyVec<A> {
	fn get_item_mut(&mut self, index: usize) -> Option<&mut Self::Item> {
		self.get_mut(index)
	}

	fn set_item(&mut self, index: usize, item: Self::Item) {
		self.insert(index, item);
	}

	fn remove_item(&mut self, index: usize) -> Option<Self::Item> {
		(index < self.len()).then(|| self.remove(index))
	}

	fn clear(&mut self) {
		self.clear();
	}
}
